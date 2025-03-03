use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Node {
    pub id: i64,
    pub blockchain_type: String,
    pub cpu_cores: i32,
    pub ram_gb: i32,
    pub storage_gb: i32,
    pub network_mbps: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNodeRequest {
    pub blockchain_type: String,
    pub cpu_cores: i32,
    pub ram_gb: i32,
    pub storage_gb: i32,
    pub network_mbps: i32,
}
