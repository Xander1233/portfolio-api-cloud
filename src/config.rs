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
            Err(ConfigError::Message("Failed to load configuration".into()))
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

#[derive(Envconfig, Clone, Debug)]
pub struct EnvSettingsStruct {
    #[envconfig(nested = true)]
    pub database: Database,
    #[envconfig(nested = true)]
    pub entra_id: EntraId,
}

#[derive(Envconfig, Clone, Debug)]
pub struct Database {
    #[envconfig(from = "DATABASE_URL")]
    pub url: String,
}

#[derive(Envconfig, Clone, Debug)]
pub struct EntraId {
    #[envconfig(from = "ENTRA_ID_CLIENT_ID")]
    pub client_id: String,
    #[envconfig(from = "ENTRA_ID_TENANT_ID")]
    pub tenant_id: String,
    #[envconfig(from = "ENTRA_ID_CLIENT_SECRET")]
    pub client_secret: String,
    #[envconfig(from = "ENTRA_ID_JWT_SECRET")]
    pub jwt_secret: String,
    #[envconfig(from = "ENTRA_ID_REDIRECT_URI")]
    pub redirect_uri: String,
}

pub static EnvSettings: Lazy<EnvSettingsStruct> = Lazy::new(|| {
    EnvSettingsStruct::init_from_env().expect("Failed to initialize environment settings")
});