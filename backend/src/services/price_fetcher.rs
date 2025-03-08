use async_trait::async_trait;
use phf;
use reqwest::Client;
use serde::Deserialize;
use serde_json;

use crate::error::AppError;

// Service IDs for GCP
static GCP_SERVICES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "compute" => "6F81-5844-456A",  // Compute Engine
    "ram" => "6F81-5844-456A",      // Same as compute - RAM is part of Compute Engine
    "storage" => "6F81-5844-456A",   // Also part of Compute Engine
    "network" => "6F81-5844-456A",   // Network is also under Compute Engine
};

// Make structs public for testing
#[derive(Debug, Deserialize)]
pub struct GcpSkuResponse {
    pub skus: Vec<GcpSku>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GcpSku {
    pub name: String,
    #[serde(rename = "skuId")]
    pub sku_id: String,
    pub description: String,
    pub category: GcpCategory,
    #[serde(rename = "serviceRegions", default)]
    pub service_regions: Vec<String>,
    #[serde(rename = "pricingInfo")]
    pub pricing_info: Vec<GcpPricingInfo>,
    #[serde(rename = "serviceProviderName")]
    pub service_provider_name: String,
    #[serde(rename = "geoTaxonomy")]
    pub geo_taxonomy: Option<GeoTaxonomy>,
}

#[derive(Debug, Deserialize)]
pub struct GcpCategory {
    #[serde(rename = "serviceDisplayName")]
    pub service_display_name: String,
    #[serde(rename = "resourceFamily")]
    pub resource_family: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[serde(rename = "usageType")]
    pub usage_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GcpPricingInfo {
    #[serde(rename = "effectiveTime")]
    pub effective_time: String,
    #[serde(rename = "pricingExpression")]
    pub pricing_expression: GcpPricingExpression,
}

#[derive(Debug, Deserialize)]
pub struct GcpPricingExpression {
    #[serde(rename = "usageUnit")]
    pub usage_unit: String,
    #[serde(rename = "usageUnitDescription")]
    pub usage_unit_description: String,
    #[serde(rename = "baseUnit")]
    pub base_unit: String,
    #[serde(rename = "baseUnitDescription")]
    pub base_unit_description: String,
    #[serde(rename = "baseUnitConversionFactor")]
    pub base_unit_conversion_factor: f64,
    #[serde(rename = "displayQuantity")]
    pub display_quantity: i32,
    #[serde(rename = "tieredRates")]
    pub tiered_rates: Vec<GcpTieredRate>,
}

#[derive(Debug, Deserialize)]
pub struct GcpTieredRate {
    #[serde(rename = "startUsageAmount")]
    pub start_usage_amount: f64,
    #[serde(rename = "unitPrice")]
    pub unit_price: GcpMoney,
}

#[derive(Debug, Deserialize)]
pub struct GcpMoney {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub units: String,
    pub nanos: i32,
}

#[derive(Debug, Deserialize)]
pub struct GeoTaxonomy {
    #[serde(rename = "type")]
    pub taxonomy_type: String,
    pub regions: Vec<String>,
}

#[async_trait]
pub trait PriceFetcher {
    async fn fetch_price(
        &self,
        cpu_cores: i32,
        ram_gb: i32,
        storage_gb: i32,
    ) -> Result<f64, AppError>;
}

pub struct GcpPriceFetcher {
    api_key: String,
    client: Client,
    base_url: String,
}

#[allow(dead_code)] // Remove warning since this will be implemented later
pub struct AwsPriceFetcher {
    api_key: String,
    client: Client,
}

impl GcpPriceFetcher {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self {
            api_key,
            client,
            base_url: "https://cloudbilling.googleapis.com".to_string(),
        }
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = url.to_string();
    }

    // Make method public for testing
    pub fn calculate_sku_price(&self, sku: &GcpSku, amount: f64) -> f64 {
        let pricing_info = match sku.pricing_info.first() {
            Some(info) => info,
            None => return 0.0,
        };

        let expression = &pricing_info.pricing_expression;

        // Get the unit price
        let rate = match expression.tiered_rates.first() {
            Some(rate) => rate,
            None => return 0.0,
        };

        let dollars = rate.unit_price.units.parse::<f64>().unwrap_or(0.0);
        let nanos = rate.unit_price.nanos as f64 / 1_000_000_000.0;
        let unit_price = dollars + nanos;

        // Calculate price based on resource type and units
        match expression.usage_unit.as_str() {
            "h" => unit_price * amount,       // CPU hours
            "GiBy.h" => unit_price * amount,  // RAM GB-hours
            "GiBy.mo" => unit_price * amount, // Storage GB-months
            "GiBy" => unit_price * amount,    // Network GB
            _ => 0.0,
        }
    }

