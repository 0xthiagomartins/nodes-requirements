use sqlx::SqlitePool;
use chrono::{DateTime, Utc};

use crate::models::{PriceHistory, CreatePriceHistoryRequest};
use crate::error::AppError;

pub async fn create_price_history(
    pool: &SqlitePool,
    price: CreatePriceHistoryRequest,
) -> Result<PriceHistory, AppError> {
    // Check if node exists
    let node_exists = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM nodes WHERE id = ?",
        price.node_id
    )
    .fetch_one(pool)
    .await?;

    if node_exists == 0 {
        return Err(AppError::NotFound(format!("Node {} not found", price.node_id)));
    }

    let price = sqlx::query_as!(
        PriceHistory,
        r#"
        INSERT INTO price_history (node_id, provider, price_per_hour, currency)
        VALUES (?, ?, ?, ?)
        RETURNING 
            id as "id!: i64",
            node_id as "node_id!: i64",
            provider as "provider!",
            price_per_hour as "price_per_hour!: f64",
            currency as "currency!",
            fetched_at as "fetched_at!: DateTime<Utc>"
        "#,
        price.node_id,
        price.provider,
        price.price_per_hour,
        price.currency
    )
    .fetch_one(pool)
    .await?;

    Ok(price)
}

pub async fn get_node_prices(
    pool: &SqlitePool,
    node_id: i64,
) -> Result<Vec<PriceHistory>, AppError> {
    let prices = sqlx::query_as!(
        PriceHistory,
        r#"
        SELECT 
            id as "id!: i64",
            node_id as "node_id!: i64",
            provider as "provider!",
            price_per_hour as "price_per_hour!: f64",
            currency as "currency!",
            fetched_at as "fetched_at!: DateTime<Utc>"
        FROM price_history
        WHERE node_id = ?
        ORDER BY fetched_at DESC
        "#,
        node_id
    )
    .fetch_all(pool)
    .await?;

    Ok(prices)
}

pub async fn get_latest_node_prices(
    pool: &SqlitePool,
    node_id: i64,
) -> Result<Vec<PriceHistory>, AppError> {
    let prices = sqlx::query_as!(
        PriceHistory,
        r#"
        SELECT 
            id as "id!: i64",
            node_id as "node_id!: i64",
            provider as "provider!",
            price_per_hour as "price_per_hour!: f64",
            currency as "currency!",
            fetched_at as "fetched_at!: DateTime<Utc>"
        FROM price_history
        WHERE node_id = ? AND
            fetched_at = (
                SELECT MAX(fetched_at)
                FROM price_history ph2
                WHERE ph2.node_id = price_history.node_id
                AND ph2.provider = price_history.provider
            )
        "#,
        node_id
    )
    .fetch_all(pool)
    .await?;

    Ok(prices)
} 