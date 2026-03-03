use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::brew::handlers::get_brew_stats;
use crate::github_stats::handlers::get_github_stats;
use crate::source::handlers::get_source_stats;

pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/github", get(get_github_stats))
        .route("/brew", get(get_brew_stats))
        .route("/sources", get(get_source_stats))
}
