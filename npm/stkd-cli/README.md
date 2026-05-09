# stkd-cli

[![npm](https://img.shields.io/npm/v/stkd-cli.svg)](https://www.npmjs.com/package/stkd-cli)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Stack CLI distributed via npm** — Install the `gt` binary for managing stacked pull requests on GitHub and GitLab.

Stack is an open-source, Graphite-compatible CLI that breaks large changes into small, reviewable PRs that stay in sync automatically. This npm package downloads the correct prebuilt binary for your platform (macOS, Linux, Windows) and installs it as the `gt` command.

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
|----------------|---------------|
| One large PR with many changes | Multiple small, focused PRs |
| Reviewers overwhelmed | Easy to review incrementally |
| All-or-nothing merging | Land changes as they're approved |
| Blocked waiting for review | Unblock yourself, keep coding |

## Features

- **Auto-restack** — Edit any branch, all dependents rebase automatically
- **GitHub + GitLab** — Full support for both platforms, including self-hosted GitLab
- **Stack-aware submit** — One command creates PRs for your entire stack
- **Interactive TUI** — Keyboard-driven terminal UI for browsing stacks
- **MCP Server** — AI agent integration via Model Context Protocol
- **Undo/redo** — Recover from mistakes with `gt undo`
- **Graphite compatible** — Drop-in replacement for `gt` commands

## Installation

### Global install

```bash
npm install -g stkd-cli
```

### With npx (no install)

```bash
npx stkd-cli log
npx stkd-cli submit --stack
```

### Local install (for Node.js projects)

```bash
npm install --save-dev stkd-cli
npx gt init
```

## Quick Start

```bash
# Initialize in your repository
gt init

# Authenticate with GitHub or GitLab
gt auth --github

# Create a stack
gt create feature/step-1
gt create feature/step-2
gt create feature/step-3

# Submit all as PRs
gt submit --stack
```

## Platform Support

- macOS (Intel & Apple Silicon)
- Linux (x86_64 & aarch64)
- Windows (x86_64)

If a prebuilt binary is not available for your platform, the installer falls back to building from source with cargo.

## Node.js API

```js
const { run, spawn, getBinaryPath } = require('stkd-cli');

// Run a command and get output
const output = run(['log', '--json']);
console.log(JSON.parse(output));

// Spawn interactively
spawn(['sync']);

// Get binary path
console.log(getBinaryPath());
```

## Documentation

Full documentation at **[docs.neullabs.com/stkd](https://docs.neullabs.com/stkd)**.

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.

Built by [Neul Labs](https://neullabs.com).
