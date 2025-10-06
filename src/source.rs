use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde_json::{Value, json};
use tracing::info;

use crate::{ApiResponse, SourceRequest, entities::{prelude::*, sources}};

/// Handles GET requests to the sources path ("/sources").
/// Fetches all sources and their counts from the database.
///
/// # Arguments
/// * `State(db)` - The database connection.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the sources and their counts.
///
/// # Panics
/// * If the JSON object cannot be mutated, which should not happen in normal operation.
pub async fn get_sources(State(db): State<DatabaseConnection>) -> Json<Value> {
    info!("GET `/sources` endpoint called");

    match Sources::find()
        .order_by_asc(sources::Column::Name)
        .all(&db)
        .await
    {
        Ok(sources_list) => {
            // Convert rows into a map of name to count
            let sources_map = sources_list.into_iter().fold(json!({}), |mut acc, model| {
                acc.as_object_mut()
                    .unwrap()
                    .insert(model.name, json!(model.count));

                acc
            });

            ApiResponse::success(json!({
                "sources": sources_map
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
/// * `State(db)` - The database connection.
/// * `Json(payload)` - The request payload containing the source name.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the updated source count or an error message.
pub async fn increment_source(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<SourceRequest>,
) -> Json<Value> {
    info!("POST `/source` endpoint called for: {}", payload.source);

    // Increment the source counter
    match crate::db::increment_source(&db, &payload.source).await {
        Ok(()) => {
            // Get the current count
            let count = Sources::find()
                .filter(sources::Column::Name.eq(&payload.source))
                .one(&db)
                .await
                .ok()
                .flatten()
                .map_or(-1, |model| model.count);

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
