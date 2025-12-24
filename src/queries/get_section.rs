use aws_sdk_dynamodb as dynamodb;
use dynamodb::types::AttributeValue;
use crate::common::StoredSection;

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