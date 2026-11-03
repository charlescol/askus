use thiserror::Error;

use crate::common::error::ApiError;

#[derive(Debug, Error)]
pub enum PollError {
    #[error("Invalid poll request: {0}")]
    InvalidPollRequest(String),

    #[error("LLM Server Error: {0}")]
    LLMServerError(#[from] anyhow::Error),

    #[error("Invalid poll response: {0}")]
    InvalidPollResponse(String),
}

impl From<PollError> for ApiError {
    fn from(error: PollError) -> Self {
        match error {
            PollError::InvalidPollRequest(msg) => ApiError::BadRequest(msg.to_string()),
            PollError::LLMServerError(_) => ApiError::Internal,
            PollError::InvalidPollResponse(_) => ApiError::Internal,
        }
    }
}
