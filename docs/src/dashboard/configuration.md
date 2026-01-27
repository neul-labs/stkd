# Dashboard Configuration

Complete reference for Stack dashboard configuration options.

## Environment Variables

### Database

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | Database connection string | `sqlite:./data/stack.db` |
| `STACK_DB_BACKEND` | Database backend: `sqlite` or `postgres` | `sqlite` |

### Server

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Server bind address | `0.0.0.0` |
| `PORT` | Server port | `3000` |
| `BASE_URL` | Public URL for callbacks | `http://localhost:3000` |

### Authentication

| Variable | Description | Required |
|----------|-------------|----------|
| `JWT_SECRET` | Secret for signing JWTs | Yes |
| `JWT_EXPIRY` | Token expiry in seconds | `604800` (7 days) |

### GitHub OAuth

| Variable | Description | Required |
|----------|-------------|----------|
| `GITHUB_CLIENT_ID` | GitHub OAuth app client ID | For GitHub login |
| `GITHUB_CLIENT_SECRET` | GitHub OAuth app secret | For GitHub login |

### GitLab OAuth

| Variable | Description | Required |
|----------|-------------|----------|
| `GITLAB_CLIENT_ID` | GitLab OAuth application ID | For GitLab login |
| `GITLAB_CLIENT_SECRET` | GitLab OAuth application secret | For GitLab login |
| `GITLAB_URL` | GitLab instance URL | `https://gitlab.com` |

### Frontend

| Variable | Description | Default |
|----------|-------------|---------|
| `VITE_API_URL` | Backend API URL | `http://localhost:3000` |

## Example Configurations

### Development (SQLite)

```bash
# .env
DATABASE_URL=sqlite:./data/stack.db
STACK_DB_BACKEND=sqlite
HOST=127.0.0.1
PORT=3000
BASE_URL=http://localhost:3000
JWT_SECRET=dev-secret-change-in-production
GITHUB_CLIENT_ID=Iv1.xxxxxxxxxxxx
GITHUB_CLIENT_SECRET=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Production (PostgreSQL)

```bash
# .env
DATABASE_URL=postgres://stack:password@db.example.com:5432/stack
STACK_DB_BACKEND=postgres
HOST=0.0.0.0
PORT=3000
BASE_URL=https://stack.example.com
JWT_SECRET=your-very-long-random-secret-key-here
GITHUB_CLIENT_ID=Iv1.xxxxxxxxxxxx
GITHUB_CLIENT_SECRET=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
GITLAB_CLIENT_ID=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
GITLAB_CLIENT_SECRET=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
GITLAB_URL=https://gitlab.example.com
```

## Database Schema

The database stores:

| Table | Purpose |
|-------|---------|
| `organizations` | Teams/organizations |
| `users` | User accounts |
| `memberships` | User-org relationships |
| `repositories` | Connected repos |
| `branches` | Tracked branches |
| `merge_requests` | PR metadata |
| `sessions` | Login sessions |

## Security Considerations

### JWT Secret

Generate a strong random secret:

```bash
openssl rand -base64 32
```

### OAuth Redirect URIs

In production, always use HTTPS for callback URLs:

```
https://stack.example.com/api/auth/oauth/github/callback
https://stack.example.com/api/auth/oauth/gitlab/callback
```

### Database

- Use strong passwords for PostgreSQL
- Enable SSL for database connections in production
- Consider connection pooling for high traffic

## Feature Flags

Currently, features are controlled via environment:

| Feature | Control |
|---------|---------|
| GitHub login | Set `GITHUB_CLIENT_ID` |
| GitLab login | Set `GITLAB_CLIENT_ID` |
| PostgreSQL | Set `STACK_DB_BACKEND=postgres` |

## Logging

Control log output with `RUST_LOG`:

```bash
# Verbose
RUST_LOG=debug ./stkd-server

# Quiet
RUST_LOG=warn ./stkd-server

# Specific modules
RUST_LOG=stkd_server=debug,tower_http=info ./stkd-server
```
