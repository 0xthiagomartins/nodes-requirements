use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
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
        }
    }
}
