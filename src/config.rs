use std::sync::LazyLock;

use aws_config::SdkConfig;
use config::{Config as ConfigLoader, ConfigError};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct HttpServer {
    pub host: String,
    pub port: u16,
    #[serde(rename = "cache_ttl")]
    pub cache_ttl_seconds: u64,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logging {
    pub level: String,
    pub file: String,
    pub directory: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Assets {
    pub cdn_base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigStruct {
    pub http_server: HttpServer,
    pub logging: Logging,
    pub assets: Assets,
}

impl ConfigStruct {
    fn load() -> Result<Self, ConfigError> {
        ConfigLoader::builder()
            .add_source(config::File::with_name("config/config.toml").required(true))
            .build()?
            .try_deserialize()
    }
}

pub static CONFIG: LazyLock<ConfigStruct> = LazyLock::new(|| {
    ConfigStruct::load().expect("failed to load configuration from config/config.toml")
});

#[derive(Debug, Clone, Deserialize)]
pub struct AwsSecrets {
    #[serde(rename = "DYNAMODB_TABLE")]
    pub ddb_table: String,
}

#[derive(Debug, Error)]
pub enum SecretsError {
    #[error("failed to fetch secret: {0}")]
    Fetch(String),
    #[error("secret payload is empty")]
    EmptyPayload,
    #[error("invalid secret JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

impl AwsSecrets {
    pub async fn load_from_aws(shared: &SdkConfig) -> Result<Self, SecretsError> {
        let asm = aws_sdk_secretsmanager::Client::new(shared);

        let response = asm
            .get_secret_value()
            .secret_id("prod/portfolio/env")
            .send()
            .await
            .map_err(|err| SecretsError::Fetch(format!("{err:?}")))?;

        let payload = response.secret_string.ok_or(SecretsError::EmptyPayload)?;
        Ok(serde_json::from_str(&payload)?)
    }
}
