pub mod nodes;

#[cfg(test)]
pub async fn create_pool(database_url: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    sqlx::SqlitePool::connect(database_url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_pool() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        assert!(pool.acquire().await.is_ok());
    }
} 