    // Make client field public for testing
    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    async fn get_service_skus(&self, service_id: &str) -> Result<Vec<GcpSku>, AppError> {
        let mut filtered_skus = Vec::new();
        let mut page_token: Option<String> = None;
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 3;

        loop {
            let url = format!("{}/v1/services/{}/skus", self.base_url, service_id);

            let mut query_params = vec![
                ("key", self.api_key.as_str()),
                ("pageSize", "100"), // Process in smaller chunks
            ];

            if let Some(token) = &page_token {
                query_params.push(("pageToken", token));
            }

            // Try to fetch page with retries
            let skus_response = loop {
                match self.fetch_page(&url, &query_params).await {
                    Ok(response) => break response,
                    Err(e) => {
                        retry_count += 1;
                        if retry_count >= MAX_RETRIES {
                            return Err(e);
                        }
                        println!(
                            "Retrying page fetch (attempt {}/{})",
                            retry_count + 1,
                            MAX_RETRIES
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            };

            // Filter SKUs from this page immediately
            filtered_skus.extend(skus_response.skus);

            // Check if there are more pages
            match skus_response.next_page_token {
                Some(token) if !token.is_empty() => {
                    page_token = Some(token);
                    retry_count = 0; // Reset retry counter for new page
                }
                _ => break,
            }
        }

        Ok(filtered_skus)
    }

    async fn fetch_page(
        &self,
        url: &str,
        query_params: &[(&str, &str)],
    ) -> Result<GcpSkuResponse, AppError> {
        let request = self
            .client
            .get(url)
            .query(query_params)
            .build()
            .map_err(|e| AppError::ExternalService(format!("Failed to build request: {}", e)))?;

        let response =
            self.client.execute(request).await.map_err(|e| {
                AppError::ExternalService(format!("Failed to fetch GCP SKUs: {}", e))
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            return Err(AppError::ExternalService(format!(
                "GCP API error (status {}): {}",
                status, error_text
            )));
        }

        let response_text = response.text().await.map_err(|e| {
            AppError::ExternalService(format!("Failed to read GCP response: {}", e))
        })?;

        serde_json::from_str(&response_text)
            .map_err(|e| AppError::ExternalService(format!("Failed to parse GCP response: {}", e)))
    }

    fn filter_skus<'a>(&self, skus: &'a [GcpSku], resource_type: &str) -> Vec<&'a GcpSku> {
        println!("Filtering {} SKUs for {}", skus.len(), resource_type);

        let filtered = skus
            .iter()
            .filter(|sku| {
                let matches = match resource_type {
                    "CPU" => {
                        sku.description.contains("CPU") && !sku.description.contains("Preemptible")
                    }
                    "RAM" => {
                        sku.description.contains("RAM") && !sku.description.contains("Preemptible")
                    }
                    "SSD" => {
                        sku.description.contains("SSD")
                            || sku.description.contains("Persistent Disk")
                    }
                    "Network" => {
                        sku.description.contains("Network") && sku.description.contains("Egress")
                    }
                    _ => false,
                } && sku.category.usage_type == "OnDemand";
                matches
            })
            .collect::<Vec<_>>();

        println!("Found {} matching SKUs", filtered.len());
        filtered
    }

    async fn calculate_resource_price(
        &self,
        resource_type: &str,
        amount: i32,
    ) -> Result<f64, AppError> {
        if amount <= 0 {
            return Ok(0.0); // Return 0 for zero or negative amounts
        }

        let service_id = GCP_SERVICES.get(resource_type).ok_or_else(|| {
            AppError::InvalidInput(format!("Invalid resource type: {}", resource_type))
        })?;

        let skus = self.get_service_skus(service_id).await?;

        println!("Got {} SKUs for service {}", skus.len(), service_id);

        // Map internal resource types to GCP resource groups
        let resource_group = match resource_type {
            "compute" => "CPU",
            "ram" => "RAM", // Add RAM mapping
            "storage" => "SSD",
            "network" => "Network",
            _ => return Err(AppError::InvalidInput("Invalid resource type".to_string())),
        };

        let filtered_skus = self.filter_skus(&skus, resource_group);

        println!(
            "Filtered to {} SKUs for resource type {}",
            filtered_skus.len(),
            resource_group
        );

        if filtered_skus.is_empty() {
            return Err(AppError::ExternalService(format!(
                "No SKUs found for {} resource",
                resource_type
            )));
        }

        // Calculate price based on the first matching SKU
        let sku = filtered_skus
            .first()
            .ok_or_else(|| AppError::ExternalService("No SKU found".to_string()))?;

        Ok(self.calculate_sku_price(sku, amount as f64))
    }
}

#[async_trait]
impl PriceFetcher for GcpPriceFetcher {
    async fn fetch_price(
        &self,
        cpu_cores: i32,
        ram_gb: i32,
        storage_gb: i32,
    ) -> Result<f64, AppError> {
        // Validate inputs first
        if cpu_cores < 0 {
            return Err(AppError::InvalidInput(
                "CPU cores cannot be negative".to_string(),
            ));
        }
        if ram_gb < 0 {
            return Err(AppError::InvalidInput("RAM cannot be negative".to_string()));
        }
        if storage_gb < 0 {
            return Err(AppError::InvalidInput(
                "Storage cannot be negative".to_string(),
            ));
        }

        let mut total_price = 0.0;

        // Calculate prices for non-zero values
        if cpu_cores > 0 {
            total_price += self.calculate_resource_price("compute", cpu_cores).await?;
        }
        if ram_gb > 0 {
            total_price += self.calculate_resource_price("ram", ram_gb).await?;
        }
        if storage_gb > 0 {
            total_price += self.calculate_resource_price("storage", storage_gb).await?;
        }

        // Only try to fetch network price if we're calculating a full price
        if cpu_cores > 0 && ram_gb > 0 && storage_gb > 0 {
            match self.calculate_resource_price("network", 1).await {
                Ok(price) => total_price += price,
                Err(AppError::ExternalService(_)) => {
                    println!("Warning: Could not fetch network price, continuing without it");
                }
                Err(e) => return Err(e),
            }
        }

        Ok(total_price)
    }
}

#[async_trait]
impl PriceFetcher for AwsPriceFetcher {
    async fn fetch_price(
        &self,
        _cpu_cores: i32,
        _ram_gb: i32,
        _storage_gb: i32,
    ) -> Result<f64, AppError> {
        // TODO: Implement AWS price fetching
        Ok(0.0)
    }
}
