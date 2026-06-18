use actix_web::{get, web, HttpResponse};

use crate::error::ApiError;
use crate::models::Projects;
use crate::routes::sections::handler::serve_section_with;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/projects", skip(state))]
#[get("/projects")]
pub async fn get_projects(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    serve_section_with::<Projects, _>(state.get_ref(), "PROJECTS", |section| {
        section.data.items.sort_by_key(|item| item.order);
    })
    .await
}
