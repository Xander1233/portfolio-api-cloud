use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("section '{0}' not found")]
    SectionNotFound(&'static str),

    #[error("dynamodb error: {0}")]
    Dynamo(String),

    #[error("deserialization error: {0}")]
    Deserialize(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    code: &'a str,
    message: String,
}

impl ApiError {
    fn code(&self) -> &'static str {
        match self {
            ApiError::SectionNotFound(_) => "section_not_found",
            ApiError::Dynamo(_) => "dynamo_error",
            ApiError::Deserialize(_) => "deserialize_error",
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::SectionNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Dynamo(_) | ApiError::Deserialize(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        if self.status_code().is_server_error() {
            tracing::error!(error = %self, code = self.code(), "request failed");
        } else {
            tracing::warn!(error = %self, code = self.code(), "request rejected");
        }

        HttpResponse::build(self.status_code()).json(ErrorBody {
            code: self.code(),
            message: self.to_string(),
        })
    }
}

impl<E, R> From<aws_sdk_dynamodb::error::SdkError<E, R>> for ApiError
where
    E: std::fmt::Debug,
    R: std::fmt::Debug,
{
    fn from(err: aws_sdk_dynamodb::error::SdkError<E, R>) -> Self {
        ApiError::Dynamo(format!("{err:?}"))
    }
}

impl From<serde_dynamo::Error> for ApiError {
    fn from(err: serde_dynamo::Error) -> Self {
        ApiError::Deserialize(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Deserialize(err.to_string())
    }
}
