use axum::{Extension, Json};
use serde_json::Value;
use tracing::info;

use crate::data_response;

/// Handles GET requests for GitHub stats.
///
/// Reads the pre-fetched stats JSON file written by the periodic background task.
///
/// # Returns
/// The contents of `github-stats.json`, or `null` if the file is missing or invalid.
pub async fn get_github_stats(Extension(static_dir): Extension<String>) -> Json<Value> {
    info!("GET `/stats/github` endpoint called");

    let path = format!("{static_dir}/github-stats.json");

    let data = tokio::fs::read_to_string(&path)
        .await
        .map_or(Value::Null, |contents| {
            serde_json::from_str(&contents).unwrap_or(Value::Null)
        });

    data_response(data)
}
