use actix_web::test;
use serde_json::json;

use crate::common;
use backend::Node;

#[actix_rt::test]
async fn test_get_nodes_empty() {
    let (app, _pool) = common::setup_test_app().await;
    
    let req = test::TestRequest::get().uri("/nodes").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body: Vec<Node> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_rt::test]
async fn test_create_node_success() {
    let (app, _pool) = common::setup_test_app().await;
    
    let req = test::TestRequest::post()
        .uri("/nodes")
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "blockchain_type": "test-chain",
            "cpu_cores": 4,
            "ram_gb": 8,
            "storage_gb": 500,
            "network_mbps": 1000
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    let node: Node = test::read_body_json(resp).await;
    assert_eq!(node.blockchain_type, "test-chain");
    assert_eq!(node.cpu_cores, 4);
    assert_eq!(node.ram_gb, 8);
    assert_eq!(node.storage_gb, 500);
    assert_eq!(node.network_mbps, 1000);
}

#[actix_rt::test]
async fn test_get_node_success() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node first
    let inserted_node = common::insert_test_node(&pool).await;
    
    let req = test::TestRequest::get()
        .uri(&format!("/nodes/{}", inserted_node.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let node: Node = test::read_body_json(resp).await;
    assert_eq!(node.id, inserted_node.id);
    assert_eq!(node.blockchain_type, "test-chain");
}

// ... other tests ... 