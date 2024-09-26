use crate::{Availability, Handle, Index, Referential, Slot, Subject};
use clap::{Args, ValueEnum};
use regex::Regex;
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

pub fn evaluate(index: &mut Index, args: FetchCommand) {
    let req = reqwest::blocking::get(args.url).expect("could not get w2m");
    let body = req.text().expect("could not parse body");

    let slot_regex = Regex::new(r"TimeOfSlot\[(\d+)]=(\d+)").unwrap();
    let mut slots = HashMap::new();

    for (_, [slot, id]) in slot_regex.captures_iter(&body).map(|c| c.extract()) {
        slots.insert(slot.parse::<u64>().unwrap(), id.parse::<u64>().unwrap());
    }

    let person_regex =
        Regex::new(r"PeopleNames\[\d+] = '([^']+)';PeopleIDs\[\d+] = (\d+)").unwrap();
    let mut people = HashMap::new();

    for (_, [name, id]) in person_regex.captures_iter(&body).map(|c| c.extract()) {
        let sub = Subject::new(id.parse::<u64>().unwrap(), name);
        people.insert(sub.id(), sub);
    }

    let entry_regex = Regex::new(r"AvailableAtSlot\[(\d+)].push\((\d+)\)").unwrap();
    let mut availability: HashMap<Handle<Subject>, HashSet<u64>> = HashMap::new();

    let mut subjects = HashSet::new();

    for (_, [slot_id, person_id]) in entry_regex.captures_iter(&body).map(|c| c.extract()) {
        if let Some(sub) = people.get(&person_id.parse().unwrap()) {
            availability
                .entry(sub.handle())
                .or_default()
                .insert(slot_id.parse().unwrap());

            subjects.insert(sub);
        }
    }

    for subject in subjects.iter() {
        subject.commit().expect("could not save subject");
    }

    let mut result = Availability::new();
    let mut time_slots = vec![];

    for (subject, availability_slots) in availability {
        for &slot in availability_slots.iter() {
            if slot % 4 != 0
                || !availability_slots.contains(&(slot + 1))
                || !availability_slots.contains(&(slot + 2))
                || !availability_slots.contains(&(slot + 3))
            {
                continue;
            }

            let time_slot = Slot::new(*slots.get(availability_slots.get(&slot).unwrap()).unwrap());

            result.insert(time_slot, subject);
            time_slots.push(time_slot);
        }
    }

    let handle = result.commit().expect("could not save availability");

    index
        .subjects
        .extend(subjects.into_iter().map(|s| s.handle()));

    index.slots.extend(time_slots);
    index.availability = Some(handle);
}
