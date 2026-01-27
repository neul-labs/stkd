//! Stack Server - Web API for the Stack dashboard.

use anyhow::Result;
use stkd_server::{run_server, ServerConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "stkd_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = ServerConfig::from_env()?;

    tracing::info!("Stack Server v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Database: {:?}", config.database.backend);

    // Run server
    run_server(config).await
}
