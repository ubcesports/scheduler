use crate::Context;
use clap::Args;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Args)]
pub struct ShowCommand;

pub async fn evaluate(ctx: &Context, _args: ShowCommand) {
    let query = sqlx::query!(
        "
            SELECT slot.w2m_id, subject.name FROM availability_entry
                LEFT JOIN slot ON slot_id = slot.id
                LEFT JOIN subject ON subject_id = subject.id
                WHERE availability_id = (SELECT availability FROM parameters);
        "
    )
    .fetch_all(&ctx.db)
    .await
    .expect("could not resolve availability");

    let mut slots: Vec<i32> = HashSet::<i32>::from_iter(query.iter().filter_map(|r| r.w2m_id))
        .into_iter()
        .collect();

    slots.sort();

    let mut map: HashMap<i32, Vec<String>> = HashMap::new();

    query
        .into_iter()
        .filter_map(|r| r.w2m_id.and_then(|a| r.name.map(|b| (a, b))))
        .for_each(|(id, name)| map.entry(id).or_default().push(name));

    for (i, slot) in Iterator::enumerate(slots.iter()) {
        if i % 5 == 0 {
            println!();
        }

        print!("{}\t", map[slot].len());
    }

    println!();
}
