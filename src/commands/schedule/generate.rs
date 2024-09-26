use clap::Args;

use crate::{Availability, Handle, Index, Referential, Schedule, Subject};

#[derive(Debug, Args)]
pub struct GenerateCommand;

pub fn evaluate(index: &mut Index, _args: GenerateCommand) {
    let availability = index
        .availability
        .expect("no availability found")
        .resolve()
        .expect("could not resolve availability");

    let mut schedule = Schedule::new(index.head);

    for (slot, mut subjects) in availability.sorted_by_flexibility() {
        subjects.sort_by(|&a, &b| {
            weight(&schedule, &availability, b).total_cmp(&weight(&schedule, &availability, a))
        });

        schedule.add(slot, subjects[0]);
        schedule.add(slot, subjects[1]);
    }

    let handle = schedule.commit().expect("could not save schedule");

    let mut slots: Vec<_> = index.slots.iter().collect();
    slots.sort();

    for &slot in slots {
        let subjects = schedule.get_slot(slot);

        println!(
            "{}, {}",
            subjects[0].resolve().unwrap().name(),
            subjects[1].resolve().unwrap().name()
        );
    }

    index.head = Some(handle);
}

fn weight(schedule: &Schedule, availability: &Availability, subject: Handle<Subject>) -> f64 {
    let weeks_since = schedule.last_scheduled(subject).unwrap_or(100) as f64 - 1.0;
    let flexibility = availability.for_subject(subject).len() as f64;
    let total_shifted = schedule.count_total(subject) as f64;
    let shifts_current = schedule.count(subject) as f64;

    1.0 + 5.0 * weeks_since
        - flexibility / 20.0
        - total_shifted / 5.0
        - (1.0 + shifts_current * 2.0).powi(2)
}
