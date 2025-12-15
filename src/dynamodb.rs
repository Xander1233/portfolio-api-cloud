use aws_sdk_dynamodb as dynamodb;
use dynamodb::types::AttributeValue;
use crate::common::StoredSection;

#[derive(Clone)]
pub struct DynamoDbAppState {
    pub ddb: dynamodb::Client,
    pub table: String,
}

pub async fn initialize_dynamodb(table_name: &str) -> DynamoDbAppState {
    let shared_config = aws_config::load_from_env().await;
    let ddb_client = dynamodb::Client::new(&shared_config);

    DynamoDbAppState {
        ddb: ddb_client,
        table: table_name.to_string(),
    }
}

pub async fn get_section<T: for<'de> serde::Deserialize<'de>>(
    ddb: &dynamodb::Client,
    table: &str,
    id: i64,
    section_type: &str,
) -> anyhow::Result<Option<StoredSection<T>>> {
    let response = ddb
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::N(id.to_string()))
        .key("type", AttributeValue::S(section_type.to_string()))
        .send()
        .await?;

    if let Some(item) = response.item {
        let stored_section: StoredSection<T> = serde_dynamo::from_item(item)?;
        Ok(Some(stored_section))
    } else {
        Ok(None)
    }
}

pub async fn put_section<T: serde::Serialize>(
    ddb: &dynamodb::Client,
    table: &str,
    id: i64,
    section_type: &str,
    data: T,
    updated_at: i64,
) -> anyhow::Result<()> {
    let section_to_store = StoredSection {
        id,
        section_type: section_type.to_string(),
        updated_at,
        data,
    };

    let item = serde_dynamo::to_item(section_to_store)?;

    ddb.put_item()
        .table_name(table)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(())
}