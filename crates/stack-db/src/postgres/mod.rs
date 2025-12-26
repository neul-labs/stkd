//! PostgreSQL database implementation.
//!
//! This module provides PostgreSQL support for production deployments.
//! Enable with the `postgres` feature flag.

mod pool;

pub use pool::PostgresPool;
