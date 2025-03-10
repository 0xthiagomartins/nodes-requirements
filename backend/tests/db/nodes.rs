use backend::{
    db,
    models::CreateNodeRequest,
};

#[tokio::test]
async fn test_node_crud() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    // Test node creation
    let node_request = CreateNodeRequest {
        blockchain_type: "test_chain".to_string(),
        cpu_cores: 2,
        ram_gb: 4,
        storage_gb: 100,
        network_mbps: 1000,
    };

    let node = db::nodes::create_node(&pool, node_request)
        .await
        .expect("Failed to create node");

    assert_eq!(node.blockchain_type, "test_chain");
}
