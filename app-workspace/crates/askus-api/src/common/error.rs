use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal,

    #[error("Service unavailable")]
    ServiceUnavailable,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.to_string(),
        })
    }
}
