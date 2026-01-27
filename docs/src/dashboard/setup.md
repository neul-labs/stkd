# Dashboard Setup

This guide walks you through setting up the Stack web dashboard.

## Prerequisites

- Rust 1.70+ (for the backend)
- Node.js 18+ (for the frontend)
- SQLite or PostgreSQL

## Backend Setup

### 1. Build the Server

```bash
# From the repository root
cargo build --release --bin stkd-server

# The binary is at target/release/stkd-server
```

### 2. Configure Environment

Create a `.env` file:

```bash
# Database (SQLite for development)
DATABASE_URL=sqlite:./data/stack.db
STACK_DB_BACKEND=sqlite

# Or PostgreSQL for production
# DATABASE_URL=postgres://user:password@localhost/stack
# STACK_DB_BACKEND=postgres

# Server
HOST=0.0.0.0
PORT=3000
BASE_URL=http://localhost:3000

# JWT
JWT_SECRET=your-secret-key-at-least-32-chars

# GitHub OAuth
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret

# GitLab OAuth (optional)
GITLAB_CLIENT_ID=your-gitlab-client-id
GITLAB_CLIENT_SECRET=your-gitlab-client-secret
GITLAB_URL=https://gitlab.com  # or self-hosted URL
```

### 3. Create OAuth Apps

#### GitHub

1. Go to [GitHub Developer Settings](https://github.com/settings/developers)
2. Click "New OAuth App"
3. Set:
   - Application name: `Stack Dashboard`
   - Homepage URL: `http://localhost:3000`
   - Authorization callback URL: `http://localhost:3000/api/auth/oauth/github/callback`
4. Copy Client ID and Client Secret

#### GitLab

1. Go to GitLab → Preferences → Applications
2. Create new application with:
   - Name: `Stack Dashboard`
   - Redirect URI: `http://localhost:3000/api/auth/oauth/gitlab/callback`
   - Scopes: `read_user`, `api`
3. Copy Application ID and Secret

### 4. Run the Server

```bash
# Run with SQLite
./target/release/stkd-server

# Or run in development
cargo run --bin stkd-server

# Server starts at http://localhost:3000
```

## Frontend Setup

### 1. Install Dependencies

```bash
cd web
npm install
```

### 2. Configure API URL

Create `.env.local`:

```bash
VITE_API_URL=http://localhost:3000
```

### 3. Run Development Server

```bash
npm run dev

# Frontend starts at http://localhost:5173
```

### 4. Build for Production

```bash
npm run build

# Output is in web/dist/
```

## Database Setup

### SQLite (Development)

SQLite is the default for development. The database file is created automatically.

```bash
# Database file location
./data/stack.db
```

### PostgreSQL (Production)

For production, use PostgreSQL:

```bash
# Create database
createdb stack

# Set environment variable
export DATABASE_URL=postgres://user:password@localhost/stack
export STACK_DB_BACKEND=postgres

# Run migrations (automatic on server start)
./target/release/stkd-server
```

## Verify Installation

1. Open http://localhost:5173
2. Click "Login with GitHub"
3. Authorize the OAuth app
4. You should see the dashboard

## Troubleshooting

### "Invalid OAuth callback"

Check that your OAuth app's callback URL matches exactly:
- GitHub: `http://localhost:3000/api/auth/oauth/github/callback`
- GitLab: `http://localhost:3000/api/auth/oauth/gitlab/callback`

### "Database connection failed"

Verify your `DATABASE_URL` is correct and the database exists.

### "CORS errors"

Make sure the frontend is configured to point to the correct backend URL.

## Next Steps

- [Configuration](./configuration.md) - Detailed configuration options
- [Deployment](./deployment.md) - Deploy to production
