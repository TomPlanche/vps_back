use sqlx::sqlite::SqlitePool;
use tracing::info;

/// Initialize the database pool.
///
/// # Returns
/// A `Result` containing the `SqlitePool` if successful, or an error if the initialization fails.
///
/// # Errors
/// Returns an error if:
/// - Database connection fails
pub async fn init_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:data/data.db").await?;

    info!("Database pool initialized");

    Ok(pool)
}

/// Increment the counter for a given source.
///
/// # Arguments
/// * `pool` - A reference to the `SqlitePool` for database operations.
/// * `source` - The name of the source to increment.
///
/// # Returns
/// A `Result` indicating success or failure.
///
/// # Errors
/// Returns an error if:
/// - Database query fails
/// - Database transaction fails
pub async fn increment_source(pool: &SqlitePool, source: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        INSERT INTO sources (name, count)
        VALUES (?, 1)
        ON CONFLICT(name) DO UPDATE SET
            count = count + 1,
            updated_at = CURRENT_TIMESTAMP
        ",
    )
    .bind(source)
    .execute(pool)
    .await?;

    Ok(())
}
