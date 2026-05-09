# stkd-server

[![crates.io](https://img.shields.io/crates/v/stkd-server.svg)](https://crates.io/crates/stkd-server)
[![docs.rs](https://docs.rs/stkd-server/badge.svg)](https://docs.rs/stkd-server)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Web API server for the Stack dashboard** — provides REST API and WebSocket support for managing stacks, repositories, and organizations.

`stkd-server` runs the Stack web server, offering a REST API for managing organizations and stacks, OAuth authentication with GitHub and GitLab, real-time WebSocket updates, and webhook handlers for provider events.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. `stkd-server` provides the backend for the self-hosted Stack dashboard, a web UI for visualizing and managing stacks.

## Features

- **REST API** for organizations, repositories, and stacks
- **OAuth authentication** with GitHub and GitLab
- **WebSocket** real-time updates
- **Webhook handlers** for provider events
- **SQLite / PostgreSQL** via `stkd-db`
- **Self-hosted** — Run your own Stack dashboard

## Installation

```bash
cargo add stkd-server
```

## Usage

```bash
# Run the server
cargo run -p stkd-server

# With configuration
STKD_PORT=8080 cargo run -p stkd-server
```

## Related Crates

- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary
- [`stkd-db`](https://crates.io/crates/stkd-db) — Database layer
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
