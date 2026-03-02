//! Static file serving module
//!
//! This module provides functionality for serving static files,
//! such as `LastFM` fetcher output.

use tower_http::services::ServeDir;

/// Creates a `ServeDir` service for serving static files.
///
/// # Parameters
/// * `dir` - The directory path to serve files from.
///
/// # Returns
/// * `ServeDir` - A service configured to serve files from the given directory.
#[must_use]
pub fn static_files_service(dir: &str) -> ServeDir {
    ServeDir::new(dir)
}
