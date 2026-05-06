//! Stack Engine - Programmatic API for Stacked Diffs
//!
//! This library exposes `gt` CLI operations as reusable, composable
//! functions that return structured, serializable results. It is designed
//! for consumption by multi-agent harnesses, IDE plugins, CI systems,
//! and other programmatic callers.
//!
//! # Example
//!
//! ```rust,ignore
//! use stkd_engine::{ProviderContext, submit, SubmitOptions};
//! use stkd_core::Repository;
//!
//! async fn example() -> anyhow::Result<()> {
//!     let repo = Repository::open(".")?;
//!     let ctx = ProviderContext::from_repo(&repo).await?;
//!     let result = submit(&repo, SubmitOptions::default(), ctx.provider(), &ctx.repo_id).await?;
//!     println!("Created {} MRs", result.created.len());
//!     Ok(())
//! }
//! ```

pub mod init;
pub mod land;
pub mod provider;
pub mod restack;
pub mod retry;
pub mod submit;
pub mod sync;

pub use init::{init, InitOptions, InitResult};
pub use land::{land, LandOptions, LandResult, LandedBranch};
pub use provider::{detect_provider_type, ProviderContext, ProviderType};
pub use restack::{restack, RestackEntry, RestackOptions, RestackResult, RestackStatus};
pub use submit::{select_branches, submit, CreatedMr, SubmitOptions, SubmitResult, UpdatedMr};
pub use sync::{sync, SyncOptions, SyncResult};
