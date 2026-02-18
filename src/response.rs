//! API response structures
//!
//! This module defines the standardized response format for the API.
//! Responses follow the format:
//! ```json
//! {
//!   "_metadata": { ... },  // Optional pagination metadata
//!   "data": { ... }         // Response data
//! }
//! ```

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Links for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
}

/// Metadata for paginated responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(clippy::pub_underscore_fields)]
    pub _links: Option<Links>,
}

impl Metadata {
    /// Create minimal metadata for non-paginated responses
    #[must_use]
    pub const fn minimal() -> Self {
        Self {
            page: None,
            limit: None,
            page_count: None,
            total_pages: None,
            total_count: None,
            _links: None,
        }
    }

    /// Create metadata for paginated responses
    #[must_use]
    #[allow(dead_code)]
    pub fn paginated(page: u32, limit: u32, total_count: u32, self_link: String) -> Self {
        let total_pages = total_count.div_ceil(limit);
        let page_count = if page < total_pages {
            limit
        } else {
            total_count - (page - 1) * limit
        };

        let next = if page < total_pages {
            Some(format!("{}?page={}&limit={}", self_link, page + 1, limit))
        } else {
            None
        };

        let prev = if page > 1 {
            Some(format!("{}?page={}&limit={}", self_link, page - 1, limit))
        } else {
            None
        };

        Self {
            page: Some(page),
            limit: Some(limit),
            page_count: Some(page_count),
            total_pages: Some(total_pages),
            total_count: Some(total_count),
            _links: Some(Links {
                self_link,
                next,
                prev,
            }),
        }
    }
}

/// Standard API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(clippy::pub_underscore_fields)]
    pub _metadata: Option<Metadata>,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a simple response with data only
    pub const fn data(data: T) -> Json<Self> {
        Json(Self {
            _metadata: None,
            data,
        })
    }

    /// Create a response with metadata
    #[allow(dead_code)]
    pub const fn with_metadata(data: T, metadata: Metadata) -> Json<Self> {
        Json(Self {
            _metadata: Some(metadata),
            data,
        })
    }
}

/// Helper function to create a simple data response
pub fn data_response(data: impl Serialize) -> Json<Value> {
    Json(json!({
        "data": data
    }))
}

/// Helper function to create a response with metadata
#[allow(dead_code)]
pub fn data_response_with_metadata(data: impl Serialize, metadata: &Metadata) -> Json<Value> {
    Json(json!({
        "_metadata": metadata,
        "data": data
    }))
}
