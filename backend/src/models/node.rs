use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Node {
    pub id: i64,
    pub blockchain_type: String,
    pub cpu_cores: i32,
    pub ram_gb: i32,
    pub storage_gb: i32,
    pub network_mbps: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNodeRequest {
    #[validate(length(min = 1, message = "blockchain_type cannot be empty"))]
    pub blockchain_type: String,
    #[validate(range(min = 1, message = "cpu_cores must be at least 1"))]
    pub cpu_cores: i32,
    #[validate(range(min = 1, message = "ram_gb must be at least 1"))]
    pub ram_gb: i32,
    #[validate(range(min = 1, message = "storage_gb must be at least 1"))]
    pub storage_gb: i32,
    #[validate(range(min = 1, message = "network_mbps must be at least 1"))]
    pub network_mbps: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNodeRequest {
    #[validate(length(min = 1, message = "blockchain_type cannot be empty"))]
    pub blockchain_type: Option<String>,
    #[validate(range(min = 1, message = "cpu_cores must be at least 1"))]
    pub cpu_cores: Option<i32>,
    #[validate(range(min = 1, message = "ram_gb must be at least 1"))]
    pub ram_gb: Option<i32>,
    #[validate(range(min = 1, message = "storage_gb must be at least 1"))]
    pub storage_gb: Option<i32>,
    #[validate(range(min = 1, message = "network_mbps must be at least 1"))]
    pub network_mbps: Option<i32>,
}
