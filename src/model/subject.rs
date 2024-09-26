use std::io::Write;

use serde::{Deserialize, Serialize};

use super::Referential;

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subject {
    id: u64,
    name: String,
}

impl Subject {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Referential for Subject {
    fn ref_name() -> &'static str {
        &"subjects"
    }

    fn serialize(&self, mut writer: impl Write) {
        let data = bitcode::serialize(&self).expect("could not serialize subject");
        writer.write(&data).expect("could not write subject");
    }

    fn deserialize(data: &[u8]) -> Self {
        bitcode::deserialize(&data).expect("could not deserialize subject")
    }
}
