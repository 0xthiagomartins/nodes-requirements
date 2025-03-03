use actix_web::{web, HttpResponse, Scope};
use sqlx::SqlitePool;

use crate::models::Node;

pub fn config() -> Scope {
    web::scope("/nodes")
        .route("", web::get().to(get_nodes))
}

async fn get_nodes(pool: web::Data<SqlitePool>) -> HttpResponse {
    match sqlx::query_as::<_, Node>("SELECT * FROM nodes")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(nodes) => HttpResponse::Ok().json(nodes),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
} 