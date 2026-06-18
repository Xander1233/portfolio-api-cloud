use actix_web::{get, web, HttpResponse};

use crate::error::ApiError;
use crate::models::PersonalInfo;
use crate::routes::sections::handler::serve_section;
use crate::state::AppState;

#[tracing::instrument(name = "GET /sections/personal_information", skip(state))]
#[get("/personal_information")]
pub async fn get_personal_information(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    serve_section::<PersonalInfo>(state.get_ref(), "PERSONAL_INFORMATION").await
}
