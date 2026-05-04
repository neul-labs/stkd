# stkd-server

[![crates.io](https://img.shields.io/crates/v/stkd-server)](https://crates.io/crates/stkd-server)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--server-blue)](https://docs.rs/stkd-server)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Web API server for the [Stack](https://github.com/neul-labs/stkd) dashboard — provides REST API and WebSocket support.

This crate runs the Stack web server, offering a REST API for managing organizations and stacks, OAuth authentication with GitHub and GitLab, real-time WebSocket updates, and webhook handlers for provider events.

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

## Features

- **REST API** for organizations, repositories, and stacks
- **OAuth authentication** with GitHub and GitLab
- **WebSocket** real-time updates
- **Webhook handlers** for provider events
- **SQLite / PostgreSQL** via `stkd-db`

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
