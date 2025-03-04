use sqlx::SqlitePool;
use chrono::{DateTime, Utc};

use crate::models::{Node, CreateNodeRequest, UpdateNodeRequest};  // Import from models
use crate::error::AppError;

pub async fn create_node(pool: &SqlitePool, node: CreateNodeRequest) -> Result<Node, AppError> {
    // Check for duplicate blockchain type
    let exists = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM nodes WHERE blockchain_type = ?",
        node.blockchain_type
    )
    .fetch_one(pool)
    .await?
    > 0;

    if exists {
        return Err(AppError::Conflict(format!(
            "Node with blockchain type '{}' already exists",
            node.blockchain_type
        )));
    }

    // Insert new node
    let node = sqlx::query_as!(
        Node,
        r#"
        INSERT INTO nodes (blockchain_type, cpu_cores, ram_gb, storage_gb, network_mbps)
        VALUES (?, ?, ?, ?, ?)
        RETURNING 
            id, blockchain_type,
            cpu_cores as "cpu_cores: i32",
            ram_gb as "ram_gb: i32",
            storage_gb as "storage_gb: i32",
            network_mbps as "network_mbps: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>"
        "#,
        node.blockchain_type,
        node.cpu_cores,
        node.ram_gb,
        node.storage_gb,
        node.network_mbps
    )
    .fetch_one(pool)
    .await?;

    Ok(node)
}

pub async fn update_node(
    pool: &SqlitePool,
    id: i64,
    update: UpdateNodeRequest,
) -> Result<Node, AppError> {
    // Check if node exists
    let _existing = sqlx::query_as!(
        Node,
        r#"
        SELECT 
            id, blockchain_type,
            cpu_cores as "cpu_cores: i32",
            ram_gb as "ram_gb: i32",
            storage_gb as "storage_gb: i32",
            network_mbps as "network_mbps: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>"
        FROM nodes 
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    // Check for duplicate blockchain type if it's being updated
    if let Some(blockchain_type) = &update.blockchain_type {
        let exists = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM nodes WHERE blockchain_type = ? AND id != ?",
            blockchain_type,
            id
        )
        .fetch_one(pool)
        .await?
        > 0;

        if exists {
            return Err(AppError::Conflict(format!(
                "Node with blockchain type '{}' already exists",
                blockchain_type
            )));
        }
    }

    // Build update query dynamically
    let mut query = String::from(
        r#"
        UPDATE nodes SET 
        "#
    );
    let mut params = Vec::new();
    
    if let Some(blockchain_type) = update.blockchain_type {
        query.push_str("blockchain_type = ?, ");
        params.push(blockchain_type.to_string());
    }
    if let Some(cpu_cores) = update.cpu_cores {
        query.push_str("cpu_cores = ?, ");
        params.push(cpu_cores.to_string());
    }
    if let Some(ram_gb) = update.ram_gb {
        query.push_str("ram_gb = ?, ");
        params.push(ram_gb.to_string());
    }
    if let Some(storage_gb) = update.storage_gb {
        query.push_str("storage_gb = ?, ");
        params.push(storage_gb.to_string());
    }
    if let Some(network_mbps) = update.network_mbps {
        query.push_str("network_mbps = ?, ");
        params.push(network_mbps.to_string());
    }

    query.push_str(
        r#"
        updated_at = CURRENT_TIMESTAMP 
        WHERE id = ? 
        RETURNING 
            id, blockchain_type,
            cpu_cores as "cpu_cores: i32",
            ram_gb as "ram_gb: i32",
            storage_gb as "storage_gb: i32",
            network_mbps as "network_mbps: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>"
        "#
    );
    params.push(id.to_string());

    let mut query = sqlx::query_as::<_, Node>(&query);
    for param in params {
        query = query.bind(param);
    }
    let node = query.fetch_one(pool).await?;

    Ok(node)
} 