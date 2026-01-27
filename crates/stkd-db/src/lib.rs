//! # Stack Database Abstraction Layer
//!
//! This crate provides a database abstraction layer for Stack that supports
//! both SQLite (for development and self-hosted) and PostgreSQL (for production).
//!
//! ## Features
//!
//! - `sqlite` (default): Enable SQLite backend support
//! - `postgres`: Enable PostgreSQL backend support
//!
//! ## Usage
//!
//! ```rust,ignore
//! use stkd_db::{DatabaseConfig, DatabaseBackend, create_pool};
//!
//! let config = DatabaseConfig {
//!     url: "sqlite::memory:".to_string(),
//!     backend: DatabaseBackend::Sqlite,
//! };
//!
//! let pool = create_pool(&config).await?;
//! pool.migrate().await?;
//! ```

pub mod config;
pub mod error;
pub mod models;
pub mod pool;
pub mod repositories;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "postgres")]
pub mod postgres;

// Re-exports
pub use config::{DatabaseBackend, DatabaseConfig};
pub use error::{DbError, DbResult};
pub use pool::{create_pool, DatabasePool};

// Re-export models
pub use models::{
    Branch, BranchStatus, MergeRequest, MergeRequestState, Membership, MembershipRole,
    Organization, Repository, Session, User,
};

// Re-export repository traits
pub use repositories::{
    BranchRepository, MergeRequestRepository, MembershipRepository, OrganizationRepository,
    RepositoryRepository, SessionRepository, UserRepository,
};
