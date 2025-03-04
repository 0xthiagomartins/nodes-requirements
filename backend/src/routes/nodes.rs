use actix_web::{web, HttpResponse, Scope};
use sqlx::SqlitePool;

use crate::models::Node;
use crate::error::AppError;

pub fn config() -> Scope {
    web::scope("/nodes")
        .route("", web::get().to(get_nodes))
        .route("/{id}", web::get().to(get_node))
}

async fn get_nodes(pool: web::Data<SqlitePool>) -> Result<HttpResponse, AppError> {
    let nodes = sqlx::query_as::<_, Node>("SELECT * FROM nodes")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(nodes))
}

async fn get_node(
    id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    let node = sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE id = ?")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match node {
        Some(node) => Ok(HttpResponse::Ok().json(node)),
        None => Err(AppError::NotFound),
    }
} 