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

#[actix_rt::test]
async fn test_update_node_success() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node first
    let inserted_node = common::insert_test_node(&pool).await;
    
    let req = test::TestRequest::put()
        .uri(&format!("/nodes/{}", inserted_node.id))
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "blockchain_type": "updated-chain",
            "cpu_cores": 16,
            "ram_gb": 32,
            "storage_gb": 1000,
            "network_mbps": 2000
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    let node: Node = test::read_body_json(resp).await;
    assert_eq!(node.blockchain_type, "updated-chain");
    assert_eq!(node.cpu_cores, 16);
    assert_eq!(node.ram_gb, 32);
    assert_eq!(node.storage_gb, 1000);
    assert_eq!(node.network_mbps, 2000);
}

#[actix_rt::test]
async fn test_update_node_not_found() {
    let (app, _pool) = common::setup_test_app().await;
    
    let req = test::TestRequest::put()
        .uri("/nodes/999")
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "blockchain_type": "updated-chain"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_update_node_partial() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node first
    let inserted_node = common::insert_test_node(&pool).await;
    
    let req = test::TestRequest::put()
        .uri(&format!("/nodes/{}", inserted_node.id))
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "cpu_cores": 8
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    let node: Node = test::read_body_json(resp).await;
    assert_eq!(node.cpu_cores, 8);
    // Other fields should remain unchanged
    assert_eq!(node.blockchain_type, inserted_node.blockchain_type);
    assert_eq!(node.ram_gb, inserted_node.ram_gb);
    assert_eq!(node.storage_gb, inserted_node.storage_gb);
    assert_eq!(node.network_mbps, inserted_node.network_mbps);
}

#[actix_rt::test]
async fn test_delete_node_success() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node first
    let inserted_node = common::insert_test_node(&pool).await;
    
    let req = test::TestRequest::delete()
        .uri(&format!("/nodes/{}", inserted_node.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    // Verify node was deleted
    let req = test::TestRequest::get()
        .uri(&format!("/nodes/{}", inserted_node.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_delete_node_not_found() {
    let (app, _pool) = common::setup_test_app().await;
    
    let req = test::TestRequest::delete()
        .uri("/nodes/999")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 404);
}

// ... other tests ... 