use sqlx::SqlitePool;
use crate::models::{Node, CreateNodeRequest, UpdateNodeRequest};
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
        RETURNING *
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
    let node = sqlx::query_as!(Node, "SELECT * FROM nodes WHERE id = ?", id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Node with id {} not found", id)))?;

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
    let mut query = String::from("UPDATE nodes SET ");
    let mut params = Vec::new();
    
    if let Some(blockchain_type) = update.blockchain_type {
        query.push_str("blockchain_type = ?, ");
        params.push(blockchain_type);
    }
    if let Some(cpu_cores) = update.cpu_cores {
        query.push_str("cpu_cores = ?, ");
        params.push(cpu_cores.to_string());
    }
    // ... similar for other fields ...

    query.push_str("updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING *");
    params.push(id.to_string());

    let node = sqlx::query_as::<_, Node>(&query)
        .bind_all(params)
        .fetch_one(pool)
        .await?;

    Ok(node)
} 