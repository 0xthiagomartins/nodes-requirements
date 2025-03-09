use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PriceHistory {
    pub id: i64,
    pub node_id: i64,
    pub provider: String,
    pub price_per_hour: f64,
    pub currency: String,
    pub fetched_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePriceHistoryRequest {
    #[serde(skip_deserializing)] // This will be set from the path parameter
    pub node_id: i64,
    #[validate(length(min = 1, message = "provider cannot be empty"))]
    pub provider: String,
    #[validate(range(min = 0.0, message = "price must be non-negative"))]
    pub price_per_hour: f64,
    #[validate(length(min = 3, max = 3, message = "currency must be 3 characters"))]
    pub currency: String,
}
