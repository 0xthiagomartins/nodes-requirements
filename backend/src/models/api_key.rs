use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub requests_per_minute: i32,
    pub requests_this_minute: i32,
    pub last_request_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(range(min = 1, max = 1000))]
    pub requests_per_minute: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub requests_per_minute: i32,
}
