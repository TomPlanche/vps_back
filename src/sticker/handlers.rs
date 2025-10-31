//! Sticker route handlers
//!
//! This module contains all HTTP handlers for sticker-related endpoints:
//! - GET /stickers - Fetch all stickers
//! - GET /stickers/:id - Fetch a single sticker by ID
//! - POST /stickers - Create a new sticker

use anyhow::Context;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect, Set,
};
use serde_json::json;
use tracing::info;

use super::models::{StickerRequest, StickerResponse};
use crate::{
    data_response, data_response_with_metadata,
    entities::{prelude::*, stickers},
    error::{ApiError, ApiResult},
    pagination::PaginationParams,
    response::Metadata,
};

/// Handles GET requests to fetch all stickers.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Query(params)` - Pagination parameters (page, limit).
///
/// # Returns
/// * `ApiResult<Json<Value>>` - JSON response containing all stickers ordered by creation date (newest first) with pagination metadata.
///
/// # Errors
/// Returns an error if the database query fails.
pub async fn get_all_stickers(
    State(db): State<DatabaseConnection>,
    Query(mut params): Query<PaginationParams>,
) -> ApiResult<Json<serde_json::Value>> {
    info!(
        "GET `/stickers` endpoint called with page={}, limit={}",
        params.page, params.limit
    );

    // Validate pagination parameters
    params.validate();

    // Create base query
    let query = Stickers::find().order_by_desc(stickers::Column::CreatedAt);

    // Count total items
    #[allow(clippy::cast_possible_truncation)]
    let total_count = query
        .clone()
        .count(&db)
        .await
        .context("Failed to count stickers")? as u32;

    // Fetch paginated results
    let stickers_list = query
        .offset(params.offset())
        .limit(params.limit_u64())
        .all(&db)
        .await
        .context("Failed to fetch stickers from database")?;

    let stickers: Result<Vec<StickerResponse>, anyhow::Error> = stickers_list
        .into_iter()
        .map(|model| {
            let pictures: Vec<String> =
                serde_json::from_value(model.pictures).context("Failed to parse pictures JSON")?;

            Ok(StickerResponse {
                id: i64::from(model.id),
                name: model.name,
                latitude: model.latitude,
                longitude: model.longitude,
                place_name: model.place_name,
                pictures,
                created_at: model.created_at.to_string(),
                updated_at: model.updated_at.to_string(),
            })
        })
        .collect();

    let stickers = stickers?;

    // Build metadata
    let metadata = Metadata::paginated(
        params.page,
        params.limit,
        total_count,
        "/secure/stickers".to_string(),
    );

    Ok(data_response_with_metadata(
        json!({
            "stickers": stickers
        }),
        &metadata,
    ))
}

/// Handles GET requests to fetch a single sticker by ID.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Path(id)` - The ID of the sticker to fetch.
///
/// # Returns
/// * `ApiResult<Json<Value>>` - JSON response containing the sticker.
///
/// # Errors
/// Returns an error if the database query fails or the sticker is not found.
pub async fn get_sticker(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    info!("GET `/stickers/{}` endpoint called", id);

    let model = Stickers::find_by_id(id)
        .one(&db)
        .await
        .with_context(|| format!("Failed to fetch sticker with id {id}"))?
        .ok_or_else(|| ApiError::not_found(format!("Sticker with id {id} not found")))?;

    let pictures: Vec<String> =
        serde_json::from_value(model.pictures).context("Failed to parse pictures JSON")?;

    let sticker = StickerResponse {
        id: i64::from(model.id),
        name: model.name,
        latitude: model.latitude,
        longitude: model.longitude,
        place_name: model.place_name,
        pictures,
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    };

    Ok(data_response(json!({
        "sticker": sticker
    })))
}

/// Handles POST requests to create a new sticker.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Json(payload)` - The request payload containing sticker data.
///
/// # Returns
/// * `ApiResult<Json<Value>>` - JSON response containing the created sticker.
///
/// # Errors
/// Returns an error if the database operation fails or JSON serialization fails.
pub async fn create_sticker(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<StickerRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    info!("POST `/stickers` endpoint called for: {}", payload.name);

    let pictures_json =
        serde_json::to_value(&payload.pictures).context("Failed to serialize pictures to JSON")?;

    let new_sticker = stickers::ActiveModel {
        name: Set(payload.name),
        latitude: Set(payload.latitude),
        longitude: Set(payload.longitude),
        place_name: Set(payload.place_name),
        pictures: Set(pictures_json),
        ..Default::default()
    };

    let model = new_sticker
        .insert(&db)
        .await
        .context("Failed to insert new sticker into database")?;

    let pictures: Vec<String> = serde_json::from_value(model.pictures)
        .context("Failed to parse pictures JSON from created sticker")?;

    let sticker = StickerResponse {
        id: i64::from(model.id),
        name: model.name,
        latitude: model.latitude,
        longitude: model.longitude,
        place_name: model.place_name,
        pictures,
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    };

    Ok(data_response(json!({
        "sticker": sticker
    })))
}
