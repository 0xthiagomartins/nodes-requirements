use backend::db;
use backend::models::CreatePriceHistoryRequest;

#[tokio::test]
async fn test_price_history_crud() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    // Create a test node first
    let node = crate::common::create_test_node(&pool).await;

    // Test price history creation
    let price_request = CreatePriceHistoryRequest {
        node_id: node.id,
        price_per_hour: 1.5,
        provider: "gcp".to_string(),
        currency: "USD".to_string(),
    };

    let price = db::price_history::create_price_history(&pool, price_request)
        .await
        .expect("Failed to create price history");

    assert_eq!(price.node_id, node.id);
    assert_eq!(price.price_per_hour, 1.5);
}
