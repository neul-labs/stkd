# stkd-cli

[![crates.io](https://img.shields.io/crates/v/stkd-cli)](https://crates.io/crates/stkd-cli)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--cli-blue)](https://docs.rs/stkd-cli)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

CLI for [Stack](https://github.com/neul-labs/stkd) — stacked diffs for Git.

This crate provides the `gt` binary, a command-line tool for managing stacked pull requests on GitHub and GitLab. It handles branch creation, stack visualization, restacking, MR/PR submission, and landing.

## Installation

### From crates.io

```bash
cargo install stkd-cli
```

### From source

```bash
git clone https://github.com/neul-labs/stkd
cd stkd
cargo install --path crates/stkd-cli
```

### Prebuilt binaries

Download from [GitHub Releases](https://github.com/neul-labs/stkd/releases).

## Quick Start

```bash
# Initialize in your repository
gt init

# Create a stack
gt create feature/step-1
# ... make changes, commit ...

gt create feature/step-2
# ... make changes, commit ...

# Submit all as PRs
gt submit --stack
```

## Commands

| Command | Description |
|---------|-------------|
| `gt init` | Initialize Stack in this repository |
| `gt create <name>` | Create a branch stacked on current |
| `gt track <branch>` | Start tracking an existing branch |
| `gt log` | Visualize the stack |
| `gt up` / `gt down` | Navigate between branches |
| `gt restack` | Rebase all dependent branches |
| `gt submit --stack` | Create/update PRs for the stack |
| `gt sync` | Fetch, restack, clean merged branches |
| `gt land --stack` | Merge the stack in order |

For full documentation, visit [docs.neullabs.com/stkd](https://docs.neullabs.com/stkd).

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
