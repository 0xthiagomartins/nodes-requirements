#[cfg(test)]
mod tests {
    use actix_web::{test, App, web};
    use sqlx::SqlitePool;
    use serde_json::json;

    use crate::routes;

    async fn setup_test_app() -> (actix_web::test::TestApp, SqlitePool) {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations on in-memory database
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(routes::nodes::config())
        ).await;

        (app, pool)
    }

    #[actix_rt::test]
    async fn test_get_nodes_empty() {
        let (app, _pool) = setup_test_app().await;
        
        let req = test::TestRequest::get().uri("/nodes").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
        
        let body: Vec<crate::models::Node> = test::read_body_json(resp).await;
        assert!(body.is_empty());
    }

    #[actix_rt::test]
    async fn test_get_node_not_found() {
        let (app, _pool) = setup_test_app().await;
        
        let req = test::TestRequest::get().uri("/nodes/1").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), 404);
    }

    #[actix_rt::test]
    async fn test_create_node_success() {
        let (app, _pool) = setup_test_app().await;
        
        let req = test::TestRequest::post()
            .uri("/nodes")
            .set_json(json!({
                "blockchain_type": "test-chain",
                "cpu_cores": 4,
                "ram_gb": 8,
                "storage_gb": 500,
                "network_mbps": 1000
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let node: crate::models::Node = test::read_body_json(resp).await;
        assert_eq!(node.blockchain_type, "test-chain");
        assert_eq!(node.cpu_cores, 4);
    }

    #[actix_rt::test]
    async fn test_create_node_validation_error() {
        let (app, _pool) = setup_test_app().await;
        
        let req = test::TestRequest::post()
            .uri("/nodes")
            .set_json(json!({
                "blockchain_type": "",  // Empty string should fail validation
                "cpu_cores": 0,         // Should be at least 1
                "ram_gb": 8,
                "storage_gb": 500,
                "network_mbps": 1000
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_create_duplicate_node() {
        let (app, pool) = setup_test_app().await;
        
        // Create first node
        let node = test_utils::insert_test_node(&pool).await;
        
        // Try to create duplicate
        let req = test::TestRequest::post()
            .uri("/nodes")
            .set_json(json!({
                "blockchain_type": node.blockchain_type,
                "cpu_cores": 4,
                "ram_gb": 8,
                "storage_gb": 500,
                "network_mbps": 1000
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 409); // Conflict
    }

    #[actix_rt::test]
    async fn test_update_node_success() {
        let (app, pool) = setup_test_app().await;
        
        // Create a test node first
        let node = test_utils::insert_test_node(&pool).await;
        
        let req = test::TestRequest::put()
            .uri(&format!("/nodes/{}", node.id))
            .set_json(json!({
                "cpu_cores": 16,
                "ram_gb": 32
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let updated: crate::models::Node = test::read_body_json(resp).await;
        assert_eq!(updated.cpu_cores, 16);
        assert_eq!(updated.ram_gb, 32);
    }
} 