use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
            AppError::Database(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Database error"
            })),
            AppError::Validation(err) => HttpResponse::BadRequest().json(json!({
                "error": "Validation error",
                "details": err.to_string()
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": msg
            })),
            AppError::Conflict(msg) => HttpResponse::Conflict().json(json!({
                "error": msg
            })),
            AppError::ExternalService(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "External service error",
                "details": msg
            })),
            AppError::InvalidInput(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Invalid input",
                "details": msg
            })),
        }
    }
}
