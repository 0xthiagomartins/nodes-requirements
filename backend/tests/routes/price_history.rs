use actix_web::test;
use serde_json::json;

use crate::common;
use backend::models::PriceHistory;

#[actix_rt::test]
async fn test_create_price_success() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node first
    let node = common::insert_test_node(&pool).await;
    
    // Print the actual request being made
    let uri = format!("/nodes/{}/prices", node.id);
    println!("Making request to: {}", uri);
    
    let req = test::TestRequest::post()
        .uri(&uri)
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "provider": "gcp",
            "price_per_hour": 1.25,
            "currency": "USD"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    let price: PriceHistory = test::read_body_json(resp).await;
    assert_eq!(price.node_id, node.id);
    assert_eq!(price.provider, "gcp");
    assert_eq!(price.price_per_hour, 1.25);
    assert_eq!(price.currency, "USD");
}

#[actix_rt::test]
async fn test_create_price_node_not_found() {
    let (app, _pool) = common::setup_test_app().await;
    
    let req = test::TestRequest::post()
        .uri("/nodes/999/prices")
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "provider": "gcp",
            "price_per_hour": 1.25,
            "currency": "USD"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_get_node_prices() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node and price
    let node = common::insert_test_node(&pool).await;
    let _price = common::insert_test_price(&pool, node.id).await;
    
    let req = test::TestRequest::get()
        .uri(&format!("/nodes/{}/prices", node.id))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    let prices: Vec<PriceHistory> = test::read_body_json(resp).await;
    assert!(!prices.is_empty());
    assert_eq!(prices[0].node_id, node.id);
}

#[actix_rt::test]
async fn test_get_latest_node_prices() {
    let (app, pool) = common::setup_test_app().await;
    
    // Insert a test node and prices with different providers
    let node = common::insert_test_node(&pool).await;
    let _price1 = common::insert_test_price(&pool, node.id).await;
    let _price2 = common::insert_test_price(&pool, node.id).await;
    
    let req = test::TestRequest::get()
        .uri(&format!("/nodes/{}/prices/latest", node.id))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Response status: {}", resp.status());

    let prices: Vec<PriceHistory> = test::read_body_json(resp).await;
    assert!(!prices.is_empty());
    assert!(prices.len() <= 2); // Should have at most 2 prices (one per provider)
    assert_eq!(prices[0].node_id, node.id);
} 