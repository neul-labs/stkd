//! API error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// API error type.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Bad request
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Unauthorized
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Forbidden
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Conflict
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] stack_db::DbError),
}

/// Result type alias for API operations.
pub type ApiResult<T> = Result<T, ApiError>;

/// Error response body.
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "unauthorized"),
            ApiError::Forbidden(_) => (StatusCode::FORBIDDEN, "forbidden"),
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            ApiError::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}
