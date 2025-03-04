use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Not Found")]
    NotFound,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                log::error!("Internal server error occurred");
                HttpResponse::InternalServerError().json(self)
            }
            AppError::NotFound => HttpResponse::NotFound().json(self),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(self),
        }
    }
}
