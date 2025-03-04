use actix_web::{web, HttpResponse, Scope};
use sqlx::SqlitePool;
use validator::Validate;

use crate::models::Node;
use crate::error::AppError;
use crate::db::nodes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nodes")
            .route("", web::get().to(get_nodes))
            .route("/{id}", web::get().to(get_node))
            .route("/", web::post().to(create_node))
            .route("/{id}", web::put().to(update_node))
    );
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

#[post("/nodes")]
async fn create_node(
    pool: web::Data<SqlitePool>,
    node: web::Json<CreateNodeRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request body
    node.validate()?;

    let node = db::nodes::create_node(&pool, node.into_inner()).await?;
    Ok(HttpResponse::Created().json(node))
}

#[put("/nodes/{id}")]
async fn update_node(
    id: web::Path<i64>,
    update: web::Json<UpdateNodeRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    update.validate()?;
    let node = db::nodes::update_node(&pool, id.into_inner(), update.into_inner()).await?;
    Ok(HttpResponse::Ok().json(node))
} 