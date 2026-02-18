# vps-back

Rust/Axum backend running on my VPS. Handles analytics tracking, location stickers, and Homebrew bottle download counting. PostgreSQL via SeaORM.

## Setup

```bash
cp .env.example .env
sea-orm-cli migrate up
cargo run
```

## Configuration

| Variable          | Required | Default                                       |
|-------------------|----------|-----------------------------------------------|
| `API_KEY`         | yes      | —                                             |
| `DATABASE_URL`    | yes      | —                                             |
| `HOST`            | no       | `127.0.0.1`                                   |
| `PORT`            | no       | `8000`                                        |
| `ALLOWED_ORIGINS` | no       | `http://localhost:3000,http://localhost:5173`  |
| `RUST_LOG`        | no       | `info`                                        |

`DATABASE_URL` format: `postgresql://user:password@host:port/dbname`

## Endpoints

### Public

**GET /** — health check

**GET /static/\<path\>** — static files (LastFM fetcher output)

**GET /brew/track/:project/:filename** — records a Homebrew bottle download and redirects (`302`) to the real GitHub release asset. Used by setting `root_url` in the formula's bottle block:

```ruby
bottle do
  root_url "https://your-server.com/brew/track/rona"
  sha256 cellar: :any_skip_relocation, arm64_sequoia: "<sha>"
  sha256 cellar: :any_skip_relocation, sequoia:       "<sha>"
  sha256 cellar: :any_skip_relocation, x86_64_linux:  "<sha>"
end
```

**GET /brew/stats** — download counts grouped by project and version:

```json
{
  "data": {
    "rona": {
      "total_downloads": 120,
      "total_installs": 120,
      "2.17.7": { "downloads": 80, "installs": 80 }
    }
  }
}
```

### Protected

All `/secure/*` routes require `x-api-key: <key>` in the request headers.

| Method | Path                   | Description                        |
|--------|------------------------|------------------------------------|
| GET    | /secure/source         | All sources and their hit counts   |
| POST   | /secure/source         | Increment a source counter         |
| GET    | /secure/stickers       | All stickers, newest first         |
| GET    | /secure/stickers/:id   | Single sticker                     |
| POST   | /secure/stickers       | Create a sticker                   |

Paginated endpoints accept `?page=1&limit=20` (max limit: 100) and include a `_metadata` field in the response.

## Database

```bash
sea-orm-cli migrate up       # apply pending
sea-orm-cli migrate status   # check state
sea-orm-cli migrate down     # roll back one
sea-orm-cli migrate refresh  # reset and re-run all
```

Tables: `sources`, `stickers`, `brew_downloads`.

## Development

```bash
cargo check              # type-check
cargo clippy             # lint
cargo test               # tests
cargo build --release    # production build
RUST_LOG=sqlx::query=debug cargo run  # with SQL logging
```

## License

MIT
