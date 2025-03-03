use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;

mod routes;
mod models;
mod db;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool");

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
            .service(routes::nodes::config())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
