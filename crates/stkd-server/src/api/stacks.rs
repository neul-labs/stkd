//! Stack-related API routes.
//!
//! Additional stack endpoints are included in repos.rs.
//! This module contains any stack-specific operations.

use axum::Router;

use crate::state::AppState;

/// Build stack routes.
pub fn routes() -> Router<AppState> {
    Router::new()
}
