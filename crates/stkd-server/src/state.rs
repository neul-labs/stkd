//! Application state.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::RwLock;
use stkd_db::DatabasePool;

use crate::config::ServerConfig;

/// OAuth state entry with expiration.
struct OAuthStateEntry {
    created_at: Instant,
}

/// OAuth state store for CSRF protection.
///
/// Stores OAuth state tokens temporarily to validate callbacks.
/// Entries expire after 10 minutes.
pub struct OAuthStateStore {
    states: RwLock<HashMap<String, OAuthStateEntry>>,
    ttl: Duration,
}

impl OAuthStateStore {
    /// Create a new OAuth state store.
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(600), // 10 minutes
        }
    }

    /// Store a new OAuth state.
    pub fn store(&self, state: &str) {
        let mut states = self.states.write();
        // Clean up expired entries while we have the lock
        let now = Instant::now();
        states.retain(|_, entry| now.duration_since(entry.created_at) < self.ttl);
        // Insert new state
        states.insert(state.to_string(), OAuthStateEntry { created_at: now });
    }

    /// Validate and consume an OAuth state.
    ///
    /// Returns `true` if the state was valid and has been consumed.
    /// Returns `false` if the state was invalid, expired, or already used.
    pub fn validate(&self, state: &str) -> bool {
        let mut states = self.states.write();
        if let Some(entry) = states.remove(state) {
            // Check if expired
            Instant::now().duration_since(entry.created_at) < self.ttl
        } else {
            false
        }
    }
}

impl Default for OAuthStateStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    db: Box<dyn DatabasePool>,
    config: ServerConfig,
    oauth_states: OAuthStateStore,
}

impl AppState {
    /// Create a new application state.
    pub fn new(db: Box<dyn DatabasePool>, config: ServerConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                db,
                config,
                oauth_states: OAuthStateStore::new(),
            }),
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

    /// Get the OAuth state store.
    pub fn oauth_states(&self) -> &OAuthStateStore {
        &self.inner.oauth_states
    }
}
