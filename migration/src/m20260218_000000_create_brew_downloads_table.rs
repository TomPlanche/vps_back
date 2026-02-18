use sea_orm_migration::{
    prelude::*,
    schema::{integer, pk_auto, string, timestamp},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BrewDownloads::Table)
                    .if_not_exists()
                    .col(pk_auto(BrewDownloads::Id))
                    .col(string(BrewDownloads::Project).not_null())
                    .col(string(BrewDownloads::Version).not_null())
                    .col(string(BrewDownloads::Platform).not_null())
                    .col(integer(BrewDownloads::Count).default(0).not_null())
                    .col(
                        timestamp(BrewDownloads::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(BrewDownloads::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (project, version, platform)
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_brew_downloads_unique")
                    .table(BrewDownloads::Table)
                    .col(BrewDownloads::Project)
                    .col(BrewDownloads::Version)
                    .col(BrewDownloads::Platform)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index for querying by project
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_brew_downloads_project")
                    .table(BrewDownloads::Table)
                    .col(BrewDownloads::Project)
                    .to_owned(),
            )
            .await?;

        // Trigger for updated_at (reuses function created in sources migration)
        manager
            .get_connection()
            .execute_unprepared(
                r"
                CREATE TRIGGER update_brew_downloads_updated_at
                BEFORE UPDATE ON brew_downloads
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
                ",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS update_brew_downloads_updated_at ON brew_downloads",
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_brew_downloads_project")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_brew_downloads_unique")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(BrewDownloads::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum BrewDownloads {
    Table,
    Id,
    Project,
    Version,
    Platform,
    Count,
    CreatedAt,
    UpdatedAt,
}
