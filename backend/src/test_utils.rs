#[cfg(test)]
pub mod test_utils {
    use sqlx::SqlitePool;
    use crate::models::Node;

    pub async fn insert_test_node(pool: &SqlitePool) -> Node {
        sqlx::query_as::<_, Node>(
            "INSERT INTO nodes (blockchain_type, cpu_cores, ram_gb, storage_gb, network_mbps)
             VALUES ('ethereum', 8, 16, 500, 1000)
             RETURNING *"
        )
        .fetch_one(pool)
        .await
        .expect("Failed to insert test node")
    }
} 