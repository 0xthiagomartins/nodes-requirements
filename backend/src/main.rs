use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::fs;

mod db;
mod error;
mod models;
mod routes;

pub use crate::error::AppError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    // Ensure database directory exists
    fs::create_dir_all("db").expect("Failed to create database directory");
    
    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:db/app.db".to_string());
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    println!("Starting server at http://localhost:8080");

    // Start server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::nodes::config)
            .configure(routes::price_history::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
