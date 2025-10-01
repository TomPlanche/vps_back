use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_key: String,
    pub allowed_origins: Vec<String>,
    pub rust_log: String,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Returns
    /// A `Result` containing the `Config` if all required environment variables are set.
    ///
    /// # Errors
    /// Returns an error if any required environment variable is missing or invalid.
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid number")?;

        let api_key =
            env::var("API_KEY").map_err(|_| "API_KEY must be set in environment variables")?;

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            host,
            port,
            api_key,
            allowed_origins,
            rust_log,
        })
    }
}
