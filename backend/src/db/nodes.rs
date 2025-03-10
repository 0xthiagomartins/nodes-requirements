use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{CreateNodeRequest, Node, UpdateNodeRequest}; // Import from models

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
            updated_at as "updated_at?: DateTime<Utc>"
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
    let existing = sqlx::query_as!(
        Node,
        r#"
        SELECT 
            id, blockchain_type,
            cpu_cores as "cpu_cores: i32",
            ram_gb as "ram_gb: i32",
            storage_gb as "storage_gb: i32",
            network_mbps as "network_mbps: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at?: DateTime<Utc>"
        FROM nodes 
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    let existing = existing.ok_or_else(|| AppError::NotFound("Node not found".to_string()))?;

    // If no fields to update, return existing node
    if update.blockchain_type.is_none()
        && update.cpu_cores.is_none()
        && update.ram_gb.is_none()
        && update.storage_gb.is_none()
        && update.network_mbps.is_none()
    {
        return Ok(existing);
    }

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

    // Store the reference to blockchain_type
    let blockchain_type_ref = update.blockchain_type.as_ref();

    // Update node with COALESCE to handle NULL values
    let updated = sqlx::query!(
        r#"
        UPDATE nodes SET 
            blockchain_type = COALESCE(?1, blockchain_type),
            cpu_cores = COALESCE(?2, cpu_cores),
            ram_gb = COALESCE(?3, ram_gb),
            storage_gb = COALESCE(?4, storage_gb),
            network_mbps = COALESCE(?5, network_mbps),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?6
        RETURNING 
            id as "id!: i64",
            blockchain_type as "blockchain_type!",
            cpu_cores as "cpu_cores!: i32",
            ram_gb as "ram_gb!: i32",
            storage_gb as "storage_gb!: i32",
            network_mbps as "network_mbps!: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at?: DateTime<Utc>"
        "#,
        blockchain_type_ref,
        update.cpu_cores,
        update.ram_gb,
        update.storage_gb,
        update.network_mbps,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(Node {
        id: updated.id,
        blockchain_type: updated.blockchain_type,
        cpu_cores: updated.cpu_cores,
        ram_gb: updated.ram_gb,
        storage_gb: updated.storage_gb,
        network_mbps: updated.network_mbps,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    })
}

pub async fn delete_node(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM nodes WHERE id = ?", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Node not found".to_string()));
    }

    Ok(())
}

pub async fn get_all_nodes(pool: &SqlitePool) -> Result<Vec<Node>, AppError> {
    let nodes = sqlx::query_as!(
        Node,
        r#"
        SELECT 
            id as "id!: i64",
            blockchain_type as "blockchain_type!: String",
            cpu_cores as "cpu_cores!: i32",
            ram_gb as "ram_gb!: i32",
            storage_gb as "storage_gb!: i32",
            network_mbps as "network_mbps!: i32",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at?: DateTime<Utc>"
        FROM nodes
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(nodes)
}
