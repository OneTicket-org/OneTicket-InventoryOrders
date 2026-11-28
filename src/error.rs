use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AppError {
    /// The request cannot be read: malformed body, unknown or missing field, wrong type.
    Malformed(String),
    /// The URL names nothing.
    NotFound(String),
    /// The request is valid, but the state of the resource does not allow it.
    Conflict(String),
    /// The request is well formed, but a value cannot be processed.
    Invalid(String),
    /// Any other status, for example 405 or 415: the problem type stays `about:blank`.
    Other(StatusCode, String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Malformed(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Invalid(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::Other(status, msg) => (status, msg),
        };

        (status, message).into_response()
    }
}

pub async fn not_found_route() -> AppError {
    AppError::NotFound("no resource at this path".to_string())
}

pub async fn wrong_method() -> AppError {
    AppError::Other(
        StatusCode::METHOD_NOT_ALLOWED,
        "method not supported on this path".to_string(),
    )
}
