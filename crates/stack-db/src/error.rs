//! Database error types.

use thiserror::Error;

/// Database error type.
#[derive(Debug, Error)]
pub enum DbError {
    /// Database connection error
    #[error("Database connection error: {0}")]
    Connection(String),

    /// Query execution error
    #[error("Query error: {0}")]
    Query(String),

    /// Record not found
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Duplicate record
    #[error("Duplicate record: {0}")]
    Duplicate(String),

    /// Migration error
    #[error("Migration error: {0}")]
    Migration(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// SQLx error wrapper
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

/// Result type alias for database operations.
pub type DbResult<T> = Result<T, DbError>;
