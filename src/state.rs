use std::time::Duration;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client as DynamoClient;

use crate::cache::SectionCache;

#[derive(Clone)]
pub struct AppState {
    pub ddb: DynamoClient,
    pub table: String,
    pub cache: SectionCache,
}

impl AppState {
    pub fn new(shared: &SdkConfig, table: impl Into<String>, cache_ttl: Duration) -> Self {
        let table = table.into();
        tracing::info!(
            table = %table,
            cache_ttl_secs = cache_ttl.as_secs(),
            "initialized dynamodb app state"
        );
        Self {
            ddb: DynamoClient::new(shared),
            table,
            cache: SectionCache::new(cache_ttl),
        }
    }
}
