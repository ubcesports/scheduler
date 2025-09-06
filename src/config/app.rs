use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub mode: AppEnv,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum AppEnv {
    #[serde(rename = "development")]
    Development,

    #[serde(rename = "staging")]
    Staging,

    #[serde(rename = "production")]
    Production,
}
