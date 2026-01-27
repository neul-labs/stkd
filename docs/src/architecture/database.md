# Database Layer

The `stkd-db` crate provides a database abstraction layer that supports both SQLite and PostgreSQL.

## Architecture

```
┌─────────────────────────────────────────────┐
│              Application Code               │
└──────────────────────┬──────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────┐
│           DatabasePool Trait                │
│  (organizations, users, sessions, etc.)     │
└──────────────────────┬──────────────────────┘
                       │
          ┌────────────┴────────────┐
          │                         │
          ▼                         ▼
┌──────────────────┐      ┌──────────────────┐
│   SqlitePool     │      │  PostgresPool    │
└──────────────────┘      └──────────────────┘
```

## Feature Flags

```toml
[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite"]
postgres = ["sqlx/postgres"]
```

Build with specific backend:

```bash
# SQLite only (default)
cargo build -p stkd-db

# PostgreSQL only
cargo build -p stkd-db --no-default-features --features postgres

# Both
cargo build -p stkd-db --features postgres
```

## Data Models

### Organization

Multi-tenant container for teams.

```rust
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,          // URL-safe identifier
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### User

User account linked to OAuth provider.

```rust
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub provider: String,      // "github" or "gitlab"
    pub provider_id: String,   // ID from provider
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Membership

User's role in an organization.

```rust
pub struct Membership {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,      // Owner, Admin, Member
    pub joined_at: DateTime<Utc>,
}

pub enum MemberRole {
    Owner,
    Admin,
    Member,
}
```

### Repository

Connected Git repository.

```rust
pub struct Repository {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub provider: String,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub default_branch: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Branch

Tracked branch in a stack.

```rust
pub struct Branch {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub parent_name: Option<String>,
    pub merge_request_id: Option<u64>,
    pub status: BranchStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### MergeRequest

PR/MR metadata.

```rust
pub struct MergeRequest {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub number: u64,
    pub title: String,
    pub state: MergeRequestState,
    pub web_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Session

User login session.

```rust
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

## Repository Pattern

Each entity has a repository trait:

```rust
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn create(&self, org: &Organization) -> DbResult<Organization>;
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Organization>>;
    async fn get_by_slug(&self, slug: &str) -> DbResult<Option<Organization>>;
    async fn list_for_user(&self, user_id: Uuid) -> DbResult<Vec<Organization>>;
    async fn update(&self, org: &Organization) -> DbResult<Organization>;
    async fn delete(&self, id: Uuid) -> DbResult<()>;
}
```

Access repositories through the pool:

```rust
let pool = create_pool(&config).await?;

// Get organizations for a user
let orgs = pool.organizations().list_for_user(user_id).await?;

// Create a new session
let session = Session::new(user_id, token);
pool.sessions().create(&session).await?;
```

## Migrations

Migrations are embedded in the binary and run automatically on startup.

### SQLite

```sql
-- migrations/sqlite/001_initial.sql

CREATE TABLE organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT,
    avatar_url TEXT,
    provider TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(provider, provider_id)
);

-- ... more tables
```

### PostgreSQL

```sql
-- migrations/postgres/001_initial.sql

CREATE TABLE organizations (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT,
    avatar_url TEXT,
    provider TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_id)
);

-- ... more tables
```

## Usage

```rust
use stack_db::{create_pool, DatabaseConfig, DatabaseBackend};

// Create configuration
let config = DatabaseConfig {
    url: "sqlite:./data/stack.db".to_string(),
    backend: DatabaseBackend::SQLite,
};

// Create pool
let pool = create_pool(&config).await?;

// Run migrations
pool.migrate().await?;

// Use repositories
let user = pool.users().get_by_id(user_id).await?;
```

## Error Handling

```rust
pub enum DbError {
    NotFound,
    Duplicate,
    Connection(String),
    Query(String),
    Migration(String),
}

pub type DbResult<T> = Result<T, DbError>;
```
