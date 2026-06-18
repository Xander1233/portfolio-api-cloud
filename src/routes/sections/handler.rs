use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::ApiError;
use crate::models::StoredSection;
use crate::repo::require_section;
use crate::state::AppState;

/// Fetch a strongly-typed section and serve it as JSON.
pub async fn serve_section<T>(
    state: &AppState,
    section_type: &'static str,
) -> Result<HttpResponse, ApiError>
where
    T: DeserializeOwned + Serialize,
{
    let section = require_section::<T>(state, section_type).await?;
    Ok(HttpResponse::Ok().json(section))
}

/// Fetch a strongly-typed section, mutate it (e.g. enrich/sort), and serve it as JSON.
pub async fn serve_section_with<T, F>(
    state: &AppState,
    section_type: &'static str,
    transform: F,
) -> Result<HttpResponse, ApiError>
where
    T: DeserializeOwned + Serialize,
    F: FnOnce(&mut StoredSection<T>),
{
    let mut section = require_section::<T>(state, section_type).await?;
    transform(&mut section);
    Ok(HttpResponse::Ok().json(section))
}
