use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{ApiKey, CreateApiKeyRequest, CreateApiKeyResponse};

pub async fn create_api_key(
    pool: &SqlitePool,
    request: CreateApiKeyRequest,
) -> Result<CreateApiKeyResponse, AppError> {
    // Generate a unique API key using UUID
    let key = Uuid::new_v4().to_string();
    let requests_per_minute = request.requests_per_minute.unwrap_or(60);

    let api_key = sqlx::query_as!(
        ApiKey,
        r#"
        INSERT INTO api_keys (key, name, requests_per_minute)
        VALUES (?, ?, ?)
        RETURNING 
            id as "id!: i64",
            key as "key!: String",
            name as "name!: String",
            created_at as "created_at!: DateTime<Utc>",
            last_used_at as "last_used_at?: DateTime<Utc>",
            is_active as "is_active!: bool",
            deleted_at as "deleted_at?: DateTime<Utc>",
            requests_per_minute as "requests_per_minute!: i32",
            requests_this_minute as "requests_this_minute!: i32",
            last_request_time as "last_request_time?: DateTime<Utc>"
        "#,
        key,
        request.name,
        requests_per_minute
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(CreateApiKeyResponse {
        id: api_key.id,
        key: api_key.key,
        name: api_key.name,
        requests_per_minute: api_key.requests_per_minute,
    })
}

pub async fn validate_api_key(pool: &SqlitePool, key: &str) -> Result<bool, AppError> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) 
        FROM api_keys 
        WHERE key = ? 
        AND is_active = TRUE 
        AND deleted_at IS NULL
        "#,
        key
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(count > 0)
}

pub async fn update_last_used(pool: &SqlitePool, key: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE api_keys
        SET last_used_at = CURRENT_TIMESTAMP
        WHERE key = ?
        "#,
        key
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

pub async fn check_and_update_rate_limit(pool: &SqlitePool, key: &str) -> Result<bool, AppError> {
    // Start a transaction since we need to check and update atomically
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    // Get current API key status
    let api_key = sqlx::query_as!(
        ApiKey,
        r#"
        SELECT 
            id as "id!: i64",
            key as "key!: String",
            name as "name!: String",
            created_at as "created_at!: DateTime<Utc>",
            last_used_at as "last_used_at?: DateTime<Utc>",
            is_active as "is_active!: bool",
            deleted_at as "deleted_at?: DateTime<Utc>",
            requests_per_minute as "requests_per_minute!: i32",
            requests_this_minute as "requests_this_minute!: i32",
            last_request_time as "last_request_time?: DateTime<Utc>"
        FROM api_keys 
        WHERE key = ? 
        AND is_active = TRUE 
        AND deleted_at IS NULL
        "#,
        key
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    let Some(api_key) = api_key else {
        return Ok(false);
    };

    let now = Utc::now();
    let should_reset = match api_key.last_request_time {
        Some(last_time) => now.signed_duration_since(last_time).num_minutes() >= 1,
        None => true,
    };

    // Reset counter if it's been more than a minute
    if should_reset {
        sqlx::query!(
            r#"
            UPDATE api_keys 
            SET requests_this_minute = 1,
                last_request_time = ?
            WHERE key = ?
            "#,
            now,
            key
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;
        return Ok(true);
    }

    // Check if we're under the limit
    if api_key.requests_this_minute >= api_key.requests_per_minute {
        return Ok(false);
    }

    // Increment the counter
    sqlx::query!(
        r#"
        UPDATE api_keys 
        SET requests_this_minute = requests_this_minute + 1,
            last_request_time = ?
        WHERE key = ?
        "#,
        now,
        key
    )
    .execute(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;
    Ok(true)
}
