use std::sync::OnceLock;
use aws_config::SdkConfig;
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
    pub port: u16,
    #[serde(rename = "cache_ttl")]
    pub cache_ttl_seconds: u64,
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

#[derive(Debug, Deserialize, Clone)]
pub struct AwsSecrets {
    #[serde(rename = "DYNAMODB_TABLE")]
    pub ddb_table: String,
}

impl AwsSecrets {
    pub async fn load_from_aws(shared_config: &SdkConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let asm = aws_sdk_secretsmanager::Client::new(shared_config);

        let response = asm.get_secret_value()
            .secret_id("prod/portfolio/env")
            .send()
            .await?;

        if let Some(secret_string) = response.secret_string {
            let secrets: AwsSecrets = match serde_json::from_str(&secret_string) {
                Ok(secrets) => secrets,
                Err(err) => {
                    return Err(Box::new(err));
                }
            };
            Ok(secrets)
        } else {
            Err("Secret string is empty".into())
        }
    }
}
