//! WebSocket message types.

use serde::{Deserialize, Serialize};

/// Messages sent from the client.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Subscribe to a channel.
    Subscribe { channel: String },
    /// Unsubscribe from a channel.
    Unsubscribe { channel: String },
    /// Ping to keep connection alive.
    Ping,
}

/// Messages sent from the server.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Pong response to ping.
    Pong,
    /// Merge request updated.
    MrUpdated {
        repo_id: String,
        branch_name: String,
        mr_number: u64,
        state: String,
    },
    /// Branch synced.
    BranchSynced {
        repo_id: String,
        branch_name: String,
        head_sha: String,
    },
    /// CI status changed.
    CiStatusChanged {
        repo_id: String,
        branch_name: String,
        status: String,
    },
    /// Error message.
    Error { message: String },
}
