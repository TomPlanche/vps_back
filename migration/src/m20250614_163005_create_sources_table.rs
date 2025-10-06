use sea_orm_migration::{prelude::*, schema::{integer, pk_auto, string_uniq, timestamp}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create sources table
        manager
            .create_table(
                Table::create()
                    .table(Sources::Table)
                    .if_not_exists()
                    .col(pk_auto(Sources::Id))
                    .col(string_uniq(Sources::Name))
                    .col(integer(Sources::Count).default(0).not_null())
                    .col(timestamp(Sources::CreatedAt).default(Expr::current_timestamp()).not_null())
                    .col(timestamp(Sources::UpdatedAt).default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sources_name")
                    .table(Sources::Table)
                    .col(Sources::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sources_created_at")
                    .table(Sources::Table)
                    .col(Sources::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create trigger function for updated_at
        manager
            .get_connection()
            .execute_unprepared(
                r"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                ",
            )
            .await?;

        // Create trigger for sources table
        manager
            .get_connection()
            .execute_unprepared(
                r"
                CREATE TRIGGER update_sources_updated_at
                BEFORE UPDATE ON sources
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
                ",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_sources_updated_at ON sources")
            .await?;

        // Drop indexes
        manager
            .drop_index(Index::drop().name("idx_sources_created_at").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_sources_name").to_owned())
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().table(Sources::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Sources {
    Table,
    Id,
    Name,
    Count,
    CreatedAt,
    UpdatedAt,
}
