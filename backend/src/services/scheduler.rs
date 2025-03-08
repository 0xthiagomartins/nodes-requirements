use rand::Rng;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;

use crate::db;
use crate::error::AppError;
use crate::models::{CreatePriceHistoryRequest, Node};
use crate::services::price_fetcher::PriceFetcher;
use sqlx::SqlitePool;

pub struct PriceUpdateScheduler {
    pool: SqlitePool,
    pub(crate) price_fetcher: Arc<dyn PriceFetcher + Send + Sync>,
    interval: Duration,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl PriceUpdateScheduler {
    pub fn new(pool: SqlitePool, price_fetcher: Arc<dyn PriceFetcher + Send + Sync>) -> Self {
        // Get interval from env var, default to 60 minutes if not set
        let interval_minutes = env::var("PRICE_UPDATE_INTERVAL_MINUTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        Self {
            pool,
            price_fetcher,
            interval: Duration::from_secs(interval_minutes * 60),
            task_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self) {
        let pool = self.pool.clone();
        let price_fetcher = self.price_fetcher.clone();
        let interval = self.interval;

        let task = tokio::spawn(async move {
            // Add initial jitter delay (0-30% of interval)
            let jitter_secs = rand::thread_rng().gen_range(0..=(interval.as_secs() * 3 / 10));
            time::sleep(Duration::from_secs(jitter_secs)).await;

            let mut interval_timer = time::interval(interval);

            loop {
                interval_timer.tick().await;
                if let Err(e) = Self::update_prices(&pool, &price_fetcher).await {
                    eprintln!("Failed to update prices: {}", e);
                }
            }
        });

        let mut handle = self.task_handle.lock().await;
        *handle = Some(task);
    }

    pub async fn stop(&self) {
        if let Some(handle) = self.task_handle.lock().await.take() {
            handle.abort();
        }
    }

    pub(crate) async fn update_prices(
        pool: &SqlitePool,
        price_fetcher: &Arc<dyn PriceFetcher + Send + Sync>,
    ) -> Result<(), AppError> {
        println!("Starting price update cycle");

        // Fetch all nodes
        let nodes = db::nodes::get_all_nodes(pool).await?;
        println!("Found {} nodes to update", nodes.len());

        for node in nodes {
            println!("Updating prices for node {}", node.id);
            match Self::update_node_prices(pool, price_fetcher, &node).await {
                Ok(_) => println!("Successfully updated prices for node {}", node.id),
                Err(e) => eprintln!("Failed to update prices for node {}: {}", node.id, e),
            }
        }

        println!("Completed price update cycle");
        Ok(())
    }

    async fn update_node_prices(
        pool: &SqlitePool,
        price_fetcher: &Arc<dyn PriceFetcher + Send + Sync>,
        node: &Node,
    ) -> Result<(), AppError> {
        println!(
            "Fetching prices for node {} (CPU: {}, RAM: {}, Storage: {})",
            node.id, node.cpu_cores, node.ram_gb, node.storage_gb
        );

        // Fetch current prices from cloud providers
        let gcp_price = price_fetcher
            .fetch_price(node.cpu_cores, node.ram_gb, node.storage_gb)
            .await?;

        println!("Got GCP price: ${}/hour", gcp_price);

        // Create price history records
        let gcp_price_history = CreatePriceHistoryRequest {
            node_id: node.id,
            provider: "gcp".to_string(),
            price_per_hour: gcp_price,
            currency: "USD".to_string(),
        };

        // Store in database
        println!("Saving price history record for node {}", node.id);
        let result = db::price_history::create_price_history(pool, gcp_price_history).await?;
        println!(
            "Successfully saved price history record with id {}",
            result.id
        );

        Ok(())
    }

    pub async fn trigger_update(&self) -> Result<(), AppError> {
        Self::update_prices(&self.pool, &self.price_fetcher).await
    }
}
