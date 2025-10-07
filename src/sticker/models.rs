//! Sticker data models and request/response types

use serde::{Deserialize, Serialize};

/// Request payload for creating a new sticker
#[derive(Debug, Deserialize, Serialize)]
pub struct StickerRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place_name: String,
    #[serde(default)]
    pub pictures: Vec<String>,
}

/// Response structure for sticker data
#[derive(Debug, Serialize)]
pub struct StickerResponse {
    pub id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place_name: String,
    pub pictures: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
