use std::collections::HashMap;
use std::path::PathBuf;

use clap::Args;
use souvenir::{id, Id, Tagged};
use sqlx::{query, query_as, PgPool};

use crate::{Availability, Context, Schedule, Slot, Subject};

#[derive(Debug, Args)]
#[command(about = "Initialize a new project")]
pub struct MigrateCommand {
    source: PathBuf,
}

#[derive(sqlx::FromRow)]
struct OldSchedule {
    id: i64,
    parent_id: Option<i64>,
}

#[derive(sqlx::FromRow)]
struct OldScheduleAssignment {
    schedule_id: i64,
    slot_id: i64,
    subject_id: i64,
}

#[derive(sqlx::FromRow)]
struct OldSubject {
    id: i64,
    w2m_id: Option<i32>,
    name: String,
}

#[derive(sqlx::FromRow)]
struct OldSlot {
    id: i64,
    w2m_id: i32,
}

#[derive(sqlx::FromRow)]
struct OldAvailability {
    id: i64,
}

#[derive(sqlx::FromRow)]
struct OldAvailabilityEntry {
    availability_id: i64,
    slot_id: i64,
    subject_id: i64,
}

pub async fn evaluate(ctx: &Context, args: MigrateCommand) {
    let source = PgPool::connect(args.source.to_str().unwrap())
        .await
        .expect("could not connect to source database");

    let mut tx = ctx.db.begin().await.expect("could not begin transaction");

    let mut schedule_map: HashMap<i64, Id> = HashMap::new();
    let mut slot_map: HashMap<i64, Id> = HashMap::new();
    let mut subject_map: HashMap<i64, Id> = HashMap::new();
    let mut availability_map: HashMap<i64, Id> = HashMap::new();

    fn get_id<T: Tagged>(value: i64, map: &mut HashMap<i64, Id>) -> Id {
        *map.entry(value).or_insert(id!(T))
    }

    let schedules = query_as::<_, OldSchedule>("SELECT * from schedule;")
        .fetch_all(&source)
        .await
        .expect("could not fetch schedules");

    for schedule in schedules {
        let id = get_id::<Schedule>(schedule.id, &mut schedule_map);

        let parent = schedule
            .parent_id
            .map(|value| get_id::<Schedule>(value, &mut schedule_map));

        query!(
            "INSERT INTO schedule (id, parent_id) VALUES ($1, $2);",
            id as Id,
            parent as Option<Id>,
        )
        .execute(&mut *tx)
        .await
        .expect("could not insert into schedule");
    }

    let schedule_assginments =
        query_as::<_, OldScheduleAssignment>("SELECT * from schedule_assignment;")
            .fetch_all(&source)
            .await
            .expect("could not fetch schedule assignments");

    for assignment in schedule_assginments {
        let schedule_id = get_id::<Schedule>(assignment.schedule_id, &mut schedule_map);
        let slot_id = get_id::<Slot>(assignment.slot_id, &mut slot_map);
        let subject_id = get_id::<Subject>(assignment.subject_id, &mut subject_map);

        query!(
            "INSERT INTO schedule_assignment (schedule_id, slot_id, subject_id) VALUES ($1, $2, $3);",
            schedule_id as Id,
            slot_id as Id,
            subject_id as Id,
        )
        .execute(&mut *tx)
        .await
        .expect("could not insert into schedule");
    }

    let subjects = query_as::<_, OldSubject>("SELECT * from subject;")
        .fetch_all(&source)
        .await
        .expect("could not could not fetch subjects");

    for subject in subjects {
        let id = get_id::<Subject>(subject.id, &mut subject_map);

        query!(
            "INSERT INTO subject (id, w2m_id, name) VALUES ($1, $2, $3);",
            id as Id,
            subject.w2m_id,
            subject.name,
        )
        .execute(&mut *tx)
        .await
        .expect("could not insert into schedule");
    }

    let slots = query_as::<_, OldSlot>("SELECT * from slot;")
        .fetch_all(&source)
        .await
        .expect("could not fetch slots");

    for slot in slots {
        let id = get_id::<Slot>(slot.id, &mut slot_map);

        query!(
            "INSERT INTO slot (id, w2m_id) VALUES ($1, $2);",
            id as Id,
            slot.w2m_id,
        )
        .execute(&mut *tx)
        .await
        .expect("could not insert into schedule");
    }

    let availabilities = query_as::<_, OldAvailability>("SELECT * from availability;")
        .fetch_all(&source)
        .await
        .expect("could not fetch availabilities");

    for availability in availabilities {
        let id = get_id::<Availability>(availability.id, &mut availability_map);

        query!("INSERT INTO availability (id) VALUES ($1);", id as Id)
            .execute(&mut *tx)
            .await
            .expect("could not insert into schedule");
    }

    let availability_entries =
        query_as::<_, OldAvailabilityEntry>("SELECT * from availability_entry;")
            .fetch_all(&source)
            .await
            .expect("could not fetch availability entries");

    for entry in availability_entries {
        let availability_id = get_id::<Availability>(entry.availability_id, &mut availability_map);
        let slot_id = get_id::<Slot>(entry.slot_id, &mut slot_map);
        let subject_id = get_id::<Subject>(entry.subject_id, &mut subject_map);

        query!(
            "
                INSERT INTO availability_entry (availability_id, slot_id, subject_id) 
                    VALUES ($1, $2, $3);
            ",
            availability_id as Id,
            slot_id as Id,
            subject_id as Id
        )
        .execute(&mut *tx)
        .await
        .expect("could not insert into schedule");
    }

    let (availability, schedule): (i64, i64) =
        query_as("SELECT availability, schedule from parameters;")
            .fetch_one(&source)
            .await
            .expect("could not fetch parameters");

    let availability_id = get_id::<Availability>(availability, &mut availability_map);
    let schedule_id = get_id::<Schedule>(schedule, &mut schedule_map);

    query!(
        "UPDATE parameters SET availability = $1, schedule = $2",
        availability_id as Id,
        schedule_id as Id
    )
    .execute(&mut *tx)
    .await
    .expect("could not insert into schedule");

    tx.commit().await.expect("could not commit transaction");
}
