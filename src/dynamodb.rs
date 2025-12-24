use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use aws_sdk_dynamodb as aws_dynamodb;
use serde_json::Value;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct DynamoDbAppState {
    pub ddb: aws_dynamodb::Client,
    pub table: String,
    pub cache: Arc<RwLock<HashMap<String, (Value, Instant)>>>,
    pub cache_ttl: Duration,
}

pub async fn initialize_dynamodb(table_name: &str, cache_ttl: Duration) -> DynamoDbAppState {
    let shared_config = aws_config::load_from_env().await;
    let ddb_client = aws_dynamodb::Client::new(&shared_config);

    DynamoDbAppState {
        ddb: ddb_client,
        table: table_name.to_string(),
        cache: Arc::new(RwLock::new(HashMap::new())),
        cache_ttl,
    }
}
