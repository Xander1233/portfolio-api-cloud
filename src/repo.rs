use std::time::Instant;

use aws_sdk_dynamodb::types::AttributeValue;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::ApiError;
use crate::models::StoredSection;
use crate::state::AppState;

const DEFAULT_PROFILE_ID: i64 = 1;

pub fn profile_id() -> i64 {
    DEFAULT_PROFILE_ID
}

fn cache_key(id: i64, section_type: &str) -> String {
    format!("{id}:{section_type}")
}

#[tracing::instrument(skip(state), fields(section = section_type, id))]
pub async fn get_section<T>(
    state: &AppState,
    id: i64,
    section_type: &'static str,
) -> Result<Option<StoredSection<T>>, ApiError>
where
    T: DeserializeOwned + Serialize,
{
    let key = cache_key(id, section_type);

    if let Some(cached) = state.cache.get(&key).await {
        tracing::debug!("cache hit");
        return Ok(Some(serde_json::from_value(cached)?));
    }

    tracing::debug!("cache miss; fetching from dynamodb");

    let started = Instant::now();
    let response = state
        .ddb
        .get_item()
        .table_name(&state.table)
        .key("id", AttributeValue::N(id.to_string()))
        .key("type", AttributeValue::S(section_type.to_string()))
        .send()
        .await?;
    let ddb_ms = started.elapsed().as_millis() as u64;

    let Some(item) = response.item else {
        tracing::warn!(ddb_ms, "section not found in dynamodb");
        return Ok(None);
    };

    let section: StoredSection<T> = serde_dynamo::from_item(item)?;
    state
        .cache
        .insert(key, serde_json::to_value(&section)?)
        .await;

    tracing::info!(
        ddb_ms,
        updated_at = section.updated_at,
        "section fetched from dynamodb and cached"
    );

    Ok(Some(section))
}

#[tracing::instrument(skip(state), fields(section = section_type))]
pub async fn require_section<T>(
    state: &AppState,
    section_type: &'static str,
) -> Result<StoredSection<T>, ApiError>
where
    T: DeserializeOwned + Serialize,
{
    get_section::<T>(state, profile_id(), section_type)
        .await?
        .ok_or(ApiError::SectionNotFound(section_type))
}

#[tracing::instrument(skip(state), fields(id))]
pub async fn query_all_sections(
    state: &AppState,
    id: i64,
) -> Result<Vec<StoredSection<serde_json::Value>>, ApiError> {
    let started = Instant::now();
    let response = state
        .ddb
        .query()
        .table_name(&state.table)
        .key_condition_expression("id = :id")
        .expression_attribute_values(":id", AttributeValue::N(id.to_string()))
        .send()
        .await?;
    let ddb_ms = started.elapsed().as_millis() as u64;

    let raw_items = response.items.unwrap_or_default();
    let count = raw_items.len();

    let mut sections = Vec::with_capacity(count);
    for item in raw_items {
        sections.push(serde_dynamo::from_item(item)?);
    }

    tracing::info!(ddb_ms, count, "fetched all sections for profile");

    Ok(sections)
}
