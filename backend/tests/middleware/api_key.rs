use actix_web::{test, web, App, HttpResponse};
use backend::middleware::ApiKeyMiddleware;

use crate::common::insert_test_api_key;

#[actix_rt::test]
async fn test_api_key_middleware() {
    let (app, pool) = crate::common::setup_test_app().await;
    
    // Create a test API key
    let api_key = insert_test_api_key(&pool).await;

    // Test valid API key
    let req = test::TestRequest::get()
        .uri("/nodes")
        .insert_header(("X-API-Key", api_key.key.clone()))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // Test invalid API key
    let req = test::TestRequest::get()
        .uri("/nodes")
        .insert_header(("X-API-Key", "invalid-key"))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);

    // Test missing API key
    let req = test::TestRequest::get().uri("/nodes").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);

    // Test API key management endpoint bypass
    let req = test::TestRequest::post().uri("/api-keys").to_request();
    let resp = test::call_service(&app, req).await;
    assert_ne!(resp.status(), 401); // Should not return 401
} 