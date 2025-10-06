use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use tracing::info;

use crate::entities::{prelude::*, sources};

/// Initialize the database connection.
///
/// # Returns
/// A `Result` containing the `DatabaseConnection` if successful, or an error if the initialization fails.
///
/// # Errors
/// Returns an error if:
/// - Database connection fails
/// - `DATABASE_URL` environment variable is not set
///
/// # Panics
/// Panics if the `DATABASE_URL` environment variable is not set.
pub async fn init_pool() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url).await?;

    info!("Database connection initialized");

    Ok(db)
}

/// Increment the counter for a given source.
///
/// # Arguments
/// * `db` - A reference to the `DatabaseConnection` for database operations.
/// * `source` - The name of the source to increment.
///
/// # Returns
/// A `Result` indicating success or failure.
///
/// # Errors
/// Returns an error if:
/// - Database query fails
/// - Database transaction fails
pub async fn increment_source(db: &DatabaseConnection, source: &str) -> Result<(), DbErr> {
    // Try to find existing source
    let existing = Sources::find()
        .filter(sources::Column::Name.eq(source))
        .one(db)
        .await?;

    if let Some(model) = existing {
        // Update existing source
        let mut active_model: sources::ActiveModel = model.into();
        active_model.count = Set(active_model.count.unwrap() + 1);
        active_model.update(db).await?;
    } else {
        // Insert new source
        let new_source = sources::ActiveModel {
            name: Set(source.to_string()),
            count: Set(1),
            ..Default::default()
        };
        new_source.insert(db).await?;
    }

    Ok(())
}
