pub use sea_orm_migration::prelude::*;

mod m20250614_163005_create_sources_table;
mod m20251001_000000_create_stickers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250614_163005_create_sources_table::Migration),
            Box::new(m20251001_000000_create_stickers_table::Migration),
        ]
    }
}
