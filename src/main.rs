use axum::{
    Json, Router,
    http::{HeaderName, HeaderValue, Method},
    routing::{get, post},
};
use serde_json::{Value, json};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vps_back::{ApiResponse, db, source};

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    assert!(
        std::env::var("API_KEY").is_ok(),
        "API_KEY must be set in the environment variables"
    );

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration from environment
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    // Initialize database
    let pool = db::init_pool()
        .await
        .expect("Failed to initialize database");

    // Configure CORS
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
        .split(',')
        .map(|origin| origin.trim().parse::<HeaderValue>().unwrap())
        .collect::<Vec<_>>();

    let mut cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            HeaderName::from_static("x-api-key"),
        ])
        .allow_credentials(true);

    // Add each allowed origin to the CORS configuration
    for origin in allowed_origins {
        cors = cors.allow_origin(origin);
    }

    // Build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/source", get(source::get_sources))
        .route("/source", post(source::increment_source))
        .nest_service("/static", ServeDir::new("static"))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Run it
    let addr = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Handles GET requests to the root path ("/").
/// Serves as a simple health check endpoint.
async fn root() -> Json<Value> {
    tracing::info!("GET `/` endpoint called");

    ApiResponse::success(json!({
        "message": "Hello, I'm Tom Planche!"
    }))
}
