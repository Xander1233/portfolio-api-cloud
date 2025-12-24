use std::sync::Arc;
use aws_sdk_dynamodb as aws_dynamodb;
use crate::common::StoredSection;

#[derive(Clone)]
pub struct DynamoDbAppState {
    pub ddb: aws_dynamodb::Client,
    pub table: String,
}

pub async fn initialize_dynamodb(table_name: &str) -> Arc<DynamoDbAppState> {
    let shared_config = aws_config::load_from_env().await;
    let ddb_client = aws_dynamodb::Client::new(&shared_config);

    Arc::new(DynamoDbAppState {
        ddb: ddb_client,
        table: table_name.to_string(),
    })
}
