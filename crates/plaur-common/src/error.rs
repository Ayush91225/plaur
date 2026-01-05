use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlaurdError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Authentication error: {message}")]
    Authentication { message: String },
    
    #[error("Authorization error: {message}")]
    Authorization { message: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Conflict: {message}")]
    Conflict { message: String },
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },
    
    #[error("Internal server error: {message}")]
    Internal { message: String },
}

impl IntoResponse for PlaurdError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            PlaurdError::Authentication { .. } => (StatusCode::UNAUTHORIZED, self.to_string()),
            PlaurdError::Authorization { .. } => (StatusCode::FORBIDDEN, self.to_string()),
            PlaurdError::Validation { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            PlaurdError::NotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            PlaurdError::Conflict { .. } => (StatusCode::CONFLICT, self.to_string()),
            PlaurdError::RateLimit => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = json!({
            "error": error_message,
            "status": status.as_u16()
        });

        (status, axum::Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, PlaurdError>;