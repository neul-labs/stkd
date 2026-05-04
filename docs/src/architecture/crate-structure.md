# Crate Structure

This page details the structure and contents of each crate in the Stack workspace.

## Workspace Layout

```
Cargo.toml           # Workspace definition
crates/
├── stkd-core/       # Core library: Repository, Stack, DAG
├── stkd-provider-api/  # Provider trait definitions
├── stkd-github/     # GitHub implementation
├── stkd-gitlab/     # GitLab implementation
├── stkd-engine/     # Programmatic engine: reusable library functions
├── stkd-mcp/        # MCP server for AI agent integration
├── stkd-db/         # Database abstraction (SQLite/PostgreSQL)
├── stkd-server/     # Web dashboard API server
└── stkd-cli/        # CLI application (gt binary)

web/                 # Vue 3 + TailwindCSS frontend
```

## stkd-core

Core library for git operations and stack management.

```
stkd-core/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── repository.rs    # Git repository wrapper
    ├── branch.rs        # Branch metadata
    ├── stack.rs         # Stack representation
    ├── dag.rs           # Branch dependency graph
    ├── config.rs        # Configuration
    ├── storage.rs       # Data persistence
    ├── rebase.rs        # Rebase/restack operations
    └── error.rs         # Error types
```

### Key Types

```rust
pub struct Repository {
    git: git2::Repository,
    config: StackConfig,
    storage: Storage,
}

impl Repository {
    pub fn open(path: &str) -> Result<Self>;
    pub fn current_branch(&self) -> Result<Option<String>>;
    pub fn checkout(&self, branch: &str) -> Result<()>;
    pub fn load_graph(&self) -> Result<BranchGraph>;
}
```

## stkd-provider-api

Provider-agnostic traits and types.

```
stkd-provider-api/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── traits.rs        # Provider traits
    ├── types.rs         # Common types (MergeRequest, Pipeline, etc.)
    ├── auth.rs          # Credential management
    └── error.rs         # Provider errors
```

### Core Traits

```rust
#[async_trait]
pub trait MergeRequestProvider: Send + Sync {
    async fn create_mr(&self, repo: &RepoId, request: CreateMergeRequest)
        -> ProviderResult<MergeRequest>;
    async fn update_mr(&self, repo: &RepoId, id: MergeRequestId, update: UpdateMergeRequest)
        -> ProviderResult<MergeRequest>;
    async fn merge_mr(&self, repo: &RepoId, id: MergeRequestId, method: MergeMethod)
        -> ProviderResult<MergeResult>;
}

pub trait Provider: MergeRequestProvider + UserProvider + RepositoryProvider {
    fn name(&self) -> &'static str;
    fn capabilities(&self) -> ProviderCapabilities;
}
```

## stkd-github

GitHub API implementation.

```
stkd-github/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── provider.rs      # GitHubProvider
    ├── api.rs           # HTTP client
    ├── auth.rs          # GitHub auth
    ├── oauth.rs         # Device flow / OAuth
    ├── pr.rs            # PR operations
    └── sync.rs          # Remote sync
```

## stkd-gitlab

GitLab API implementation.

```
stkd-gitlab/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── provider.rs      # GitLabProvider
    ├── api.rs           # HTTP client
    ├── auth.rs          # GitLab auth
    ├── mr.rs            # MR operations
    └── sync.rs          # Remote sync
```

## stkd-db

Database abstraction layer supporting SQLite and PostgreSQL.

```
stkd-db/
├── Cargo.toml
└── src/
    ├── lib.rs              # Public API
    ├── error.rs            # DbError, DbResult
    ├── config.rs           # DatabaseConfig, DatabaseBackend
    ├── pool.rs             # DatabasePool trait
    ├── models/             # Entity definitions
    │   ├── mod.rs
    │   ├── organization.rs
    │   ├── user.rs
    │   ├── membership.rs
    │   ├── repository.rs
    │   ├── branch.rs
    │   ├── merge_request.rs
    │   └── session.rs
    ├── repositories/       # Repository traits (CRUD)
    │   ├── mod.rs
    │   ├── organization.rs
    │   ├── user.rs
    │   └── ...
    ├── sqlite/             # SQLite implementation
    │   ├── mod.rs
    │   ├── pool.rs
    │   └── repositories/
    └── postgres/           # PostgreSQL implementation
        ├── mod.rs
        ├── pool.rs
        └── repositories/
```

### Feature Flags

```toml
[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite"]
postgres = ["sqlx/postgres"]
```

### Key Types

```rust
pub trait DatabasePool: Send + Sync {
    fn organizations(&self) -> &dyn OrganizationRepository;
    fn users(&self) -> &dyn UserRepository;
    fn sessions(&self) -> &dyn SessionRepository;
    async fn migrate(&self) -> DbResult<()>;
}

pub async fn create_pool(config: &DatabaseConfig) -> DbResult<Box<dyn DatabasePool>>;
```

## stkd-server

Axum-based web API server for the dashboard.

