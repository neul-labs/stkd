# stkd-mcp

[![crates.io](https://img.shields.io/crates/v/stkd-mcp)](https://crates.io/crates/stkd-mcp)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--mcp-blue)](https://docs.rs/stkd-mcp)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

MCP server for [Stack](https://github.com/neul-labs/stkd) — exposes `gt` operations as Model Context Protocol tools.

This crate runs a stdio-based MCP server that makes all Stack primitives available to AI agents and IDE integrations. It uses the [`rmcp`](https://crates.io/crates/rmcp) SDK to expose tools such as `gt_init`, `gt_create`, `gt_submit`, `gt_sync`, `gt_land`, and more.

## Installation

```bash
cargo install stkd-mcp
```

## Usage

Add to your MCP client configuration (e.g., Claude Code, Claude Desktop):

```json
{
  "mcpServers": {
    "stkd": {
      "command": "stkd-mcp"
    }
  }
}
```

## Available Tools

- `gt_init` — Initialize Stack in a repository
- `gt_create` — Create a new stacked branch
- `gt_log` — Show the current stack
- `gt_status` — Show branch and MR status
- `gt_submit` — Submit PRs/MRs
- `gt_sync` — Sync with remote
- `gt_land` — Merge the stack
- `gt_restack` — Restack branches
- `gt_track` — Track an existing branch
- `gt_delete` — Delete a tracked branch

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
