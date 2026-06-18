use actix_web::{get, web, HttpResponse};

use crate::error::ApiError;
use crate::models::Experience;
use crate::routes::sections::handler::serve_section_with;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/experience", skip(state))]
#[get("/experience")]
pub async fn get_experience(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    serve_section_with::<Experience, _>(state.get_ref(), "EXPERIENCE", |section| {
        section.data.items.sort_by_key(|item| item.order);
        for item in &mut section.data.items {
            item.career.sort_by_key(|c| c.order);
        }
    })
    .await
}
