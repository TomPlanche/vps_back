use axum::{Json, extract::State};
use axum_extra::TypedHeader;
use headers::Authorization;
use serde_json::{Value, json};
use sqlx::sqlite::SqlitePool;
use tracing::info;

use crate::{ApiResponse, SourceRequest};

/// Handles GET requests to the sources path ("/sources").
/// Fetches all sources and their counts from the database.
///
/// # Arguments
/// * `State(pool)` - The database connection pool.
/// * `TypedHeader(api_key)` - The API key for authorization.
///
/// # Panics
/// * If the `as_object_mut` method fails, which should not happen in normal operation.
/// * If the `unwrap` on the environment variable fails, which should not happen if the API key is set correctly.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the sources and their counts.
pub async fn get_sources(
    State(pool): State<SqlitePool>,
    TypedHeader(api_key): TypedHeader<Authorization<headers::authorization::Bearer>>,
) -> Json<Value> {
    info!("GET `/sources` endpoint called");

    if api_key.token() != std::env::var("API_KEY").unwrap() {
        return ApiResponse::unauthorized("Invalid API key");
    }

    match sqlx::query!(
        r#"
        SELECT name, count
        FROM sources
        ORDER BY name
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => {
            // Convert rows into a map of name to count
            let sources = rows.into_iter().fold(json!({}), |mut acc, row| {
                acc.as_object_mut()
                    .unwrap()
                    .insert(row.name, json!(row.count));

                acc
            });

            ApiResponse::success(json!({
                "sources": sources
            }))
        }
        Err(e) => {
            info!("Database error: {}", e);
            ApiResponse::internal_error("Failed to fetch sources")
        }
    }
}

/// Handles POST requests to the source path ("/source").
/// Increments the count for a given source in the database.
///
/// # Arguments
/// * `State(pool)` - The database connection pool.
/// * `TypedHeader(api_key)` - The API key for authorization.
/// * `Json(payload)` - The request payload containing the source name.
///
/// # Panics
/// * If the `unwrap` on the environment variable fails, which should not happen if the API key is set correctly.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the updated source count or an error message.
pub async fn increment_source(
    State(pool): State<SqlitePool>,
    TypedHeader(api_key): TypedHeader<Authorization<headers::authorization::Bearer>>,
    Json(payload): Json<SourceRequest>,
) -> Json<Value> {
    // Verify API key
    let expected_api_key = std::env::var("API_KEY").unwrap(); // We already checked this in the main.rs

    if api_key.token() != expected_api_key {
        return ApiResponse::unauthorized("Invalid API key");
    }

    info!("POST `/source` endpoint called for: {}", payload.source);

    // Increment the source counter
    match crate::db::increment_source(&pool, &payload.source).await {
        Ok(()) => {
            // Get the current count
            let count =
                sqlx::query_scalar!("SELECT count FROM sources WHERE name = ?", payload.source)
                    .fetch_one(&pool)
                    .await
                    .unwrap_or(0);

            ApiResponse::success(json!({
                "source": payload.source,
                "count": count
            }))
        }
        Err(e) => {
            info!("Database error: {}", e);
            ApiResponse::internal_error("Failed to update source counter")
        }
    }
}
