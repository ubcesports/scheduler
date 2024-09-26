use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{Handle, Referential, Subject};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Slot(u64);

impl Slot {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schedule {
    parent: Option<Handle<Schedule>>,
    schedule: HashMap<Slot, HashSet<Handle<Subject>>>,
}

impl Schedule {
    pub fn new(parent: Option<Handle<Schedule>>) -> Self {
        Self {
            parent,
            schedule: Default::default(),
        }
    }

    pub fn parent(&self) -> Option<Handle<Schedule>> {
        self.parent
    }

    pub fn count(&self, subject: Handle<Subject>) -> u64 {
        self.schedule
            .iter()
            .filter(|(_, v)| v.contains(&subject))
            .count()
            .try_into()
            .unwrap()
    }

    pub fn count_total(&self, subject: Handle<Subject>) -> u64 {
        let parent_count = if let Some(parent) = self.parent {
            parent
                .resolve()
                .expect("could not resolve schedule parent")
                .count_total(subject)
        } else {
            0
        };

        parent_count + self.count(subject)
    }

    pub fn last_scheduled(&self, subject: Handle<Subject>) -> Option<u32> {
        if self
            .schedule
            .iter()
            .find(|(_, v)| v.contains(&subject))
            .is_some()
        {
            return Some(0);
        }

        Some(self.parent?.resolve().ok()?.last_scheduled(subject)? + 1)
    }

    pub fn add(&mut self, slot: Slot, subject: Handle<Subject>) {
        self.schedule
            .entry(slot)
            .or_insert(Default::default())
            .insert(subject);
    }

    pub fn get_slot(&self, slot: Slot) -> Vec<Handle<Subject>> {
        self.schedule
            .get(&slot)
            .unwrap_or(&Default::default())
            .into_iter()
            .map(|r| *r)
            .collect()
    }

    pub fn rewrite(mut self) -> Schedule {
        self.delete()
            .inspect_err(|_| eprintln!("warning: could not delete schedule"))
            .ok();

        self.parent = self.parent.map(|parent| {
            parent
                .resolve()
                .expect("could not resolve parent schedule")
                .rewrite()
                .handle()
        });

        self.commit().expect("could not rewrite schedule");
        self
    }
}

impl Referential for Schedule {
    fn ref_name() -> &'static str {
        &"schedules"
    }

    fn serialize(&self, mut buf: impl std::io::Write) {
        buf.write(&bitcode::serialize(self).expect("could not serialize schedule"))
            .expect("could not write schedule");
    }

    fn deserialize(data: &[u8]) -> Self {
        bitcode::deserialize(data)
            .or_else(|_| serde_json::from_slice(data))
            .expect("could not deserialize schedule")
    }
}
