//! # Authentication Middleware
//!
//! This module provides API key authentication middleware for protecting routes.
//! It validates the `x-api-key` header against the configured API key.
//!
//! ## Usage
//!
//! ```no_run
//! use axum::{Router, middleware};
//! use vps_back::middlewares::auth::{AppState, validate_api_key};
//! use std::sync::Arc;
//!
//! let app_state = AppState {
//!     api_key: Arc::new("your-api-key".to_string()),
//! };
//!
//! let app = Router::new()
//!     .layer(middleware::from_fn_with_state(app_state.clone(), validate_api_key));
//! ```

use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

/// Application state containing the API key.
#[derive(Clone)]
pub struct AppState {
    pub api_key: Arc<String>,
}

/// Middleware to validate API key from the `x-api-key` header.
///
/// # Arguments
/// * `State(state)` - The application state containing the expected API key.
/// * `request` - The incoming HTTP request.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
/// * `Response` - Either the next middleware/handler response or an unauthorized error.
pub async fn validate_api_key(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    let expected_api_key = &*state.api_key;

    match api_key {
        Some(key) if key == expected_api_key => next.run(request).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": 401,
                "success": false,
                "error": {
                    "message": "Invalid API key"
                }
            })),
        )
            .into_response(),
    }
}
