//! Stack MCP Server
//!
//! This crate provides a [Model Context Protocol](https://modelcontextprotocol.io) server
//! that exposes Stack (`gt`) operations as MCP tools. It communicates over stdio and is
//! designed for integration with AI agents and IDE plugins.
//!
//! The server exposes the following tools:
//! - `gt_init` — Initialize Stack in a repository
//! - `gt_create` — Create a new stacked branch
//! - `gt_log` — Show the current stack
//! - `gt_status` — Show branch and merge request status
//! - `gt_submit` — Submit PRs/MRs for the current branch or stack
//! - `gt_sync` — Sync with remote, restack, and clean merged branches
//! - `gt_land` — Merge and land the current branch or stack
//! - `gt_restack` — Restack branches onto updated parents
//! - `gt_track` — Track an existing branch for stacking
//! - `gt_delete` — Delete a tracked branch
//!
//! # Usage
//!
//! Run the `stkd-mcp` binary to start the server over stdio:
//!
//! ```bash
//! stkd-mcp
//! ```
//!
//! To integrate with Claude Code, add the following to your MCP configuration:
//!
//! ```json
//! {
//!   "mcpServers": {
//!     "stkd": {
//!       "command": "stkd-mcp"
//!     }
//!   }
//! }
//! ```
//!
//! For more information, see the [Stack documentation](https://docs.neullabs.com/stkd).
