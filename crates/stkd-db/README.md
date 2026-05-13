# stkd-db — SQLite / PostgreSQL Database Layer for Stack

[![crates.io](https://img.shields.io/crates/v/stkd-db.svg)](https://crates.io/crates/stkd-db)
[![docs.rs](https://docs.rs/stkd-db/badge.svg)](https://docs.rs/stkd-db)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Database abstraction layer for Stack** — SQLite and PostgreSQL support with type-safe queries via sqlx.

`stkd-db` provides the database layer used by the Stack web server for persistent storage of organizations, repositories, stacks, branches, and user data. It uses `sqlx` for async, compile-time-checked database access and includes migration support.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. `stkd-db` powers the self-hosted Stack dashboard's persistent storage layer.

## Features

- **SQLite** — Embedded, file-based storage (default)
- **PostgreSQL** — Production-ready async connections
- **Migrations** — Automatic schema versioning on startup
- **Type-safe queries** — Compile-time checked via `sqlx`

## Installation

```bash
cargo add stkd-db
```

## Usage

```rust
use stkd_db::{Database, DatabaseConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect(DatabaseConfig::default()).await?;

    // Run migrations automatically
    db.migrate().await?;

    println!("Database connected and migrated");
    Ok(())
}
```

## Related Crates

- [`stkd-server`](https://crates.io/crates/stkd-server) — Web dashboard API server
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
