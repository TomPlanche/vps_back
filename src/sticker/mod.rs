//! Sticker module
//!
//! This module provides functionality for managing location-based stickers.
//! It includes:
//! - Data models for requests and responses
//! - HTTP handlers for CRUD operations
//! - Database operations for sticker management

pub mod handlers;
pub mod models;

use handlers::{create_sticker, get_all_stickers, get_sticker};

use axum::{Router, routing::get, routing::post};
use sea_orm::DatabaseConnection;

/// Creates the sticker router with all endpoints
pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(get_all_stickers))
        .route("/", post(create_sticker))
        .route("/:id", get(get_sticker))
}
