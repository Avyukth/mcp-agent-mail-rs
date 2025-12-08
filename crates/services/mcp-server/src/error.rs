use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error; // Added thiserror import
use hyper; // Added hyper import

#[derive(Debug, Error)] // Added derive(Error)
pub enum ServerError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error), // Added for std::io::Error

    #[error(transparent)]
    Hyper(#[from] hyper::Error), // Added for hyper::error::Error

    #[error(transparent)]
    Core(#[from] lib_core::Error),

    #[error("Internal Server Error")]
    Internal,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::Anyhow(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {}", e)),
            ServerError::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("IO error: {}", e)),
            ServerError::Hyper(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {}", e)),
            ServerError::Core(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            ServerError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ServerError>;
