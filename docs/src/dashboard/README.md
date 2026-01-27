# Web Dashboard

Stack includes an optional web dashboard for visualizing and managing stacks across multiple repositories.

## Overview

The web dashboard provides:

- **Stack Visualization**: See all your stacks in a tree view with PR status
- **Multi-Tenancy**: Organization-based access control for teams
- **Real-Time Updates**: WebSocket-powered live updates when PRs change
- **OAuth Authentication**: Login with GitHub or GitLab
- **Dark Mode**: Full dark mode support

## Quick Start

```bash
# Start the backend server
cargo run --bin stkd-server

# In another terminal, start the frontend dev server
cd web
npm install
npm run dev

# Access at http://localhost:5173
```

## Architecture

```
┌─────────────────┐     ┌─────────────────┐
│   Vue Frontend  │────▶│   Axum Server   │
│  (TailwindCSS)  │◀────│    (REST API)   │
└─────────────────┘     └────────┬────────┘
                                 │
                    ┌────────────┴────────────┐
                    │                         │
              ┌─────▼─────┐           ┌───────▼───────┐
              │  SQLite   │           │  PostgreSQL   │
              │   (dev)   │           │    (prod)     │
              └───────────┘           └───────────────┘
```

## Features

### Stack Tree Visualization

View your branches as an interactive tree:

```
main
└── feature/auth-base (#42)
    └── feature/auth-oauth (#43) ← you are here
        └── feature/auth-tests (#44) [draft]
```

Each branch shows:
- PR number and status (open, merged, closed, draft)
- CI status (passing, failing, pending)
- Review status (approved, changes requested)

### Organization Management

- Create and manage organizations
- Invite team members with role-based access
- Connect repositories from GitHub/GitLab

### Repository Sync

- Connect repositories to your organization
- View all stacks in a repository
- Automatic sync via webhooks

## Pages

| Route | Description |
|-------|-------------|
| `/login` | OAuth login page |
| `/` | Dashboard home |
| `/orgs/:slug` | Organization overview |
| `/orgs/:slug/repos` | Repository list |
| `/orgs/:slug/repos/:id` | Repository stacks |
| `/orgs/:slug/settings` | Organization settings |

## Next Steps

- [Setup](./setup.md) - Install and configure the dashboard
- [Configuration](./configuration.md) - Environment variables and options
- [Deployment](./deployment.md) - Deploy to production
