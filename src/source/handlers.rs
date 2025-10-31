//! HTTP handlers for source endpoints

use anyhow::Context;
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use serde_json::json;
use tracing::info;

use crate::{
    data_response, data_response_with_metadata,
    entities::{prelude::*, sources},
    error::ApiResult,
    pagination::PaginationParams,
    response::Metadata,
    source::models::SourceRequest,
};

/// Handles GET requests to the sources path ("/sources").
/// Fetches all sources and their counts from the database with pagination.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Query(params)` - Pagination parameters (page, limit).
///
/// # Returns
/// * `ApiResult<Json<Value>>` - JSON response containing the sources and their counts with pagination metadata.
///
/// # Errors
/// Returns an error if the database query fails.
pub async fn get_all_sources(
    State(db): State<DatabaseConnection>,
    Query(mut params): Query<PaginationParams>,
) -> ApiResult<Json<serde_json::Value>> {
    info!(
        "GET `/sources` endpoint called with page={}, limit={}",
        params.page, params.limit
    );

    // Validate pagination parameters
    params.validate();

    // Create base query
    let query = Sources::find().order_by_asc(sources::Column::Name);

    // Count total items
    #[allow(clippy::cast_possible_truncation)]
    let total_count = query
        .clone()
        .count(&db)
        .await
        .context("Failed to count sources")? as u32;

    // Fetch paginated results
    let sources_list = query
        .offset(params.offset())
        .limit(params.limit_u64())
        .all(&db)
        .await
        .context("Failed to fetch sources from database")?;

    // Convert rows into a map of name to count
    let mut sources_map = serde_json::Map::new();
    for model in sources_list {
        sources_map.insert(model.name, json!(model.count));
    }

    // Build metadata
    let metadata = Metadata::paginated(
        params.page,
        params.limit,
        total_count,
        "/secure/source".to_string(),
    );

    Ok(data_response_with_metadata(
        json!({
            "sources": sources_map
        }),
        &metadata,
    ))
}

/// Handles POST requests to the source path ("/source").
/// Increments the count for a given source in the database.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Json(payload)` - The request payload containing the source name.
///
/// # Returns
/// * `ApiResult<Json<Value>>` - JSON response containing the updated source count.
///
/// # Errors
/// Returns an error if the database operation fails.
pub async fn increment_source(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<SourceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    info!("POST `/source` endpoint called for: {}", payload.source);

    // Increment the source counter
    crate::db::increment_source_in_db(&db, &payload.source)
        .await
        .context("Failed to increment source counter")?;

    // Get the current count
    let model = Sources::find()
        .filter(sources::Column::Name.eq(&payload.source))
        .one(&db)
        .await
        .context("Failed to fetch updated source count")?
        .context("Source not found after increment")?;

    Ok(data_response(json!({
        payload.source: model.count
    })))
}
