use sqlx::postgres::PgPool;
use tracing::info;

/// Initialize the database pool.
///
/// # Returns
/// A `Result` containing the `PgPool` if successful, or an error if the initialization fails.
///
/// # Errors
/// Returns an error if:
/// - Database connection fails
/// - `DATABASE_URL` environment variable is not set
///
/// # Panics
/// Panics if the `DATABASE_URL` environment variable is not set.
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    info!("Database pool initialized");

    Ok(pool)
}

/// Increment the counter for a given source.
///
/// # Arguments
/// * `pool` - A reference to the `PgPool` for database operations.
/// * `source` - The name of the source to increment.
///
/// # Returns
/// A `Result` indicating success or failure.
///
/// # Errors
/// Returns an error if:
/// - Database query fails
/// - Database transaction fails
pub async fn increment_source(pool: &PgPool, source: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        INSERT INTO sources (name, count)
        VALUES ($1, 1)
        ON CONFLICT(name) DO UPDATE SET
            count = sources.count + 1,
            updated_at = CURRENT_TIMESTAMP
        ",
    )
    .bind(source)
    .execute(pool)
    .await?;

    Ok(())
}
