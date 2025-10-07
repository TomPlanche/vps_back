//! # Tracing Middleware
//!
//! This module provides HTTP request and response tracing middleware for observability
//! and debugging purposes. It logs incoming requests, outgoing responses, and any errors
//! that occur during request processing.
//!
//! ## Overview
//!
//! The tracing middleware provides comprehensive logging of HTTP interactions:
//! - Request logging with method, URI, and version
//! - Response logging with status codes and latency
//! - Error logging for failed requests
//! - Structured logging using the tracing framework
//!
//! ## Configuration
//!
//! Tracing behavior is controlled by the `RUST_LOG` environment variable:
//! - `RUST_LOG=info` - Standard request/response logging
//! - `RUST_LOG=debug` - Detailed debugging information
//! - `RUST_LOG=error` - Only error logging
//!
//! ## Usage
//!
//! ```rust,no_run
//! use axum::Router;
//! use vps_back::middlewares::tracing::create_tracing_layer;
//!
//! let tracing_layer = create_tracing_layer();
//! let app = Router::new()
//!     .layer(tracing_layer);
//! ```

use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;

/// Creates a tracing layer for HTTP request/response logging.
///
/// This function configures a comprehensive tracing layer that logs:
/// - Incoming HTTP requests with method and URI
/// - Outgoing HTTP responses with status codes and latency
/// - Failed requests with error details
///
/// # Configuration
///
/// The tracing layer is configured with the following settings:
/// - **Request logging**: INFO level, no headers included for privacy
/// - **Response logging**: INFO level, no headers included for privacy
/// - **Error logging**: ERROR level for failed requests
/// - **Span creation**: INFO level with request details
///
/// # Returns
///
/// A configured `TraceLayer` ready to be applied to an Axum router.
///
/// # Examples
///
/// ```rust,no_run
/// use axum::Router;
/// use vps_back::middlewares::tracing::create_tracing_layer;
///
/// let app = Router::new()
///     .layer(create_tracing_layer());
/// ```
#[must_use]
pub fn create_tracing_layer()
-> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .include_headers(false),
        )
        .on_failure(DefaultOnFailure::new().level(Level::ERROR))
}
