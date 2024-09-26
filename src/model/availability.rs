use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::{Handle, Referential, Slot, Subject};

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    availability: HashMap<Slot, HashSet<Handle<Subject>>>,
}

impl Availability {
    pub fn new() -> Self {
        Self {
            availability: Default::default(),
        }
    }

    pub fn insert(&mut self, slot: Slot, subject: Handle<Subject>) {
        self.availability
            .entry(slot)
            .or_insert(Default::default())
            .insert(subject);
    }

    pub fn for_slot(&self, slot: Slot) -> Vec<Handle<Subject>> {
        self.availability
            .get(&slot)
            .unwrap_or(&Default::default())
            .iter()
            .map(|handle| *handle)
            .collect()
    }

    pub fn for_subject(&self, subject: Handle<Subject>) -> Vec<Slot> {
        self.availability
            .iter()
            .filter(|(_, d)| d.contains(&subject))
            .map(|(s, _)| *s)
            .collect()
    }

    pub fn sorted_by_flexibility(&self) -> Vec<(Slot, Vec<Handle<Subject>>)> {
        let mut list: Vec<_> = self
            .availability
            .iter()
            .map(|(s, d)| (*s, d.iter().map(|h| *h).collect::<Vec<_>>()))
            .collect();

        list.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        list
    }
}

impl Referential for Availability {
    fn ref_name() -> &'static str {
        &"availabilities"
    }

    fn serialize(&self, mut buf: impl std::io::Write) {
        buf.write(&bitcode::serialize(self).expect("could not serialize availability"))
            .expect("could not write availability");
    }

    fn deserialize(data: &[u8]) -> Self {
        bitcode::deserialize(data).expect("could not deserialize availability")
    }
}
