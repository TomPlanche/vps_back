//! Sticker route handlers
//!
//! This module contains all HTTP handlers for sticker-related endpoints:
//! - GET /stickers - Fetch all stickers
//! - GET /stickers/:id - Fetch a single sticker by ID
//! - POST /stickers - Create a new sticker

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};
use serde_json::{Value, json};
use tracing::info;

use super::models::{StickerRequest, StickerResponse};
use crate::{
    ApiResponse,
    entities::{prelude::*, stickers},
};

/// Handles GET requests to fetch all stickers.
///
/// # Arguments
/// * `State(db)` - The database connection.
///
/// # Returns
/// * `(StatusCode, Json<Value>)` - A tuple with HTTP status code and JSON response containing all stickers ordered by creation date (newest first).
pub async fn get_all_stickers(State(db): State<DatabaseConnection>) -> (StatusCode, Json<Value>) {
    info!("GET `/stickers` endpoint called");

    match Stickers::find()
        .order_by_desc(stickers::Column::CreatedAt)
        .all(&db)
        .await
    {
        Ok(stickers_list) => {
            let stickers: Vec<StickerResponse> = stickers_list
                .into_iter()
                .map(|model| {
                    let pictures: Vec<String> =
                        serde_json::from_value(model.pictures).unwrap_or_default();

                    StickerResponse {
                        id: i64::from(model.id),
                        name: model.name,
                        latitude: model.latitude,
                        longitude: model.longitude,
                        place_name: model.place_name,
                        pictures,
                        created_at: model.created_at.to_string(),
                        updated_at: model.updated_at.to_string(),
                    }
                })
                .collect();

            ApiResponse::success(json!({
                "stickers": stickers
            }))
        }
        Err(e) => {
            info!("Database error: {}", e);
            ApiResponse::internal_error("Failed to fetch stickers")
        }
    }
}

/// Handles GET requests to fetch a single sticker by ID.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Path(id)` - The ID of the sticker to fetch.
///
/// # Returns
/// * `(StatusCode, Json<Value>)` - A tuple with HTTP status code and JSON response containing the sticker or an error.
pub async fn get_sticker(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    info!("GET `/stickers/{}` endpoint called", id);

    match Stickers::find_by_id(id).one(&db).await {
        Ok(Some(model)) => {
            let pictures: Vec<String> = serde_json::from_value(model.pictures).unwrap_or_default();

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

            ApiResponse::success(json!({
                "sticker": sticker
            }))
        }
        Ok(None) => ApiResponse::not_found("Sticker not found"),
        Err(e) => {
            info!("Database error: {}", e);
            ApiResponse::internal_error("Failed to fetch sticker")
        }
    }
}

/// Handles POST requests to create a new sticker.
///
/// # Arguments
/// * `State(db)` - The database connection.
/// * `Json(payload)` - The request payload containing sticker data.
///
/// # Returns
/// * `(StatusCode, Json<Value>)` - A tuple with HTTP status code and JSON response containing the created sticker or an error.
pub async fn create_sticker(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<StickerRequest>,
) -> (StatusCode, Json<Value>) {
    info!("POST `/stickers` endpoint called for: {}", payload.name);

    let pictures_json =
        serde_json::to_value(&payload.pictures).unwrap_or_else(|_| serde_json::json!([]));

    let new_sticker = stickers::ActiveModel {
        name: Set(payload.name),
        latitude: Set(payload.latitude),
        longitude: Set(payload.longitude),
        place_name: Set(payload.place_name),
        pictures: Set(pictures_json),
        ..Default::default()
    };

    match new_sticker.insert(&db).await {
        Ok(model) => {
            let pictures: Vec<String> = serde_json::from_value(model.pictures).unwrap_or_default();

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

            ApiResponse::created(json!({
                "sticker": sticker
            }))
        }
        Err(e) => {
            info!("Database error: {}", e);
            ApiResponse::internal_error("Failed to create sticker")
        }
    }
}
