use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    pub listen: ListenConfig,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListenConfig {
    pub address: String,
    pub port: u16,
}

impl Default for ListenConfig {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_owned(),
            port: 5678,
        }
    }
}
