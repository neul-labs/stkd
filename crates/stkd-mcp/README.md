# stkd-mcp — MCP Server for AI Agents (Stack / Stacked Diffs)

[![crates.io](https://img.shields.io/crates/v/stkd-mcp.svg)](https://crates.io/crates/stkd-mcp)
[![docs.rs](https://docs.rs/stkd-mcp/badge.svg)](https://docs.rs/stkd-mcp)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**MCP server for Stack** — exposes `gt` operations as Model Context Protocol tools for AI agents and IDE integrations.

`stkd-mcp` runs a stdio-based MCP server that makes all Stack primitives available to AI agents and IDE integrations. It uses the [`rmcp`](https://crates.io/crates/rmcp) SDK to expose tools such as `gt_init`, `gt_create`, `gt_submit`, `gt_sync`, `gt_land`, and more.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. It breaks large changes into small, reviewable PRs that stay in sync automatically.

## What is MCP?

The **Model Context Protocol (MCP)** is an open standard for connecting AI assistants to tools and data sources. `stkd-mcp` lets Claude Code, Cursor, and other MCP-compatible clients create stacks, submit PRs, sync branches, and more — directly from natural language prompts.

## Installation

```bash
cargo install stkd-mcp
```

## Usage

Add to your MCP client configuration (e.g., Claude Code, Claude Desktop, Cursor):

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
- `gt_status` — Show branch and PR/MR status
- `gt_submit` — Submit PRs/MRs
- `gt_sync` — Sync with remote
- `gt_land` — Merge the stack
- `gt_restack` — Restack branches
- `gt_track` — Track an existing branch
- `gt_delete` — Delete a tracked branch

## Features

- **AI-native** — Designed for LLM-driven workflows
- **Structured tools** — Each tool has typed inputs and outputs
- **Safe by default** — No destructive operations without explicit confirmation
- **Zero-config** — Auto-detects repository and provider settings

## Related Crates

- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary
- [`stkd-engine`](https://crates.io/crates/stkd-engine) — Programmatic API
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
