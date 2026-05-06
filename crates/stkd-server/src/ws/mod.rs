//! WebSocket support for real-time updates.

pub mod hub;
pub mod messages;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::state::AppState;

use hub::Hub;
use messages::{ClientMessage, ServerMessage};

/// WebSocket state.
#[derive(Clone)]
pub struct WsState {
    hub: Arc<Hub>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            hub: Arc::new(Hub::new()),
        }
    }

    pub fn hub(&self) -> &Hub {
        &self.hub
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket upgrade handler.
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle a WebSocket connection.
async fn handle_socket(socket: WebSocket, _state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for this connection
    let (tx, mut rx) = broadcast::channel::<ServerMessage>(100);

    // Spawn a task to forward messages from the hub to the client
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        let msg = match msg {
            Ok(Message::Text(text)) => text,
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => continue,
        };

        // Parse message
        let client_msg: ClientMessage = match serde_json::from_str(&msg) {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!("Invalid WebSocket message: {}", e);
                continue;
            }
        };

        // Handle message
        match client_msg {
            ClientMessage::Subscribe { channel } => {
                tracing::debug!("Client subscribed to channel: {}", channel);
                // In a full implementation, we would track subscriptions
            }
            ClientMessage::Unsubscribe { channel } => {
                tracing::debug!("Client unsubscribed from channel: {}", channel);
            }
            ClientMessage::Ping => {
                let _ = tx.send(ServerMessage::Pong);
            }
        }
    }

    // Clean up
    send_task.abort();
}

/// Build WebSocket routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(ws_handler))
}
