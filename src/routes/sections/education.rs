use actix_web::{get, web, HttpResponse};

use crate::error::ApiError;
use crate::models::Education;
use crate::routes::sections::handler::serve_section_with;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/education", skip(state))]
#[get("/education")]
pub async fn get_education(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    serve_section_with::<Education, _>(state.get_ref(), "EDUCATION", |section| {
        section.data.items.sort_by_key(|item| item.order);
    })
    .await
}
