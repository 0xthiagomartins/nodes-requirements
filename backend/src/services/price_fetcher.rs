use async_trait::async_trait;
use serde::Deserialize;
use crate::error::AppError;

#[async_trait]
pub trait PriceFetcher {
    async fn fetch_price(&self, cpu_cores: i32, ram_gb: i32, storage_gb: i32) -> Result<f64, AppError>;
}

pub struct GcpPriceFetcher {
    api_key: String,
}

pub struct HetznerPriceFetcher {
    api_key: String,
}

#[async_trait]
impl PriceFetcher for GcpPriceFetcher {
    async fn fetch_price(&self, cpu_cores: i32, ram_gb: i32, storage_gb: i32) -> Result<f64, AppError> {
        // TODO: Implement GCP price fetching
        Ok(0.0)
    }
}

#[async_trait]
impl PriceFetcher for HetznerPriceFetcher {
    async fn fetch_price(&self, cpu_cores: i32, ram_gb: i32, storage_gb: i32) -> Result<f64, AppError> {
        // TODO: Implement Hetzner price fetching
        Ok(0.0)
    }
} 