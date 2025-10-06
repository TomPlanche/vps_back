use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use sqlx::postgres::PgPool;
use tracing::info;

use crate::{ApiResponse, StickerRequest, StickerResponse};

/// Handles GET requests to fetch all stickers.
///
/// # Arguments
/// * `State(pool)` - The database connection pool.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing all stickers.
pub async fn get_stickers(State(pool): State<PgPool>) -> Json<Value> {
    info!("GET `/stickers` endpoint called");

    match sqlx::query!(
        r#"
        SELECT id, name, latitude, longitude, place_name, pictures, created_at, updated_at
        FROM stickers
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => {
            let stickers: Vec<StickerResponse> = rows
                .into_iter()
                .map(|row| {
                    let pictures: Vec<String> = serde_json::from_value(row.pictures)
                        .unwrap_or_default();

                    StickerResponse {
                        id: i64::from(row.id),
                        name: row.name,
                        latitude: row.latitude,
                        longitude: row.longitude,
                        place_name: row.place_name,
                        pictures,
                        created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                        updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
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
/// * `State(pool)` - The database connection pool.
/// * `Path(id)` - The ID of the sticker to fetch.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the sticker or an error.
///
/// # Panics
/// May panic if the sqlx macro encounters unexpected type mismatches.
pub async fn get_sticker(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<Value> {
    info!("GET `/stickers/{}` endpoint called", id);

    match sqlx::query!(
        r#"
        SELECT id, name, latitude, longitude, place_name, pictures, created_at, updated_at
        FROM stickers
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(row)) => {
            let pictures: Vec<String> = serde_json::from_value(row.pictures)
                .unwrap_or_default();

            let sticker = StickerResponse {
                id: i64::from(row.id),
                name: row.name,
                latitude: row.latitude,
                longitude: row.longitude,
                place_name: row.place_name,
                pictures,
                created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
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
/// * `State(pool)` - The database connection pool.
/// * `Json(payload)` - The request payload containing sticker data.
///
/// # Returns
/// * `Json<Value>` - A JSON response containing the created sticker or an error.
///
/// # Panics
/// May panic if the sqlx macro encounters unexpected type mismatches.
pub async fn create_sticker(
    State(pool): State<PgPool>,
    Json(payload): Json<StickerRequest>,
) -> Json<Value> {
    info!("POST `/stickers` endpoint called for: {}", payload.name);

    let pictures_json = serde_json::to_value(&payload.pictures)
        .unwrap_or_else(|_| serde_json::json!([]));

    match sqlx::query!(
        r#"
        INSERT INTO stickers (name, latitude, longitude, place_name, pictures)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, latitude, longitude, place_name, pictures, created_at, updated_at
        "#,
        payload.name,
        payload.latitude,
        payload.longitude,
        payload.place_name,
        pictures_json
    )
    .fetch_one(&pool)
    .await
    {
        Ok(row) => {
            let pictures: Vec<String> = serde_json::from_value(row.pictures)
                .unwrap_or_default();

            let sticker = StickerResponse {
                id: i64::from(row.id),
                name: row.name,
                latitude: row.latitude,
                longitude: row.longitude,
                place_name: row.place_name,
                pictures,
                created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
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
