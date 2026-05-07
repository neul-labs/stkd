# Stack

[![CI](https://github.com/neul-labs/stkd/actions/workflows/ci.yml/badge.svg)](https://github.com/neul-labs/stkd/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/stkd-cli.svg)](https://crates.io/crates/stkd-cli)
[![docs.rs](https://docs.rs/stkd-cli/badge.svg)](https://docs.rs/stkd-cli)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Docs](https://img.shields.io/badge/docs-neullabs.com-green.svg)](https://docs.neullabs.com/stkd)

**Stacked diffs. Simplified.**

Stack is an open-source CLI for managing stacked pull requests on GitHub and GitLab. Break large changes into small, reviewable PRs that stay in sync automatically.

## Why Stacked Diffs?

Large PRs are hard to review. Stacked diffs let you split work into a chain of dependent branches, each with its own focused PR:

```
main
 └── feature/auth-base      PR #1: Core authentication
      └── feature/auth-oauth    PR #2: OAuth support (depends on #1)
           └── feature/auth-2fa     PR #3: 2FA (depends on #2)
```

When you update a branch, Stack automatically rebases all dependent branches. When PRs merge, the stack collapses cleanly.

## Installation

```bash
# Quick install (downloads binary or builds from source)
curl -fsSL https://raw.githubusercontent.com/neul-labs/stkd/main/install.sh | bash

# Or install from crates.io
cargo install stkd-cli

# Or via npm
npm install -g stkd-cli

# Or via pip
pip install stkd-cli

# Or build from source
cargo install --path crates/stkd-cli
```

The binary is named `gt` for Graphite compatibility.

## Quick Start

```bash
# Initialize in your repository
gt init

# Authenticate with GitHub or GitLab
gt auth --github

# Create a stack of branches
gt create feature/step-1    # First branch on main
# ... make changes, commit ...

gt create feature/step-2    # Stacks on step-1
# ... make changes, commit ...

gt create feature/step-3    # Stacks on step-2
# ... make changes, commit ...

# See your stack
gt log
# ○ feature/step-1
#   ○ feature/step-2
#     ◉ feature/step-3  ← you are here

# Or explore interactively
gt tui

# Submit all as PRs
gt submit --stack
```

## Core Workflow

| Command | Description |
|---------|-------------|
| `gt create <name>` | Create a branch stacked on current |
| `gt log` | Visualize the stack |
| `gt up` / `gt down` | Navigate between branches |
| `gt restack` | Rebase all dependent branches |
| `gt submit --stack` | Create/update PRs for the stack |
| `gt sync` | Fetch, restack, clean merged branches |
| `gt land --stack` | Merge the stack in order |
| `gt tui` | Interactive terminal UI |
| `gt modify` | Edit a branch's commits interactively |

## Key Features

- **Auto-restack**: Edit any branch, all dependents rebase automatically
- **GitHub + GitLab**: Full support for both platforms
- **PR automation**: Reviewers, labels, templates, draft PRs
- **Stack-aware submit**: One command creates PRs for your entire stack
- **Interactive TUI**: Keyboard-driven terminal UI for browsing stacks and status
- **MCP Server**: AI agent integration via Model Context Protocol
- **Web Dashboard**: Self-hosted visualization and management UI
- **Conflict handling**: `gt continue` / `gt abort` for rebase conflicts
- **Undo/redo**: Recover from mistakes with `gt undo`

## Try the Demo

See Stack in action with an interactive demo:

```bash
./demo.sh
```

## Documentation

Full documentation at **[docs.neullabs.com/stkd](https://docs.neullabs.com/stkd)**:

- [Getting Started](https://docs.neullabs.com/stkd/getting-started) — Installation, quick start, first stack, authentication
- [Concepts](https://docs.neullabs.com/stkd/concepts/stacked-diffs) — How stacked diffs work
- [Command Reference](https://docs.neullabs.com/stkd/commands) — All commands with examples and flags
- [Guides](https://docs.neullabs.com/stkd/guides/workflows):
  - [Interactive TUI](https://docs.neullabs.com/stkd/guides/tui) — Keyboard-driven terminal UI
  - [Restacking Deep Dive](https://docs.neullabs.com/stkd/guides/restacking)
  - [Branch Management](https://docs.neullabs.com/stkd/guides/branch-management)
  - [Advanced Submit](https://docs.neullabs.com/stkd/guides/advanced-submit)
  - [Stack Templates](https://docs.neullabs.com/stkd/guides/templates)
  - [Performance & Large Repos](https://docs.neullabs.com/stkd/guides/performance)
  - [Day in the Life](https://docs.neullabs.com/stkd/guides/day-in-the-life)
  - [Web Dashboard](https://docs.neullabs.com/stkd/guides/dashboard)
  - [MCP Server for AI Agents](https://docs.neullabs.com/stkd/guides/mcp)
  - [Open Source Maintainers](https://docs.neullabs.com/stkd/guides/open-source)
  - [Migrating from Graphite](https://docs.neullabs.com/stkd/guides/migration-graphite)
  - [Migrating from Git](https://docs.neullabs.com/stkd/guides/migration-git)

## Architecture

```
crates/
├── stkd-cli          # CLI binary (gt)
├── stkd-core         # Core library (Repository, Stack, DAG)
├── stkd-engine       # Programmatic engine for multi-agent harnesses
├── stkd-mcp          # MCP server for AI agent integration
├── stkd-provider-api # Provider trait definitions
├── stkd-github       # GitHub implementation
├── stkd-gitlab       # GitLab implementation
├── stkd-db           # Database layer (SQLite/PostgreSQL)
└── stkd-server       # Web dashboard API
```

## Comparison with Graphite

| Feature | Graphite | Stack |
|---------|----------|-------|
| Open source | No | Yes |
| Self-hosted | No | Yes |
| GitHub | Yes | Yes |
| GitLab | No | Yes |
| CLI compatible | `gt` | `gt` |

## Contributing

```bash
git clone https://github.com/neul-labs/stkd
cd stkd
cargo build
cargo test
```

See [CONTRIBUTING.md](docs/src/contributing/guidelines.md) for guidelines.

## License

Apache-2.0 - See [LICENSE](LICENSE) for details.

---

Built by [Neul Labs](https://neullabs.com)
