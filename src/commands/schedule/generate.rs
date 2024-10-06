use crate::commands::schedule::show::ShowCommand;
use crate::{Availability, Context, Schedule, Subject};
use clap::Args;
use souvenir::Id;
use sqlx::SqliteConnection;
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct GenerateCommand;

pub async fn evaluate(ctx: &mut Context, _args: GenerateCommand) {
    let mut tx = ctx
        .db
        .begin()
        .await
        .expect("could not begin database transaction");

    let availability = Availability::fetch_current(&mut *tx)
        .await
        .expect("could not fetch availability");

    let parent = Schedule::fetch_current(&mut *tx).await.ok();
    let mut schedule = Schedule::new(parent.map(|schedule| schedule.id()));

    schedule
        .upsert(&mut *tx)
        .await
        .expect("could not save schedule");

    for (slot, mut subjects) in availability
        .sorted_by_flexibility(&mut *tx)
        .await
        .expect("could not get availability entries")
    {
        let mut weights = HashMap::new();

        for &subject in subjects.iter() {
            weights.insert(
                subject,
                weight(&schedule, &availability, subject, &mut *tx)
                    .await
                    .expect("could not calculate weights"),
            );
        }

        subjects.sort_by(|a, b| weights[b].total_cmp(&weights[a]));

        schedule
            .add(slot, subjects[0], &mut *tx)
            .await
            .expect("could not add schedule entry");

        schedule
            .add(slot, subjects[1], &mut *tx)
            .await
            .expect("could not add schedule entry");
    }

    let schedule_id = schedule.id().as_i64();

    sqlx::query!("UPDATE parameters SET schedule = $1;", schedule_id)
        .execute(&mut *tx)
        .await
        .expect("could not update schedule");

    tx.commit().await.expect("could not save schedule");

    crate::commands::schedule::show::evaluate(ctx, ShowCommand { schedule: None }).await;
}

async fn weight(
    schedule: &Schedule,
    availability: &Availability,
    subject: Id<Subject>,
    tx: &mut SqliteConnection,
) -> Result<f64, sqlx::Error> {
    let weeks_since = schedule.last_scheduled(subject, tx).await?.unwrap_or(100) as f64 - 1.0;
    let flexibility = availability.for_subject(subject, tx).await?.len() as f64;
    let total_shifted = schedule.count_total(subject, tx).await? as f64;
    let shifts_current = schedule.count(subject, tx).await? as f64;

    Ok(1.0 + 2.0 * weeks_since
        - flexibility / 20.0
        - total_shifted * 5.0
        - (1.0 + shifts_current).powi(2))
}
