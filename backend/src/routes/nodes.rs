use actix_web::{web, HttpResponse, post, put};
use sqlx::SqlitePool;
use validator::Validate;

use crate::models::{Node, CreateNodeRequest, UpdateNodeRequest};
use crate::error::AppError;
use crate::db;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nodes")
            .route("", web::get().to(get_nodes))
            .route("/{id}", web::get().to(get_node))
            .service(create_node)
            .service(update_node)
    );
}

async fn get_nodes(pool: web::Data<SqlitePool>) -> Result<HttpResponse, AppError> {
    let nodes = sqlx::query_as::<_, Node>("SELECT * FROM nodes")
        .fetch_all(pool.get_ref())
        .await
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

    match node {
        Some(node) => Ok(HttpResponse::Ok().json(node)),
        None => Err(AppError::NotFound("Node not found".to_string())),
    }
}

#[post("")]
async fn create_node(
    pool: web::Data<SqlitePool>,
    node: web::Json<CreateNodeRequest>,
) -> Result<HttpResponse, AppError> {
    node.validate()?;
    let created = db::nodes::create_node(&pool, node.into_inner()).await?;
    Ok(HttpResponse::Created().json(&created))
}

#[put("/{id}")]
async fn update_node(
    id: web::Path<i64>,
    update: web::Json<UpdateNodeRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    update.validate()?;
    let updated = db::nodes::update_node(&pool, id.into_inner(), update.into_inner()).await?;
    Ok(HttpResponse::Ok().json(&updated))
} 