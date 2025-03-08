use backend::{
    db,
    services::{price_fetcher::GcpPriceFetcher, scheduler::PriceUpdateScheduler},
};
use dotenv::dotenv;
use std::{env, sync::Arc};

use crate::common::create_test_node;

#[tokio::test]
async fn test_scheduler_updates_prices() {
    // Load environment variables
    dotenv().ok();
    let api_key = env::var("GCP_API_KEY").expect("GCP_API_KEY must be set");

    // Setup
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    // Use real GCP price fetcher with 1 minute interval
    let fetcher = Arc::new(GcpPriceFetcher::new(api_key));
    let scheduler = PriceUpdateScheduler::new(pool.clone(), fetcher, 1);

    // Create a test node
    let node = create_test_node(&pool).await;
    println!("Created test node with id: {}", node.id);

    // Start scheduler
    scheduler.start().await;
    println!("Scheduler started");

    // Trigger an immediate update
    scheduler
        .trigger_update()
        .await
        .expect("Failed to update prices");

    // Stop scheduler
    scheduler.stop().await;
    println!("Scheduler stopped");

    // Verify prices were updated
    let prices = db::price_history::get_node_prices(&pool, node.id)
        .await
        .unwrap();

    println!("Found {} prices", prices.len());
    assert!(!prices.is_empty(), "No prices were recorded");

    // Verify price values are reasonable
    let price = prices.first().unwrap();
    println!("Recorded price: ${} per hour", price.price_per_hour);
    assert!(price.price_per_hour > 0.0, "Price should be greater than 0");
    assert_eq!(price.provider, "gcp", "Provider should be GCP");
    assert_eq!(price.currency, "USD", "Currency should be USD");
}
