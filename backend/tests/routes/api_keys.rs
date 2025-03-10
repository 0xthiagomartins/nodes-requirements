use actix_web::test;
use serde_json::json;

use crate::common;

#[actix_rt::test]
async fn test_create_api_key() {
    let (app, _pool) = common::setup_test_app().await;
    
    let req = test::TestRequest::post()
        .uri("/api-keys")
        .insert_header(("content-type", "application/json"))
        .set_json(json!({
            "name": "test_key",
            "requests_per_minute": 60
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("key").is_some());
}

#[actix_rt::test]
async fn test_revoke_api_key() {
    let (app, pool) = common::setup_test_app().await;
    
    // Create a test API key first
    let api_key = common::insert_test_api_key(&pool).await;
    
    let req = test::TestRequest::delete()
        .uri(&format!("/api-keys/{}", api_key.id))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // Verify key is revoked by trying to use it
    let req = test::TestRequest::get()
        .uri("/nodes")
        .insert_header(("X-API-Key", api_key.key))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
} 