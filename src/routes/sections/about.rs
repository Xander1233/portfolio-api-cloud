use actix_web::{get, web, HttpResponse};

use crate::error::ApiError;
use crate::models::About;
use crate::routes::sections::handler::serve_section;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/about", skip(state))]
#[get("/about")]
pub async fn get_about(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    serve_section::<About>(state.get_ref(), "ABOUT").await
}
