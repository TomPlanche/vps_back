mod handlers;

use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/github", get(handlers::get_github_stats))
        .route("/brew", get(handlers::get_brew_stats))
        .route("/sources", get(handlers::get_source_stats))
}
