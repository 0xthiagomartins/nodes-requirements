pub mod db;
pub mod error;
pub mod models;
pub mod routes;

// Re-export AppError for convenience
pub use crate::error::AppError;
pub use crate::models::{CreateNodeRequest, Node, UpdateNodeRequest};
