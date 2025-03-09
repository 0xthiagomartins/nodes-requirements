use std::future::{ready, Ready};

use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;
use sqlx::SqlitePool;

pub struct ApiKeyMiddleware;

impl ApiKeyMiddleware {
    pub fn new() -> Self {
        ApiKeyMiddleware
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Transform = ApiKeyMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddlewareService { service }))
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = req
            .app_data::<actix_web::web::Data<SqlitePool>>()
            .unwrap()
            .clone();

        // Skip auth for API key management endpoints
        if req.path().starts_with("/api-keys") {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_right_body())
            });
        }

        // Get API key from header
        let api_key = match req.headers().get("X-API-Key") {
            Some(key) => match key.to_str() {
                Ok(key) => key.to_string(),
                Err(_) => {
                    let (req, _) = req.into_parts();
                    return Box::pin(async move {
                        let res = HttpResponse::Unauthorized()
                            .json(json!({"error": "Invalid API key format"}));
                        Ok(ServiceResponse::new(req, res).map_into_left_body())
                    });
                }
            },
            None => {
                let (req, _) = req.into_parts();
                return Box::pin(async move {
                    let res =
                        HttpResponse::Unauthorized().json(json!({"error": "Missing API key"}));
                    Ok(ServiceResponse::new(req, res).map_into_left_body())
                });
            }
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            match crate::db::api_keys::check_and_update_rate_limit(&pool, &api_key).await {
                Ok(true) => {
                    let res = fut.await?;
                    Ok(res.map_into_right_body())
                }
                Ok(false) => {
                    let res = fut.await?;
                    let (req, _) = res.into_parts();
                    let res = HttpResponse::TooManyRequests()
                        .json(json!({"error": "Rate limit exceeded"}));
                    Ok(ServiceResponse::new(req, res).map_into_left_body())
                }
                Err(e) => {
                    let res = fut.await?;
                    let (req, _) = res.into_parts();
                    let res = HttpResponse::InternalServerError()
                        .json(json!({"error": format!("Database error: {}", e)}));
                    Ok(ServiceResponse::new(req, res).map_into_left_body())
                }
            }
        })
    }
}
