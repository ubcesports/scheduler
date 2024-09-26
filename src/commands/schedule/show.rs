use std::collections::HashMap;

use clap::Args;

use crate::{Handle, Index};

#[derive(Debug, Args)]
pub struct ShowCommand {
    schedule: Option<String>,
}

pub fn evaluate(index: &mut Index, args: ShowCommand) {
    let schedule = args
        .schedule
        .map(|hash| Handle::parse(&hash))
        .unwrap_or(index.head.expect("no schedule to show"))
        .resolve()
        .expect("could not resolve schedule");

    let mut slots = Vec::from_iter(index.slots.iter().map(|s| *s));
    slots.sort();

    for (i, slot) in Iterator::enumerate(slots.into_iter()) {
        let res = schedule.get_slot(slot);

        if i % 5 == 0 {
            println!();
        }

        println!(
            "{}, {}",
            res[0].resolve().unwrap().name(),
            res[1].resolve().unwrap().name()
        );
    }

    let mut subjects = vec![];

    for &handle in index.subjects.iter() {
        subjects.push((
            handle.resolve().unwrap().name().to_owned(),
            schedule.count_total(handle),
        ));
    }

    println!();
    println!(
        "double shifts: {}",
        index
            .subjects
            .iter()
            .map(|&s| schedule.count(s))
            .filter(|&c| c > 1)
            .count()
    );

    println!(
        "total shifts mean: {}",
        mean(&subjects.iter().map(|x| x.1 as f64).collect::<Vec<f64>>()).unwrap_or(f64::NAN)
    );
    println!(
        "total shifts stddev: {}",
        stddev(&subjects.iter().map(|x| x.1 as f64).collect::<Vec<f64>>()).unwrap_or(f64::NAN)
    );
    println!(
        "total shifts stddev (non-zero): {}",
        stddev(
            &subjects
                .iter()
                .filter(|x| x.1 != 0)
                .map(|x| x.1 as f64)
                .collect::<Vec<f64>>()
        )
        .unwrap_or(f64::NAN)
    );

    println!();

    let mut order = HashMap::new();

    for (sub, count) in subjects {
        order.entry(count).or_insert(vec![]).push(sub);
    }

    let mut keys = Vec::from_iter(order.keys());
    keys.sort();

    for k in keys {
        println!("{}: {}", k, order[k].join(", "));
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
