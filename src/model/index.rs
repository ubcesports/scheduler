use std::{collections::HashSet, env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{Availability, Handle, Schedule, Slot, Subject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub version: u64,
    pub subjects: HashSet<Handle<Subject>>,
    pub availability: Option<Handle<Availability>>,
    pub slots: HashSet<Slot>,
    pub head: Option<Handle<Schedule>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            version: 1,
            subjects: HashSet::new(),
            availability: None,
            slots: HashSet::new(),
            head: None,
        }
    }

    pub fn load() -> Self {
        Self::load_from(
            env::current_dir()
                .unwrap_or(Default::default())
                .join("index.sch"),
        )
    }

    pub fn load_from(from: PathBuf) -> Self {
        bitcode::deserialize(&fs::read(from).expect("could not read index"))
            .expect("could not deserialize index")
    }

    pub fn write(&self) {
        self.write_to(
            env::current_dir()
                .unwrap_or(Default::default())
                .join("index.sch"),
        );
    }

    pub fn write_to(&self, to: PathBuf) {
        fs::write(
            to,
            bitcode::serialize(self).expect("could not serialize index"),
        )
        .expect("could not write index");
    }
}
