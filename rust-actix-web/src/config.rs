use config::Config as ExternalConfig;
pub use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        ExternalConfig::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
