//! Source data models and request/response types

use serde::{Deserialize, Serialize};

/// Request payload for incrementing a source counter
#[derive(Debug, Deserialize, Serialize)]
pub struct SourceRequest {
    pub source: String,
}

/// Response structure for source data
#[derive(Debug, Serialize)]
pub struct SourceResponse {
    pub id: i64,
    pub name: String,
    pub count: i32,
    pub created_at: String,
    pub updated_at: String,
}
