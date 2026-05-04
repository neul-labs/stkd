# stkd-db

[![crates.io](https://img.shields.io/crates/v/stkd-db)](https://crates.io/crates/stkd-db)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--db-blue)](https://docs.rs/stkd-db)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Database abstraction layer for [Stack](https://github.com/neul-labs/stkd) — supports SQLite and PostgreSQL.

This crate provides the database layer used by the Stack web server for persistent storage of organizations, repositories, stacks, branches, and user data. It uses `sqlx` for async, compile-time-checked database access and includes migration support.

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

## Features

- **SQLite** — Embedded, file-based storage (default)
- **PostgreSQL** — Production-ready async connections
- **Migrations** — Automatic schema versioning on startup
- **Type-safe queries** — Compile-time checked via `sqlx`

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
