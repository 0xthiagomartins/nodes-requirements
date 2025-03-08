use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;

use crate::error::AppError;
use crate::services::price_fetcher::PriceFetcher;
use sqlx::SqlitePool;

pub struct PriceUpdateScheduler {
    pool: SqlitePool,
    price_fetcher: Arc<dyn PriceFetcher + Send + Sync>,
    interval: Duration,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl PriceUpdateScheduler {
    pub fn new(
        pool: SqlitePool,
        price_fetcher: Arc<dyn PriceFetcher + Send + Sync>,
        interval_minutes: u64,
    ) -> Self {
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

    async fn update_prices(
        pool: &SqlitePool,
        price_fetcher: &Arc<dyn PriceFetcher + Send + Sync>,
    ) -> Result<(), AppError> {
        // TODO: Implement price update logic
        Ok(())
    }
}
