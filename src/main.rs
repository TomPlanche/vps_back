use axum::{
    Json, Router,
    http::{HeaderName, HeaderValue, Method},
    middleware,
    routing::get,
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vps_back::{
    ApiResponse, config::Config, db::init_pool, middlewares, source,
    static_files::static_files_service, sticker,
};

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    // Load configuration - will exit if required env vars are missing
    let config = Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    // Create application state
    let app_state = middlewares::auth::AppState {
        api_key: Arc::new(config.api_key.clone()),
    };

    // Initialize tracing with sqlx filtering
    let filter = tracing_subscriber::EnvFilter::new(&config.rust_log)
        .add_directive("sqlx::query=warn".parse().unwrap());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db = init_pool().await.expect("Failed to initialize database");

    // Configure CORS
    let allowed_origins = config
        .allowed_origins
        .iter()
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .expect("Invalid origin in ALLOWED_ORIGINS")
        })
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
    // Create API router with protected routes
    let api_router = Router::new()
        .nest("/source", source::router())
        .nest("/stickers", sticker::router())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            middlewares::auth::validate_api_key,
        ))
        .with_state(db.clone());

    let app = Router::new()
        .route("/", get(root))
        .nest_service("/static", static_files_service())
        .nest("/secure", api_router)
        .layer(cors)
        .layer(middlewares::tracing::create_tracing_layer())
        .with_state(db);

    let addr = format!("{}:{}", config.host, config.port);
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
