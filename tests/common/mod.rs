use sqlx::SqlitePool;
use backend::models::{CreateNodeRequest, Node, CreateApiKeyRequest, CreateApiKeyResponse};

pub async fn insert_test_api_key(pool: &SqlitePool) -> CreateApiKeyResponse {
    let request = CreateApiKeyRequest {
        name: "test_key".to_string(),
        requests_per_minute: Some(60),  // Add the required field
    };

    backend::db::api_keys::create_api_key(pool, request)
        .await
        .expect("Failed to create test API key")
} 