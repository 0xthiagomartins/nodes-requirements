use sqlx::SqlitePool;

pub mod api_keys;
pub mod nodes;
pub mod price_history;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(database_url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_pool() {
        let pool = create_pool("sqlite::memory:").await;
        assert!(pool.is_ok());
    }
}
