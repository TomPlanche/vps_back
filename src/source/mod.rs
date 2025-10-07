//! Source module
//!
//! This module provides functionality for tracking sources (analytics/referrers).
//! It includes:
//! - Data models for requests and responses
//! - HTTP handlers for incrementing and retrieving source counters
//! - Database operations for source management

pub mod handlers;
pub mod models;

use handlers::{get_all_sources, increment_source};

use axum::{Router, routing::get, routing::post};
use sea_orm::DatabaseConnection;

/// Creates the source router with all endpoints
pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(get_all_sources))
        .route("/", post(increment_source))
}
