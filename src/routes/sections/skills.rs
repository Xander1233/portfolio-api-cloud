use actix_web::{get, web, HttpResponse};

use crate::config::CONFIG;
use crate::error::ApiError;
use crate::models::Skills;
use crate::routes::sections::handler::serve_section_with;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/skills", skip(state))]
#[get("/skills")]
pub async fn get_skills(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let cdn_base = CONFIG.assets.cdn_base_url.trim_end_matches('/').to_string();

    serve_section_with::<Skills, _>(state.get_ref(), "SKILLS", |section| {
        for skill in &mut section.data.items {
            if !skill.logo.starts_with("http") {
                skill.logo = format!("{cdn_base}/{}", skill.logo.trim_start_matches('/'));
            }
        }
        section.data.items.sort_by(|a, b| b.level.cmp(&a.level));
    })
    .await
}
