use actix_web::{
    dev::{forward_ready, Service, ServiceFactory, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{LocalBoxFuture, Ready};
use sqlx::SqlitePool;

pub struct ApiKeyMiddleware {
    pool: sqlx::SqlitePool,
}

impl ApiKeyMiddleware {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<B>,
            Error = Error,
            InitError = (),
        > + 'static,
    S::Service: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddlewareService<S::Service>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let pool = self.pool.clone();
        Box::pin(async move {
            let service = service.new_service(()).await?;
            Ok(ApiKeyMiddlewareService { service, pool })
        })
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: S,
    pool: SqlitePool,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = self.pool.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let api_key = req
                .headers()
                .get("X-API-Key")
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing API key"))?;

            // Check rate limit
            let rate_limit_ok = sqlx::query_scalar!(
                r#"
                SELECT 
                    CASE 
                        WHEN last_request_time IS NULL 
                            OR strftime('%s', 'now') - strftime('%s', last_request_time) >= 60 
                        THEN 1
                        WHEN requests_this_minute < requests_per_minute THEN 1
                        ELSE 0
                    END
                FROM api_keys 
                WHERE key = ? AND is_active = TRUE AND deleted_at IS NULL
                "#,
                api_key
            )
            .fetch_one(&pool)
            .await
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid API key"))?;

            if rate_limit_ok != 1 {
                return Err(actix_web::error::ErrorTooManyRequests(
                    "Rate limit exceeded",
                ));
            }

            // Update rate limit counters
            sqlx::query!(
                r#"
                UPDATE api_keys 
                SET 
                    requests_this_minute = CASE 
                        WHEN last_request_time IS NULL 
                            OR strftime('%s', 'now') - strftime('%s', last_request_time) >= 60 
                        THEN 1
                        ELSE requests_this_minute + 1
                    END,
                    last_request_time = CURRENT_TIMESTAMP
                WHERE key = ?
                "#,
                api_key
            )
            .execute(&pool)
            .await
            .map_err(|_| {
                actix_web::error::ErrorInternalServerError("Failed to update rate limit")
            })?;

            // Call the service after validation
            service.call(req).await
        })
    }
}
