use actix_web::{delete, post, web, HttpResponse};
use sqlx::SqlitePool;
use validator::Validate;

use crate::error::AppError;
use crate::models::CreateApiKeyRequest;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-keys")
            .service(create_api_key)
            .service(revoke_api_key),
    );
}

#[post("")]
async fn create_api_key(
    pool: web::Data<SqlitePool>,
    request: web::Json<CreateApiKeyRequest>,
) -> Result<HttpResponse, AppError> {
    request.validate()?;
    let api_key = crate::db::api_keys::create_api_key(&pool, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(api_key))
}

#[delete("/{id}")]
async fn revoke_api_key(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    crate::db::api_keys::revoke_api_key(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
