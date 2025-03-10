use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PriceInfo {
    pub price_per_hour: f64,
    pub timestamp: chrono::DateTime<Utc>,
}

pub trait PriceFetcher: Send + Sync {
    fn fetch_price(&self) -> f64;
}

pub struct GcpPriceFetcher;

impl PriceFetcher for GcpPriceFetcher {
    fn fetch_price(&self) -> f64 {
        // TODO: Implement actual GCP price fetching
        // For now, return a placeholder value
        0.5
    }
}

pub struct AwsPriceFetcher;

impl PriceFetcher for AwsPriceFetcher {
    fn fetch_price(&self) -> f64 {
        // TODO: Implement actual AWS price fetching
        // For now, return a placeholder value
        0.4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcp_price_fetcher() {
        let fetcher = GcpPriceFetcher;
        let price = fetcher.fetch_price();
        assert!(price > 0.0);
    }

    #[test]
    fn test_aws_price_fetcher() {
        let fetcher = AwsPriceFetcher;
        let price = fetcher.fetch_price();
        assert!(price > 0.0);
    }
}
