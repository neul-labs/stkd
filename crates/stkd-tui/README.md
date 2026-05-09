# stkd-tui

[![crates.io](https://img.shields.io/crates/v/stkd-tui.svg)](https://crates.io/crates/stkd-tui)
[![docs.rs](https://docs.rs/stkd-tui/badge.svg)](https://docs.rs/stkd-tui)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Terminal UI for Stack** — interactive stacked diffs browser built with Ratatui.

`stkd-tui` provides the keyboard-driven terminal UI used by the `gt tui` command. It renders stack visualizations, branch status, PR/MR metadata, and navigation controls in a rich terminal interface.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. It breaks large changes into small, reviewable PRs that stay in sync automatically.

## Features

- **Stack visualization** — Tree view of stacked branches with status indicators
- **Keyboard navigation** — Vim-style keybindings for fast navigation
- **Live status** — Real-time PR/MR status from GitHub and GitLab
- **Mini mode** — Compact view for narrow terminals
- **Customizable themes** — Light and dark themes

## Installation

```bash
cargo add stkd-tui
```

## Usage

This crate is typically used via the `gt tui` command in `stkd-cli`:

```bash
gt tui
```

To use programmatically:

```rust
use stkd_tui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(".")?;
    app.run()?;
    Ok(())
}
```

## Related Crates

- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
