use std::collections::HashMap;

use crate::{Context, Schedule, Slot, Subject};
use clap::Args;
use souvenir::Id;

#[derive(Debug, Args)]
pub struct ShowCommand {
    pub schedule: Option<String>,
}

pub async fn evaluate(ctx: &mut Context, args: ShowCommand) {
    let mut tx = ctx
        .db
        .begin()
        .await
        .expect("could not begin database transaction");

    let schedule = if let Some(schedule) = args.schedule {
        Schedule::resolve(Id::parse(&schedule).unwrap(), &mut *tx).await
    } else {
        Schedule::fetch_current(&mut *tx).await
    }
    .expect("could not resolve schedule");

    let slots: Vec<_> =
        sqlx::query!(r#"SELECT id AS "id: Id", w2m_id FROM slot ORDER BY w2m_id ASC;"#)
            .fetch_all(&mut *tx)
            .await
            .expect("could not find slots")
            .into_iter()
            .filter_map(|record| record.w2m_id.map(|id| (record.id, id)))
            .map(|record| Slot {
                id: record.0,
                w2m_id: record.1,
            })
            .collect();

    tx.commit().await.unwrap();

    println!("{}", schedule.id);

    for (i, slot) in Iterator::enumerate(slots.into_iter()) {
        let res = schedule
            .get_slot(slot.id, &ctx.db)
            .await
            .expect("could not get slot");

        if i % 5 == 0 {
            println!();
        }

        let a = if res.len() > 0 {
            Subject::find(res[0], &ctx.db).await.unwrap().name
        } else {
            "-".to_owned()
        };

        let b = if res.len() > 1 {
            Subject::find(res[1], &ctx.db).await.unwrap().name
        } else {
            "-".to_owned()
        };

        println!("{}, {}", a, b);
    }

    let subjects = Subject::all_subjects(&ctx.db)
        .await
        .expect("could not load subjects");

    let mut double_counts = 0;
    let mut totals = vec![];

    let mut tx = ctx
        .db
        .begin()
        .await
        .expect("could not begin database transaction");

    for subject in subjects.iter() {
        if schedule.count(subject.id, &mut *tx).await.unwrap_or(0) > 1 {
            double_counts += 1;
        }

        totals.push((
            subject,
            schedule
                .count_total(subject.id, &mut *tx)
                .await
                .unwrap_or(0),
        ));
    }

    tx.commit().await.unwrap();

    println!();
    println!("double shifts: {}", double_counts);

    println!(
        "total shifts mean: {}",
        mean(&totals.iter().map(|x| x.1 as f64).collect::<Vec<f64>>()).unwrap_or(f64::NAN)
    );
    println!(
        "total shifts stddev: {}",
        stddev(&totals.iter().map(|x| x.1 as f64).collect::<Vec<f64>>()).unwrap_or(f64::NAN)
    );
    println!(
        "total shifts stddev (non-zero): {}",
        stddev(
            &totals
                .iter()
                .filter(|x| x.1 != 0)
                .map(|x| x.1 as f64)
                .collect::<Vec<f64>>()
        )
        .unwrap_or(f64::NAN)
    );

    println!();

    let mut order = HashMap::new();

    for (sub, count) in totals {
        order.entry(count).or_insert(vec![]).push(sub.name.clone());
    }

    let mut keys = Vec::from_iter(order.keys());
    keys.sort();

    for &k in keys {
        println!(
            "{}: {}",
            k,
            order[&k]
                .iter()
                .filter(|s| k != 0 || !s.ends_with('*'))
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum: f64 = data.iter().sum();
    let count = data.iter().count();

    if count < 1 {
        return None;
    }

    Some(sum / (count as f64))
}

fn stddev(data: &[f64]) -> Option<f64> {
    let mean = mean(data)?;
    let var: f64 = data.iter().map(|v| *v - mean).map(|v| v * v).sum::<f64>()
        / (data.iter().count() as f64 - 1.0);

    Some(var.sqrt())
}
