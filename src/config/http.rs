use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    pub listen: ListenConfig,
}

#[derive(Clone, Deserialize)]
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
