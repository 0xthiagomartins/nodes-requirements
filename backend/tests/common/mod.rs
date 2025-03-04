use actix_web::{test, App, web, dev::Service};
use actix_http::Request;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use backend::models::{Node, PriceHistory};

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

    // Verify tables exist
    sqlx::query!("SELECT 1 as table_exists FROM sqlite_master WHERE type='table' AND name='price_history'")
        .fetch_optional(&pool)
        .await
        .expect("Failed to check if price_history table exists")
        .expect("price_history table does not exist");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(backend::routes::nodes::config)
            .configure(backend::routes::price_history::config)
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

pub async fn insert_test_price(pool: &SqlitePool, node_id: i64) -> PriceHistory {
    static PROVIDERS: [&str; 2] = ["gcp", "hetzner"];
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    
    let idx = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) % PROVIDERS.len();
    let provider = PROVIDERS[idx];
    let seconds = idx as i32;  // Convert to i32 before using in query

    sqlx::query_as!(
        PriceHistory,
        r#"
        INSERT INTO price_history (node_id, provider, price_per_hour, currency, fetched_at)
        VALUES (?, ?, 1.25, 'USD', datetime('now', ? || ' seconds'))
        RETURNING 
            id as "id!: i64",
            node_id as "node_id!: i64",
            provider as "provider!",
            price_per_hour as "price_per_hour!: f64",
            currency as "currency!",
            fetched_at as "fetched_at!: DateTime<Utc>"
        "#,
        node_id,
        provider,
        seconds  // Use the stored i32 value
    )
    .fetch_one(pool)
    .await
    .expect("Failed to insert test price")
} 