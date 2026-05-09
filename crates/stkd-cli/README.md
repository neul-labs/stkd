# Stack CLI — Stacked Diffs for Git

[![crates.io](https://img.shields.io/crates/v/stkd-cli.svg)](https://crates.io/crates/stkd-cli)
[![npm](https://img.shields.io/npm/v/stkd-cli.svg)](https://www.npmjs.com/package/stkd-cli)
[![PyPI](https://img.shields.io/pypi/v/stkd-cli.svg)](https://pypi.org/project/stkd-cli/)
[![docs.rs](https://docs.rs/stkd-cli/badge.svg)](https://docs.rs/stkd-cli)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Stack** is an open-source CLI for managing **stacked pull requests** on GitHub and GitLab. Break large changes into small, reviewable PRs that stay in sync automatically. Stack is **Graphite-compatible** (the binary is named `gt`) and supports GitHub, GitLab, and self-hosted GitLab instances.

---

## What is Stack?

Large pull requests are hard to review. **Stacked diffs** let you split work into a chain of dependent branches, each with its own focused PR:

```
main
 └── feature/auth-base      PR #1: Core authentication
      └── feature/auth-oauth    PR #2: OAuth support (depends on #1)
           └── feature/auth-2fa     PR #3: 2FA (depends on #2)
```

When you update a branch, Stack automatically rebases all dependent branches. When PRs merge, the stack collapses cleanly.

## Why Stacked Diffs?

| Traditional PRs | Stacked Diffs |
|---------------|---------------|
| One large PR with many changes | Multiple small, focused PRs |
| Reviewers overwhelmed | Easy to review incrementally |
| All-or-nothing merging | Land changes as they're approved |
| Blocked waiting for review | Unblock yourself, keep coding |

## Key Features

- **Auto-restack** — Edit any branch, all dependents rebase automatically
- **GitHub + GitLab** — Full support for both platforms, including self-hosted GitLab
- **PR automation** — Reviewers, labels, templates, draft PRs
- **Stack-aware submit** — One command creates PRs for your entire stack
- **Interactive TUI** — Keyboard-driven terminal UI for browsing stacks and status
- **MCP Server** — AI agent integration via Model Context Protocol (Claude Code, Cursor, etc.)
- **Web Dashboard** — Self-hosted visualization and management UI
- **Conflict handling** — `gt continue` / `gt abort` for rebase conflicts
- **Undo/redo** — Recover from mistakes with `gt undo`
- **Graphite compatible** — Drop-in replacement for `gt` commands

## Installation

### Homebrew (macOS / Linux)

```bash
brew install neul-labs/tap/stkd
```

### Cargo (Rust)

```bash
cargo install stkd-cli
```

### npm (Node.js)

```bash
npm install -g stkd-cli
```

### pip (Python)

```bash
pip install stkd-cli
```

### Binary Download

Download prebuilt binaries from [GitHub Releases](https://github.com/neul-labs/stkd/releases).

### Quick Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/neul-labs/stkd/main/install.sh | bash
```

### Build from Source

```bash
git clone https://github.com/neul-labs/stkd
cd stkd
cargo install --path crates/stkd-cli
```

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

## Core Commands

| Command | Description |
|---------|-------------|
| `gt init` | Initialize Stack in this repository |
| `gt create <name>` | Create a branch stacked on current |
| `gt log` | Visualize the stack |
| `gt up` / `gt down` | Navigate between branches |
| `gt restack` | Rebase all dependent branches |
| `gt submit --stack` | Create/update PRs for the stack |
| `gt sync` | Fetch, restack, clean merged branches |
| `gt land --stack` | Merge the stack in order |
| `gt tui` | Interactive terminal UI |
| `gt modify` | Edit a branch's commits interactively |
| `gt undo` | Undo the last operation |
| `gt continue` / `gt abort` | Resolve rebase conflicts |

## Comparison with Graphite

| Feature | Graphite | Stack |
|---------|----------|-------|
| Open source | No | **Yes** |
| Self-hosted | No | **Yes** |
| GitHub | Yes | Yes |
| GitLab | No | **Yes** |
| CLI compatible | `gt` | `gt` |

## Documentation

Full documentation at **[docs.neullabs.com/stkd](https://docs.neullabs.com/stkd)**:

- [Getting Started](https://docs.neullabs.com/stkd/getting-started)
- [Command Reference](https://docs.neullabs.com/stkd/commands)
- [Interactive TUI Guide](https://docs.neullabs.com/stkd/guides/tui)
- [MCP Server for AI Agents](https://docs.neullabs.com/stkd/guides/mcp)
- [Web Dashboard](https://docs.neullabs.com/stkd/guides/dashboard)
- [Migrating from Graphite](https://docs.neullabs.com/stkd/guides/migration-graphite)

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.

Built by [Neul Labs](https://neullabs.com).
