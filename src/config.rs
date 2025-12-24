use std::sync::OnceLock;
use config::{ConfigError, Config as ConfigLoader};
use serde::Deserialize;
use envconfig::{Envconfig, Error as EnvConfigError};
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub hostname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpServer {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
pub struct Logging {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigStruct {
    pub general: General,
    pub http_server: HttpServer,
    pub logging: Logging,
}

impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = ConfigLoader::builder()
            .add_source(config::File::with_name("config/config.toml").required(true))
            .build();

        if let Ok(config) = conf {
            config.try_deserialize()
        } else {
            Err(ConfigError::Message("Failed to load configuration".to_string()))
        }
    }
}

pub static Config: Lazy<ConfigStruct> = Lazy::new(|| {
    let conf = ConfigLoader::builder()
        .add_source(config::File::with_name("config/config.toml").required(true))
        .build();

    if let Ok(config) = conf {
        if let Ok(config) = config.try_deserialize::<ConfigStruct>() {
            config
        } else {
            panic!("Failed to deserialize configuration");
        }
    } else {
        panic!("Failed to load configuration");
    }
});
