use std::collections::HashMap;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use souvenir::{Id, Identifiable};

use crate::{ApiResult, Application, Availability, Schedule};

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub name: Option<String>,
    pub parent: Option<Id>,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub name: Option<String>,
    pub id: Id,
    pub parent: Option<Id>,
}

pub async fn generate(
    State(state): State<Application>,
    Json(body): Json<GenerateRequest>,
) -> ApiResult<GenerateResponse> {
    let mut tx = state.pool.begin().await?;

    let availability = Availability::fetch_current(&mut tx).await?;

    let parent_id = match body.parent {
        Some(id) => Some(id),
        None => Schedule::fetch_current(&mut tx).await.ok().map(|s| s.id),
    };

    let mut schedule = Schedule::new(parent_id, body.name);
    schedule.upsert(&mut tx).await?;

    for (slot, mut subjects) in availability.sorted_by_flexibility(&mut tx).await? {
        let mut weights: HashMap<Id, f64> = HashMap::new();

        for &subject in subjects.iter() {
            weights.insert(
                subject,
                weight(&schedule, &availability, subject, &mut tx).await?,
            );
        }

        subjects.sort_by(|a, b| weights[b].total_cmp(&weights[a]));

        schedule.add(slot, subjects[0], &mut tx).await?;
        schedule.add(slot, subjects[1], &mut tx).await?;
    }

    sqlx::query!("UPDATE parameters SET schedule = $1;", schedule.id as Id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Json(GenerateResponse {
        id: schedule.id,
        parent: schedule.parent,
        name: schedule.name,
    }))
}

async fn weight(
    schedule: &Schedule,
    availability: &Availability,
    subject: impl Identifiable,
    tx: &mut sqlx::PgConnection,
) -> anyhow::Result<f64> {
    let subject_id = subject.id();

    let weeks_since = schedule
        .last_scheduled(subject_id, tx)
        .await?
        .unwrap_or(100) as f64
        - 1.0;
    let flexibility = availability.for_subject(subject_id, tx).await?.len() as f64;
    let total_shifted = schedule.count_total(subject_id, tx).await? as f64;
    let shifts_current = schedule.count(subject_id, tx).await? as f64;

    Ok(weeks_since - flexibility / 20.0 - total_shifted / 5.0 - (2.0 + shifts_current).powi(3))
}
