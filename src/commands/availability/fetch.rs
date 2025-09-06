use crate::{Availability, Context, Slot, Subject};
use clap::{Args, ValueEnum};
use regex::Regex;
use souvenir::{id, Id};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Args)]
pub struct FetchCommand {
    #[arg(long)]
    url: String,

    #[arg(short, long, name = "type", value_enum)]
    parse_type: PullType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum PullType {
    #[clap(name = "w2m")]
    When2Meet,
}

pub async fn evaluate(ctx: &Context, args: FetchCommand) {
    // fetch w2m page
    let body = reqwest::get(args.url)
        .await
        .expect("could not get w2m")
        .text()
        .await
        .expect("could not parse body");

    // begin database transaction
    let mut tx = ctx
        .db
        .begin()
        .await
        .expect("could not begin database transaction");

    // create a new availability object
    let mut availability = Availability::new(id!(Availability));
    availability
        .upsert(&mut *tx)
        .await
        .expect("could not add availability");

    // get all time slots, add to database, and map slot number to slot
    let slot_regex = Regex::new(r"TimeOfSlot\[(\d+)]=(\d+)").unwrap();
    let mut slots = HashMap::new();

    for (_, [slot_counter, slot_time]) in slot_regex.captures_iter(&body).map(|c| c.extract()) {
        slots.insert(
            slot_counter.parse::<i64>().unwrap(),
            slot_time.parse::<i64>().unwrap(),
        );
    }

    // get all subjects, add to database, and create a map of their w2m id to subject
    let person_regex =
        Regex::new(r"PeopleNames\[\d+] = '([^']+)';PeopleIDs\[\d+] = (\d+)").unwrap();
    let mut people = HashMap::new();

    for (_, [name, id]) in person_regex.captures_iter(&body).map(|c| c.extract()) {
        let subject = Subject::upsert(id!(Subject), id.parse::<i32>().ok(), name, &mut *tx)
            .await
            .expect("could not add subject");

        people.insert(subject.w2m_id.unwrap(), subject);
    }

    // add availability entries
    let entry_regex = Regex::new(r"AvailableAtSlot\[(\d+)].push\((\d+)\)").unwrap();
    let mut full_availability: HashMap<&Subject, HashSet<i64>> = HashMap::new();

    let mut subjects = HashSet::new();

    for (_, [slot_id, person_id]) in entry_regex.captures_iter(&body).map(|c| c.extract()) {
        if let Some(subject) = people.get(&person_id.parse().unwrap()) {
            full_availability
                .entry(subject)
                .or_default()
                .insert(slot_id.parse().unwrap());

            subjects.insert(subject);
        }
    }

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
                .await
                .expect("could not add slot");

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
            .await
            .expect("could not add availability entry");
        }
    }

    sqlx::query!(
        "UPDATE parameters SET availability = $1;",
        availability.id as Id
    )
    .execute(&mut *tx)
    .await
    .expect("could not update availability");

    tx.commit()
        .await
        .expect("could not commit database transaction");
}
