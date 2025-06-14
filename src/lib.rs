use axum::{
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod db;
pub mod source;

/// Represents a standardized API response
#[derive(Debug)]
pub struct ApiResponse {
    pub status: StatusCode,
    pub json: Value,
}

#[derive(Debug, Deserialize)]
pub struct SourceRequest {
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct SourceResponse {
    pub source: String,
    pub count: i64,
}

impl ApiResponse {
    /// Creates a new `ApiResponse` with the given status and JSON value
    pub fn base(status: StatusCode, json: &Value) -> Json<Value> {
        Json(json!({
            "status": status.as_u16(),
            "success": status.is_success(),
            "data": json
        }))
    }

    /// Creates a success response with optional data
    pub fn success(data: impl Into<Value>) -> Json<Value> {
        Self::base(StatusCode::OK, &data.into())
    }

    /// Creates a created response with optional data
    pub fn created(data: impl Into<Value>) -> Json<Value> {
        Self::base(StatusCode::CREATED, &data.into())
    }

    /// Creates an error response with a message
    pub fn error(status: StatusCode, message: &str) -> Json<Value> {
        Json(json!({
            "status": status.as_u16(),
            "success": false,
            "error": {
                "message": message
            }
        }))
    }

    /// Creates a bad request error response
    pub fn bad_request(message: &str) -> Json<Value> {
        Self::error(StatusCode::BAD_REQUEST, message)
    }

    /// Creates a not found error response
    pub fn not_found(message: &str) -> Json<Value> {
        Self::error(StatusCode::NOT_FOUND, message)
    }

    /// Creates an internal server error response
    pub fn internal_error(message: &str) -> Json<Value> {
        Self::error(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    /// Creates an unauthorized error response
    pub fn unauthorized(message: &str) -> Json<Value> {
        Self::error(StatusCode::UNAUTHORIZED, message)
    }
}
