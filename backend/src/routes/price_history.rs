use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use validator::Validate;

use crate::models::CreatePriceHistoryRequest;
use crate::error::AppError;
use crate::db;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Register routes directly since they'll be under the /nodes scope from nodes.rs
    cfg
        .service(
            web::resource("/{id}/prices")
                .route(web::get().to(get_node_prices))
                .route(web::post().to(create_price))
        )
        .service(
            web::resource("/{id}/prices/latest")
                .route(web::get().to(get_latest_node_prices))
        );
}

async fn get_node_prices(
    id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    let prices = db::price_history::get_node_prices(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(prices))
}

async fn get_latest_node_prices(
    id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    let prices = db::price_history::get_latest_node_prices(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(prices))
}

async fn create_price(
    id: web::Path<i64>,
    mut price: web::Json<CreatePriceHistoryRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    price.0.node_id = id.into_inner();
    price.validate()?;
    let created = db::price_history::create_price_history(&pool, price.into_inner()).await?;
    Ok(HttpResponse::Created().json(created))
} 