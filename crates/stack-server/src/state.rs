//! Application state.

use std::sync::Arc;

use stack_db::DatabasePool;

use crate::config::ServerConfig;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    db: Box<dyn DatabasePool>,
    config: ServerConfig,
}

impl AppState {
    /// Create a new application state.
    pub fn new(db: Box<dyn DatabasePool>, config: ServerConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { db, config }),
        }
    }

    /// Get the database pool.
    pub fn db(&self) -> &dyn DatabasePool {
        self.inner.db.as_ref()
    }

    /// Get the server configuration.
    pub fn config(&self) -> &ServerConfig {
        &self.inner.config
    }
}
