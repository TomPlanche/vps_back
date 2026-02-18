pub mod handlers;

use axum::{Router, routing::get};
use handlers::{get_brew_stats, track_brew_download};
use sea_orm::DatabaseConnection;

/// Creates the brew router with all public endpoints.
pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/track/:project/:filename", get(track_brew_download))
        .route("/stats", get(get_brew_stats))
}
