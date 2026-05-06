//! Error types for provider operations.
//!
//! This module defines the error types that can occur during interactions
//! with Git hosting providers (GitHub, GitLab, etc.).

use thiserror::Error;

/// Errors that can occur during provider operations.
///
/// These errors are provider-agnostic and represent common failure modes
/// across all Git hosting platforms.
///
/// # Examples
///
/// ```rust
/// use stkd_provider_api::ProviderError;
///
/// fn check_error(err: &ProviderError) {
///     if err.is_retryable() {
///         println!("This error can be retried");
///     }
/// }
/// ```
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Authentication failed (invalid token, expired, etc.)
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Authorization denied (valid auth but insufficient permissions)
    #[error("Authorization denied: {0}")]
    AuthorizationDenied(String),

    /// Resource not found (repo, MR, user, etc.)
    #[error("Not found: {0}")]
    NotFound(String),

    /// Rate limited by the provider
    #[error("Rate limited. Retry after {retry_after:?} seconds")]
    RateLimited {
        /// Seconds to wait before retrying, if provided by the API
        retry_after: Option<u64>,
    },

    /// Merge conflict detected
    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    /// Validation error (invalid input, missing required fields, etc.)
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Network error (connection failed, timeout, etc.)
    #[error("Network error: {0}")]
    NetworkError(String),

    /// The operation is not supported by this provider
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    /// Provider-specific error that doesn't fit other categories
    #[error("Provider error: {0}")]
    ProviderSpecific(String),

    /// Internal error (unexpected state, serialization error, etc.)
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ProviderError {
    /// Returns `true` if this error is retryable.
    ///
    /// Retryable errors include network issues and rate limiting.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ProviderError::NetworkError(_) | ProviderError::RateLimited { .. }
        )
    }

    /// Returns `true` if this is an authentication error.
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            ProviderError::AuthenticationFailed(_) | ProviderError::AuthorizationDenied(_)
        )
    }

    /// Returns `true` if the resource was not found.
    pub fn is_not_found(&self) -> bool {
        matches!(self, ProviderError::NotFound(_))
    }
}

/// Result type for provider operations.
pub type ProviderResult<T> = Result<T, ProviderError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_errors() {
        assert!(ProviderError::NetworkError("timeout".into()).is_retryable());
        assert!(ProviderError::RateLimited {
            retry_after: Some(60)
        }
        .is_retryable());
        assert!(!ProviderError::NotFound("repo".into()).is_retryable());
    }

    #[test]
    fn test_auth_errors() {
        assert!(ProviderError::AuthenticationFailed("bad token".into()).is_auth_error());
        assert!(ProviderError::AuthorizationDenied("no access".into()).is_auth_error());
        assert!(!ProviderError::NotFound("repo".into()).is_auth_error());
    }
}
