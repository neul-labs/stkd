//! WebSocket connection hub for broadcasting messages.

use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::broadcast;
use uuid::Uuid;

use super::messages::ServerMessage;

/// Connection hub for managing WebSocket connections.
pub struct Hub {
    /// Channel subscribers (channel_name -> list of connection IDs)
    channels: RwLock<HashMap<String, Vec<Uuid>>>,
    /// Connection senders (connection_id -> sender)
    connections: RwLock<HashMap<Uuid, broadcast::Sender<ServerMessage>>>,
}

impl Hub {
    /// Create a new hub.
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new connection.
    pub fn register(&self, id: Uuid, sender: broadcast::Sender<ServerMessage>) {
        let mut connections = self.connections.write().unwrap();
        connections.insert(id, sender);
    }

    /// Unregister a connection.
    pub fn unregister(&self, id: Uuid) {
        // Remove from connections
        {
            let mut connections = self.connections.write().unwrap();
            connections.remove(&id);
        }

        // Remove from all channels
        {
            let mut channels = self.channels.write().unwrap();
            for subscribers in channels.values_mut() {
                subscribers.retain(|&conn_id| conn_id != id);
            }
        }
    }

    /// Subscribe a connection to a channel.
    pub fn subscribe(&self, conn_id: Uuid, channel: &str) {
        let mut channels = self.channels.write().unwrap();
        let subscribers = channels.entry(channel.to_string()).or_default();
        if !subscribers.contains(&conn_id) {
            subscribers.push(conn_id);
        }
    }

    /// Unsubscribe a connection from a channel.
    pub fn unsubscribe(&self, conn_id: Uuid, channel: &str) {
        let mut channels = self.channels.write().unwrap();
        if let Some(subscribers) = channels.get_mut(channel) {
            subscribers.retain(|&id| id != conn_id);
        }
    }

    /// Broadcast a message to all subscribers of a channel.
    pub fn broadcast(&self, channel: &str, message: ServerMessage) {
        let channels = self.channels.read().unwrap();
        let connections = self.connections.read().unwrap();

        if let Some(subscribers) = channels.get(channel) {
            for conn_id in subscribers {
                if let Some(sender) = connections.get(conn_id) {
                    let _ = sender.send(message.clone());
                }
            }
        }
    }

    /// Broadcast a message to all connections.
    pub fn broadcast_all(&self, message: ServerMessage) {
        let connections = self.connections.read().unwrap();
        for sender in connections.values() {
            let _ = sender.send(message.clone());
        }
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
