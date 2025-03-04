#[cfg(test)]
mod tests {
    use actix_web::{test, App, web};
    use sqlx::SqlitePool;

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
} 