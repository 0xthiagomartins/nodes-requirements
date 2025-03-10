use backend::{db, models::CreateApiKeyRequest};

#[tokio::test]
async fn test_api_key_table_exists() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let result =
        sqlx::query!("SELECT name FROM sqlite_master WHERE type='table' AND name='api_keys'")
            .fetch_optional(&pool)
            .await
            .unwrap();

    assert!(result.is_some(), "api_keys table should exist");
}

#[tokio::test]
async fn test_api_key_crud() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    // Test creation
    let request = CreateApiKeyRequest {
        name: "test_key".to_string(),
        requests_per_minute: Some(60),
    };
    let response = db::api_keys::create_api_key(&pool, request).await.unwrap();
    assert!(!response.key.is_empty());

    // Test validation
    let is_valid = db::api_keys::validate_api_key(&pool, &response.key)
        .await
        .unwrap();
    assert!(is_valid);

    // Test update last used
    db::api_keys::update_last_used(&pool, &response.key)
        .await
        .unwrap();
}