```
stkd-server/
├── Cargo.toml
└── src/
    ├── lib.rs              # Server setup
    ├── config.rs           # ServerConfig
    ├── state.rs            # AppState
    ├── error.rs            # ApiError
    ├── auth/
    │   ├── mod.rs
    │   ├── jwt.rs          # JWT handling
    │   ├── middleware.rs   # Auth extractors
    │   └── oauth/
    │       ├── github.rs   # GitHub OAuth flow
    │       └── gitlab.rs   # GitLab OAuth flow
    ├── api/
    │   ├── mod.rs
    │   ├── routes.rs       # Route definitions
    │   ├── auth.rs         # /api/auth/*
    │   ├── orgs.rs         # /api/orgs/*
    │   ├── repos.rs        # /api/repos/*
    │   └── webhooks.rs     # Webhook handlers
    └── ws/
        ├── mod.rs
        ├── hub.rs          # Connection hub
        └── messages.rs     # Message types
```

### API Endpoints

```
POST /api/auth/oauth/:provider/start    # Start OAuth
GET  /api/auth/oauth/:provider/callback # OAuth callback
POST /api/auth/logout                   # Logout
GET  /api/auth/me                       # Current user

GET    /api/orgs                        # List user's orgs
POST   /api/orgs                        # Create org
GET    /api/orgs/:slug                  # Get org
PATCH  /api/orgs/:slug                  # Update org
DELETE /api/orgs/:slug                  # Delete org

GET    /api/orgs/:slug/repos            # List repos
POST   /api/orgs/:slug/repos            # Connect repo
POST   /api/repos/:id/sync              # Trigger sync

GET    /api/repos/:id/stacks            # List stacks

POST   /api/webhooks/github             # GitHub events
POST   /api/webhooks/gitlab             # GitLab events
```

## stkd-engine

Programmatic engine exposing all `gt` operations as reusable async functions.

```
stkd-engine/
├── Cargo.toml
└── src/
    ├── lib.rs              # Public API
    ├── init.rs             # Repository initialization
    ├── submit.rs           # MR/PR submission
    ├── sync.rs             # Remote sync and restack
    ├── land.rs             # Merge and cleanup
    ├── restack.rs          # Rebase operations
    ├── retry.rs            # Provider retry with backoff
    └── provider.rs         # Provider auto-detection
```

### Key Functions

```rust
pub async fn submit(
    repo: &Repository,
    opts: SubmitOptions,
    provider: &dyn Provider,
    repo_id: &RepoId,
) -> Result<SubmitResult>;

pub async fn sync(
    repo: &Repository,
    opts: SyncOptions,
    provider: Option<&dyn Provider>,
    repo_id: Option<&RepoId>,
) -> Result<SyncResult>;
```

## stkd-mcp

MCP server exposing Stack operations as Model Context Protocol tools.

```
stkd-mcp/
├── Cargo.toml
└── src/
    └── main.rs             # stdio MCP server
```

### Available Tools

- `gt_init` — Initialize Stack in a repository
- `gt_create` — Create a new stacked branch
- `gt_log` — Show the current stack
- `gt_status` — Show branch and MR status
- `gt_submit` — Submit PRs/MRs
- `gt_sync` — Sync with remote
- `gt_land` — Merge the stack
- `gt_restack` — Restack branches
- `gt_track` — Track an existing branch
- `gt_delete` — Delete a tracked branch

## stkd-cli

Command-line interface.

```
stkd-cli/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── output.rs           # Terminal formatting
    ├── provider_context.rs # Provider resolution
    └── commands/
        ├── mod.rs
        ├── init.rs
        ├── create.rs
        ├── submit.rs
        ├── sync.rs
        ├── land.rs
        ├── log.rs
        ├── status.rs
        ├── nav.rs
        ├── squash.rs
        ├── fold.rs
        ├── split.rs
        ├── auth.rs
        └── completions.rs
```

## Dependency Graph

```
                    ┌──────────────────┐
                    │    stkd-cli     │
                    └────────┬─────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  stkd-core   │  │  stkd-engine   │  │  stkd-mcp      │
└───────────────┘  └────────┬────────┘  └────────┬────────┘
        ▲                   │                    │
        │                   ▼                    │
        │         ┌─────────────────┐            │
        │         │  stkd-github   │            │
        │         └────────┬────────┘            │
        │                  │                     │
        │         ┌────────┴────────┐            │
        │         │  stkd-gitlab   │            │
        │         └────────┬────────┘            │
        │                  │                     │
        │                  ▼                     │
        │         ┌─────────────────┐            │
        └─────────┤ stkd-provider-api │◄───────────┘
                  └─────────────────┘

                    ┌──────────────────┐
                    │   stkd-server   │
                    └────────┬─────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   stkd-db    │  │  stkd-github   │  │  stkd-gitlab   │
└───────────────┘  └─────────────────┘  └─────────────────┘
```

## Feature Flags

```toml
# stkd-cli Cargo.toml
[features]
default = ["github"]
github = ["stkd-github"]
gitlab = ["stkd-gitlab"]
all-providers = ["github", "gitlab"]

# stkd-db Cargo.toml
[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite"]
postgres = ["sqlx/postgres"]
```

## Web Frontend

The `web/` directory contains the Vue 3 frontend:

```
web/
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── src/
    ├── main.ts
    ├── App.vue
    ├── router/index.ts
    ├── stores/           # Pinia stores
    │   ├── auth.ts
    │   ├── organization.ts
    │   └── repositories.ts
    ├── api/              # API client
    ├── components/       # UI components
    │   ├── common/
    │   ├── stack/        # StackTree, StackNode
    │   └── org/
    └── views/            # Page components
```
