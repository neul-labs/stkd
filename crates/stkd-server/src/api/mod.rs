//! API routes.

pub mod auth;
pub mod orgs;
pub mod repos;
pub mod stacks;
pub mod webhooks;

use axum::{routing::get, Router};

use crate::state::AppState;

/// Health check endpoint.
async fn health() -> &'static str {
    "OK"
}

/// Build API routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", auth::routes())
        .nest("/orgs", orgs::routes())
        .nest("/repos", repos::routes())
        .nest("/webhooks", webhooks::routes())
}
