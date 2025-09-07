use config::{Environment, File};
use serde::{Deserialize, Serialize};

pub use app::*;
pub use database::*;
pub use http::*;

mod app;
mod database;
mod http;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    #[serde(default = "Default::default")]
    pub http: HttpConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn read() -> anyhow::Result<Self> {
        let cfg = config::Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .add_source(
                File::with_name(&std::env::var("SC_CONFIG").unwrap_or("config.toml".to_owned()))
                    .required(false),
            )
            .add_source(Environment::with_prefix("SC").separator("_"))
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}
