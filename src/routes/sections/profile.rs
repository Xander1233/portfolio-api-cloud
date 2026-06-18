use actix_web::{get, web, HttpResponse};
use serde_json::Map;

use crate::error::ApiError;
use crate::repo::{profile_id, query_all_sections};
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/profile", skip(state))]
#[get("/profile")]
pub async fn get_profile(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let sections = query_all_sections(state.get_ref(), profile_id()).await?;

    let mut response = Map::with_capacity(sections.len());
    for section in sections {
        response.insert(section.section_type, section.data);
    }

    Ok(HttpResponse::Ok().json(response))
}
