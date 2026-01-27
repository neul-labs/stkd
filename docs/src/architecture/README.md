# Architecture

This section covers Stack's internal architecture for contributors and curious users.

## High-Level Overview

Stack is composed of several Rust crates:

```
stack/
├── crates/
│   ├── stkd-core/       # Core logic, no network
│   ├── stkd-provider-api/  # Provider trait definitions
│   ├── stkd-github/     # GitHub implementation
│   ├── stkd-gitlab/     # GitLab implementation
│   └── stkd-cli/        # Command-line interface
└── docs/                 # This documentation
```

## Crate Responsibilities

### stkd-core

The foundation crate containing:

- Repository abstraction over libgit2
- Branch metadata and relationships
- Stack representation and operations
- Configuration management
- Storage layer for branch data

**Key principle**: This crate has no network dependencies.

### stkd-provider-api

Defines the interface that all providers must implement:

- Trait definitions (`MergeRequestProvider`, etc.)
- Provider-agnostic types (`MergeRequest`, `Pipeline`, etc.)
- Error types (`ProviderError`)
- Authentication abstractions

### stkd-github

GitHub-specific implementation:

- `GitHubProvider` implementing all provider traits
- GitHub API client
- OAuth device flow
- GitHub-specific response parsing

### stkd-gitlab

GitLab-specific implementation:

- `GitLabProvider` implementing all provider traits
- GitLab API client
- PAT authentication
- GitLab-specific response parsing

### stkd-cli

The user-facing command-line interface:

- Command parsing (using clap)
- User interaction and output formatting
- Provider selection and configuration
- Integration of core + provider crates

## Data Flow

```
User Input
    │
    ▼
┌─────────┐
│  CLI    │ ─── parses commands, handles I/O
└────┬────┘
     │
     ▼
┌─────────┐
│  Core   │ ─── manipulates branches, stores metadata
└────┬────┘
     │
     ▼
┌──────────┐
│ Provider │ ─── API calls to GitHub/GitLab
└──────────┘
```

## Learn More

- [Crate Structure](./crate-structure.md) - Detailed crate breakdown
- [Provider System](./provider-system.md) - How providers work
- [Data Storage](./storage.md) - Where data is stored
