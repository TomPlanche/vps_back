use axum::{Json, extract::State};
use serde_json::{Value, json};
use sqlx::postgres::PgPool;
use tracing::info;

use crate::{ApiResponse, SourceRequest};

/// Handles GET requests to the sources path ("/sources").
/// Fetches all sources and their counts from the database.
///
/// # Arguments
/// * `State(pool)` - The database connection pool.
///
/// # Panics
/// * If the `as_object_mut` method fails, which should not happen in normal operation.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the sources and their counts.
pub async fn get_sources(State(pool): State<PgPool>) -> Json<Value> {
    info!("GET `/sources` endpoint called");

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
/// * `Json(payload)` - The request payload containing the source name.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the updated source count or an error message.
///
/// # Panics
/// May panic if the sqlx macro encounters unexpected type mismatches.
pub async fn increment_source(
    State(pool): State<PgPool>,
    Json(payload): Json<SourceRequest>,
) -> Json<Value> {
    info!("POST `/source` endpoint called for: {}", payload.source);

    // Increment the source counter
    match crate::db::increment_source(&pool, &payload.source).await {
        Ok(()) => {
            // Get the current count
            let count =
                sqlx::query_scalar!("SELECT sources.count FROM sources WHERE name = $1", payload.source)
                    .fetch_one(&pool)
                    .await
                    .unwrap_or(-1);

            ApiResponse::success(json!({
                payload.source: count
            }))
        }
        Err(e) => {
            info!("Database error: {}", e);
            ApiResponse::internal_error("Failed to update source counter")
        }
    }
}
