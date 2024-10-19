use clap::Args;
use souvenir::{Id, Identifiable};
use std::fs;

use crate::{Context, Schedule, Slot, Subject};

#[derive(Debug, Args)]
pub struct ExportCommand {
    schedule: Option<String>,
}

pub async fn evaluate(ctx: &Context, args: ExportCommand) {
    let mut tx = ctx
        .db
        .begin()
        .await
        .expect("could not begin database transaction");

    let schedule = if let Some(schedule) = args.schedule {
        Schedule::resolve(Id::<Schedule>::parse(&schedule).unwrap(), &mut *tx).await
    } else {
        Schedule::fetch_current(&mut *tx).await
    }
    .expect("could not resolve schedule");

    tx.commit().await.expect("could not commit transaction");

    let mut slots = Slot::all_slots(&ctx.db).await.expect("slots");
    slots.sort_by_key(|s| s.w2m_id);

    let mut output = String::new();

    for (i, slot) in Iterator::enumerate(slots.into_iter()) {
        let res = schedule
            .get_slot(slot, &ctx.db)
            .await
            .expect("could not get slot");

        let first = Subject::find(res[0], &ctx.db)
            .await
            .expect("could not find subject");
        let second = Subject::find(res[1], &ctx.db)
            .await
            .expect("could not find subject");

        output += &format!(
            "{}\n{}\n{}\n{}\n",
            first.name, second.name, first.name, second.name
        );

        if i % 5 == 4 {
            output += "\n";
        }
    }

    fs::write(format!("{}.csv", schedule.id()), output).expect("could not write exported csv");
}
