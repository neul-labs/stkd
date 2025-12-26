//! Session model.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A user session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier
    pub id: Uuid,
    /// User this session belongs to
    pub user_id: Uuid,
    /// Token hash for validation
    pub token_hash: String,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// User agent from the request
    pub user_agent: Option<String>,
    /// IP address from the request
    pub ip_address: Option<String>,
}

impl Session {
    /// Create a new session.
    pub fn new(user_id: Uuid, token_hash: String, duration_days: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            created_at: now,
            expires_at: now + Duration::days(duration_days),
            user_agent: None,
            ip_address: None,
        }
    }

    /// Create a session with request metadata.
    pub fn with_metadata(
        user_id: Uuid,
        token_hash: String,
        duration_days: i64,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Self {
        let mut session = Self::new(user_id, token_hash, duration_days);
        session.user_agent = user_agent;
        session.ip_address = ip_address;
        session
    }

    /// Check if this session has expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if this session is still valid.
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}
