use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub mode: AppEnv,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppEnv {
    #[serde(rename = "development")]
    Development,

    #[serde(rename = "staging")]
    Staging,

    #[serde(rename = "production")]
    Production,
}
