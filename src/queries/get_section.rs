use std::collections::HashMap;
use std::sync::{Arc};
use std::time::{Duration, Instant};
use aws_sdk_dynamodb as dynamodb;
use dynamodb::types::AttributeValue;
use serde_json::Value;
use tokio::sync::RwLock;
use crate::common::StoredSection;

pub async fn get_section<T: for<'de> serde::Deserialize<'de> + serde::Serialize>(
    ddb: &dynamodb::Client,
    table: &str,
    id: i64,
    section_type: &str,
    cache: Arc<RwLock<HashMap<String, (Value, Instant)>>>,
    cache_ttl: Duration
) -> anyhow::Result<Option<StoredSection<T>>> {

    tracing::info!("Retrieving section {} for id {}", section_type, id);

    let cache_key = format!("{id}:{section_type}");

    let cached_opt = {
        let cache_read = cache.read().await;
        cache_read.get(&cache_key).cloned()
    };

    if let Some((cached_value, ttl)) = cached_opt {
        if ttl.elapsed() < cache_ttl {
            tracing::info!("Cache hit for section {} of id {}", section_type, id);
            let stored_section: StoredSection<T> = serde_json::from_value(cached_value)?;
            return Ok(Some(stored_section));
        } else {
            tracing::info!("Cache expired for section {} of id {}", section_type, id);
        }
    }

    let response = ddb
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::N(id.to_string()))
        .key("type", AttributeValue::S(section_type.to_string()))
        .send()
        .await?;

    if let Some(item) = response.item {
        let stored_section: StoredSection<T> = serde_dynamo::from_item(item)?;

        let mut cache_write = cache.write().await;
        let value = serde_json::to_value(&stored_section)?;

        cache_write.insert(
            cache_key,
            (value, Instant::now()),
        );
        tracing::info!("Cache updated for section {} of id {}", section_type, id);

        Ok(Some(stored_section))
    } else {
        Ok(None)
    }
}