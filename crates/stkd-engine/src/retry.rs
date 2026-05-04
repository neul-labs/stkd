//! Retry utility for provider operations.
//!
//! Wraps async provider calls with exponential backoff for transient failures.

use std::time::Duration;
use stkd_provider_api::{ProviderError, ProviderResult};

/// Default maximum number of retries.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (milliseconds).
pub const DEFAULT_RETRY_BASE_MS: u64 = 500;

/// Retry an async operation with exponential backoff.
///
/// Only retries if the error is retryable according to [`ProviderError::is_retryable`].
///
/// # Example
///
/// ```rust,ignore
/// let mr = with_retry(|| provider.create_mr(repo_id, request), DEFAULT_MAX_RETRIES).await?;
/// ```
pub async fn with_retry<F, Fut, T>(operation: F, max_retries: u32) -> ProviderResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ProviderResult<T>>,
{
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                if !err.is_retryable() || attempt == max_retries {
                    return Err(err);
                }
                let delay = DEFAULT_RETRY_BASE_MS * 2_u64.pow(attempt);
                tracing::warn!(
                    "Provider call failed (attempt {}/{}), retrying in {}ms: {}",
                    attempt + 1,
                    max_retries + 1,
                    delay,
                    err
                );
                tokio::time::sleep(Duration::from_millis(delay)).await;
                last_error = Some(err);
            }
        }
    }

    // Unreachable because the loop returns on success or when max_retries is reached,
    // but we need this to satisfy the compiler.
    Err(last_error.unwrap_or_else(|| ProviderError::Internal("Retry loop exhausted".to_string())))
}
