//! # Stack Server
//!
//! Web API server for the Stack dashboard. Provides:
//!
//! - REST API for managing organizations, repositories, and stacks
//! - OAuth authentication with GitHub and GitLab
//! - WebSocket support for real-time updates
//! - Webhook handlers for provider events
//!
//! ## Features
//!
//! - Multi-tenant organization support
//! - Real-time stack visualization updates
//! - CI/CD status integration
//!
//! ## Usage
//!
//! ```rust,ignore
//! use stkd_server::{ServerConfig, run_server};
//!
//! let config = ServerConfig::from_env()?;
//! run_server(config).await?;
//! ```

pub mod api;
pub mod auth;
pub mod config;
pub mod error;
pub mod state;
pub mod ws;

// Re-exports
pub use config::ServerConfig;
pub use error::{ApiError, ApiResult};
pub use state::AppState;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Build the application router.
pub fn build_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", api::routes())
        .nest("/ws", ws::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Run the server.
pub async fn run_server(config: ServerConfig) -> anyhow::Result<()> {
    // Initialize database
    let db = stkd_db::create_pool(&config.database).await?;
    db.migrate().await?;

    // Create app state
    let state = AppState::new(db, config.clone());

    // Build app
    let app = build_app(state);

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
