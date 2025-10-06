# SeaORM Migrations

This project uses SeaORM migrations written in Rust to manage the PostgreSQL database schema.

## Prerequisites

Install `sea-orm-cli`:
```bash
cargo install sea-orm-cli
```

## Configuration

Migrations use the `DATABASE_URL` environment variable to connect to PostgreSQL.

Make sure you have a `.env` file at the project root with:
```bash
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
```

## Commands with sea-orm-cli (recommended)

### Run all pending migrations
```bash
sea-orm-cli migrate up
```

### Check migration status
```bash
sea-orm-cli migrate status
```

### Rollback the last migration
```bash
sea-orm-cli migrate down
```

### Refresh (rollback all + re-run all)
```bash
sea-orm-cli migrate refresh
```

### Create a new migration
```bash
sea-orm-cli migrate generate <migration_name>
```

## Commands with cargo run (alternative)

You can also run migrations via the integrated CLI:

### Apply all migrations
```bash
cargo run --manifest-path migration/Cargo.toml
# or
cargo run --manifest-path migration/Cargo.toml -- up
```

### Apply the first N migrations
```bash
cargo run --manifest-path migration/Cargo.toml -- up -n 10
```

### Rollback the last migration
```bash
cargo run --manifest-path migration/Cargo.toml -- down
```

### Rollback the last N migrations
```bash
cargo run --manifest-path migration/Cargo.toml -- down -n 2
```

### Drop all tables and re-run migrations
```bash
cargo run --manifest-path migration/Cargo.toml -- fresh
```

### Rollback all + re-run all
```bash
cargo run --manifest-path migration/Cargo.toml -- refresh
```

### Rollback all migrations
```bash
cargo run --manifest-path migration/Cargo.toml -- reset
```

### Check status
```bash
cargo run --manifest-path migration/Cargo.toml -- status
```

### Generate a new migration
```bash
cargo run --manifest-path migration/Cargo.toml -- generate <migration_name>
```

## Migration Structure

Migrations are in `migration/src/`:
- `m20250614_163005_create_sources_table.rs` - Creates sources table with update trigger
- `m20251001_000000_create_stickers_table.rs` - Creates stickers table

Each migration implements:
- `up()` - Applies changes
- `down()` - Reverts changes (rollback)

## Running from Rust Code

Migrations can also be executed programmatically:

```rust
use migration::{Migrator, MigratorTrait};

// Run all migrations
Migrator::up(&db, None).await?;

// Rollback
Migrator::down(&db, None).await?;
```

## Notes

- Migrations are executed in alphabetical order of their filename
- Filenames follow the pattern `mYYYYMMDD_HHMMSS_description.rs`
- PostgreSQL triggers are handled via `execute_unprepared()` for raw SQL
- Indexes are created with the SeaORM schema builder
