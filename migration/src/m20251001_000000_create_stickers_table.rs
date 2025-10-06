use sea_orm_migration::{prelude::*, schema::{double, pk_auto, string, timestamp}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create stickers table
        manager
            .create_table(
                Table::create()
                    .table(Stickers::Table)
                    .if_not_exists()
                    .col(pk_auto(Stickers::Id))
                    .col(string(Stickers::Name).not_null())
                    .col(double(Stickers::Latitude).not_null())
                    .col(double(Stickers::Longitude).not_null())
                    .col(string(Stickers::PlaceName).not_null())
                    .col(
                        ColumnDef::new(Stickers::Pictures)
                            .json_binary()
                            .not_null()
                            .default(Expr::value("[]"))
                    )
                    .col(timestamp(Stickers::CreatedAt).default(Expr::current_timestamp()).not_null())
                    .col(timestamp(Stickers::UpdatedAt).default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stickers_name")
                    .table(Stickers::Table)
                    .col(Stickers::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stickers_created_at")
                    .table(Stickers::Table)
                    .col(Stickers::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create trigger for stickers table (function already created in base_table migration)
        manager
            .get_connection()
            .execute_unprepared(
                r"
                CREATE TRIGGER update_stickers_updated_at
                BEFORE UPDATE ON stickers
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
            .execute_unprepared("DROP TRIGGER IF EXISTS update_stickers_updated_at ON stickers")
            .await?;

        // Drop indexes
        manager
            .drop_index(Index::drop().name("idx_stickers_created_at").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_stickers_name").to_owned())
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().table(Stickers::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Stickers {
    Table,
    Id,
    Name,
    Latitude,
    Longitude,
    PlaceName,
    Pictures,
    CreatedAt,
    UpdatedAt,
}
