use actix_web::{test, App, web, dev::Service};
use actix_http::Request;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use backend::Node;

pub async fn setup_test_app() -> (
    impl Service<Request, Response = ServiceResponse<impl MessageBody>, Error = actix_web::Error>,
    SqlitePool,
) {
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
            .configure(backend::routes::nodes::config)
    ).await;

    (app, pool)
}

pub async fn insert_test_node(pool: &SqlitePool) -> Node {
    sqlx::query_as!(
        Node,
        r#"
        INSERT INTO nodes (blockchain_type, cpu_cores, ram_gb, storage_gb, network_mbps)
        VALUES ('test-chain', 4, 8, 500, 1000)
        RETURNING 
            id, blockchain_type,
            cpu_cores as "cpu_cores: i32",
            ram_gb as "ram_gb: i32",
            storage_gb as "storage_gb: i32",
            network_mbps as "network_mbps: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>"
        "#
    )
    .fetch_one(pool)
    .await
    .expect("Failed to insert test node")
} 