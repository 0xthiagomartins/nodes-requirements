#[tokio::test]
async fn test_api_key_crud() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    // Test creation
    let request = CreateApiKeyRequest {
        name: "test_key".to_string(),
        requests_per_minute: Some(60),  // Add the required field
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