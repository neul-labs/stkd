//! Database configuration.

use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};

/// Database backend type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseBackend {
    /// SQLite database (for development and self-hosted)
    Sqlite,
    /// PostgreSQL database (for production)
    Postgres,
}

impl std::fmt::Display for DatabaseBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseBackend::Sqlite => write!(f, "sqlite"),
            DatabaseBackend::Postgres => write!(f, "postgres"),
        }
    }
}

impl std::str::FromStr for DatabaseBackend {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sqlite" => Ok(DatabaseBackend::Sqlite),
            "postgres" | "postgresql" => Ok(DatabaseBackend::Postgres),
            _ => Err(DbError::Config(format!("Unknown database backend: {}", s))),
        }
    }
}

/// Database configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Database backend type
    pub backend: DatabaseBackend,
    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

fn default_min_connections() -> u32 {
    1
}

impl DatabaseConfig {
    /// Create a new SQLite configuration.
    pub fn sqlite(path: &str) -> Self {
        Self {
            url: format!("sqlite:{}", path),
            backend: DatabaseBackend::Sqlite,
            max_connections: 5,
            min_connections: 1,
        }
    }

    /// Create a new in-memory SQLite configuration (for testing).
    pub fn sqlite_memory() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            backend: DatabaseBackend::Sqlite,
            max_connections: 1,
            min_connections: 1,
        }
    }

    /// Create a new PostgreSQL configuration.
    pub fn postgres(url: &str) -> Self {
        Self {
            url: url.to_string(),
            backend: DatabaseBackend::Postgres,
            max_connections: 10,
            min_connections: 2,
        }
    }

    /// Create configuration from environment variables.
    pub fn from_env() -> DbResult<Self> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| DbError::Config("DATABASE_URL not set".to_string()))?;

        let backend = std::env::var("STACK_DB_BACKEND")
            .unwrap_or_else(|_| {
                // Auto-detect from URL
                if url.starts_with("sqlite:") {
                    "sqlite".to_string()
                } else if url.starts_with("postgres") {
                    "postgres".to_string()
                } else {
                    "sqlite".to_string()
                }
            })
            .parse()?;

        let max_connections = std::env::var("STACK_DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(default_max_connections);

        let min_connections = std::env::var("STACK_DB_MIN_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(default_min_connections);

        Ok(Self {
            url,
            backend,
            max_connections,
            min_connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_from_str() {
        assert_eq!(
            "sqlite".parse::<DatabaseBackend>().unwrap(),
            DatabaseBackend::Sqlite
        );
        assert_eq!(
            "postgres".parse::<DatabaseBackend>().unwrap(),
            DatabaseBackend::Postgres
        );
        assert_eq!(
            "postgresql".parse::<DatabaseBackend>().unwrap(),
            DatabaseBackend::Postgres
        );
    }

    #[test]
    fn test_sqlite_config() {
        let config = DatabaseConfig::sqlite("./data/stack.db");
        assert_eq!(config.backend, DatabaseBackend::Sqlite);
        assert!(config.url.contains("stack.db"));
    }
}
