use axum::{extract::State, Json};
use regex::Regex;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use souvenir::{id, Id};
use std::collections::{HashMap, HashSet};

use crate::{
    model::{Availability, Slot, Subject},
    ApiError, ApiResult, Application,
};

#[derive(Deserialize, PartialEq, Eq)]
pub enum ParseType {
    #[serde(rename = "w2m")]
    When2Meet,
}

#[derive(Deserialize)]
pub struct ImportRequest {
    pub format: ParseType,
    pub source: String,
}

#[derive(Serialize)]
pub struct ImportResponse {
    pub id: Id,
    pub entries: i32,

    pub subjects_imported: i32,
    pub slots_imported: i32,
}

pub async fn import(
    State(app): State<Application>,
    Json(body): Json<ImportRequest>,
) -> ApiResult<ImportResponse> {
    if body.format != ParseType::When2Meet {
        return Err(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            error: anyhow::anyhow!("Only 'w2m' parse type is supported"),
        });
    }

    let page = reqwest::get(&body.source).await?.text().await?;
    let mut tx = app.pool.begin().await?;

    let mut availability = Availability::new(id!(Availability));
    availability.upsert(&mut *tx).await?;

    // Get all time slots, add to database, and map slot number to slot
    let slot_regex = Regex::new(r"TimeOfSlot\[(\d+)]=(\d+)").unwrap();
    let mut slots = HashMap::new();

    for (_, [slot_counter, slot_time]) in slot_regex.captures_iter(&page).map(|c| c.extract()) {
        slots.insert(
            slot_counter.parse::<i32>().unwrap(),
            slot_time.parse::<i32>().unwrap(),
        );
    }

    // Get all subjects, add to database, and create a map of their w2m id to subject
    let person_regex =
        Regex::new(r"PeopleNames\[\d+] = '([^']+)';PeopleIDs\[\d+] = (\d+)").unwrap();
    let mut people = HashMap::new();

    for (_, [name, id]) in person_regex.captures_iter(&page).map(|c| c.extract()) {
        let subject = Subject::upsert(id!(Subject), id.parse::<i32>().ok(), name, &mut *tx).await?;
        people.insert(subject.w2m_id.unwrap(), subject);
    }

    // Add availability entries
    let entry_regex = Regex::new(r"AvailableAtSlot\[(\d+)].push\((\d+)\)").unwrap();
    let mut full_availability: HashMap<&Subject, HashSet<i32>> = HashMap::new();

    let mut subjects = HashSet::new();

    for (_, [slot_id, person_id]) in entry_regex.captures_iter(&page).map(|c| c.extract()) {
        if let Some(subject) = people.get(&person_id.parse().unwrap()) {
            full_availability
                .entry(subject)
                .or_default()
                .insert(slot_id.parse().unwrap());

            subjects.insert(subject);
        }
    }

    let mut slots_imported = 0;
    let mut entries_created = 0;

    for (subject, available_slots) in full_availability {
        for &slot in available_slots.iter() {
            if slot % 4 != 0
                || !available_slots.contains(&(slot + 1))
                || !available_slots.contains(&(slot + 2))
                || !available_slots.contains(&(slot + 3))
            {
                continue;
            }

            let slot = {
                let data = Slot::new(id!(Slot), *slots.get(&slot).unwrap() as i32);
                let result = sqlx::query!(
                    r#"
                        INSERT INTO slot (id, w2m_id)
                            VALUES ($1, $2)
                            ON CONFLICT (w2m_id) DO UPDATE SET w2m_id = $2
                            RETURNING id AS "id: Id", w2m_id;
                    "#,
                    data.id as Id,
                    data.w2m_id,
                )
                .fetch_one(&mut *tx)
                .await?;

                slots_imported += 1;
                Slot::new(result.id, result.w2m_id.unwrap())
            };

            sqlx::query!(
                "
                    INSERT INTO availability_entry (availability_id, slot_id, subject_id) 
                        VALUES ($1, $2, $3);
                ",
                availability.id as Id,
                slot.id as Id,
                subject.id as Id,
            )
            .execute(&mut *tx)
            .await?;

            entries_created += 1;
        }
    }

    // Update parameters with new availability
    sqlx::query!(
        "UPDATE parameters SET availability = $1;",
        availability.id as Id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ImportResponse {
        id: availability.id,
        subjects_imported: people.len() as i32,
        slots_imported,
        entries: entries_created,
    }))
}
