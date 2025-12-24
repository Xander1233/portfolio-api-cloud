use aws_sdk_dynamodb as dynamodb;
use dynamodb::types::AttributeValue;
use crate::common::StoredSection;

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