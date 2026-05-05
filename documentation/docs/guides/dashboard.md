# Web Dashboard

Stack includes a self-hosted web dashboard for visualizing stacks, managing organizations, and configuring repositories. This guide covers running the server and using the dashboard effectively.

---

## What Is the Dashboard?

The dashboard is a Vue.js web application served by `stkd-server`. It provides:

- **Stack visualization** — Interactive tree views of your stacks
- **Organization management** — Connect teams to repositories
- **Repository overview** — See all stacks across a repo at a glance
- **Settings** — Configure providers, notifications, and defaults

Unlike Graphite's cloud-hosted dashboard, Stack's dashboard runs on your infrastructure.

---

## Running the Server

### Installation

The server is a separate binary from the CLI:

```bash
# Via cargo
cargo install stkd-server

# Or from source
git clone https://github.com/neul-labs/stkd
cd stkd
cargo install --path crates/stkd-server
```

### Starting the Server

```bash
$ stkd-server
Server running at http://localhost:3000
Database: ~/.local/share/stkd/server.db
```

### Configuration

Create `~/.config/stkd/server.toml`:

```toml
[server]
host = "0.0.0.0"
port = 3000

[database]
path = "/var/lib/stkd/server.db"

[auth]
session_timeout = 86400  # 24 hours in seconds

[providers.github]
client_id = "your-oauth-app-id"
client_secret = "your-oauth-app-secret"

[providers.gitlab]
client_id = "your-app-id"
client_secret = "your-app-secret"
base_url = "https://gitlab.company.com"
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `STKD_SERVER_HOST` | Bind address | `127.0.0.1` |
| `STKD_SERVER_PORT` | Port | `3000` |
| `STKD_DATABASE_URL` | SQLite database path | `~/.local/share/stkd/server.db` |
| `STKD_GITHUB_CLIENT_ID` | GitHub OAuth app ID | — |
| `STKD_GITHUB_CLIENT_SECRET` | GitHub OAuth secret | — |
| `STKD_GITLAB_CLIENT_ID` | GitLab app ID | — |
| `STKD_GITLAB_CLIENT_SECRET` | GitLab app secret | — |
| `STKD_JWT_SECRET` | JWT signing key | auto-generated |

### Production Deployment

For production use, run behind a reverse proxy:

```nginx
# /etc/nginx/sites-available/stkd
server {
    listen 443 ssl;
    server_name stkd.company.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    ssl_certificate /etc/letsencrypt/live/stkd.company.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/stkd.company.com/privkey.pem;
}
```

Use systemd to manage the service:

```ini
# /etc/systemd/system/stkd-server.service
[Unit]
Description=Stack Dashboard Server
After=network.target

[Service]
Type=simple
User=stkd
ExecStart=/usr/local/bin/stkd-server
Restart=on-failure
Environment="STKD_DATABASE_URL=/var/lib/stkd/server.db"

[Install]
WantedBy=multi-user.target
```

---

## Dashboard Views

### Dashboard Home

The home page shows an overview of your activity:

```
┌─────────────────────────────────────────────┐
│  Stack Dashboard                            │
├─────────────────────────────────────────────┤
│                                             │
│  Your Stacks                                │
│  ┌─────────────────────────────────────┐   │
│  │ feature/payment-models      #45    │   │
│  │  └── feature/payment-api    #46    │   │
│  │       └── feature/payment-tests #47│   │
│  └─────────────────────────────────────┘   │
│                                             │
│  Recent Activity                            │
│  • Landed PR #42 (feature/auth-models)    │
│  • Submitted PR #48 (feature/webhook-models)│
│  • Bob approved PR #45                    │
│                                             │
│  Quick Actions                              │
│  [Sync All] [Create Stack] [Browse Repos]  │
│                                             │
└─────────────────────────────────────────────┘
```

### Organizations

Organizations group repositories and team members:

```
┌─────────────────────────────────────────────┐
│  Organizations                              │
├─────────────────────────────────────────────┤
│                                             │
│  Neul Labs                                  │
│  ├─ stkd (CLI)                             │
│  ├─ stkd-server (Dashboard)                │
│  └─ stkd-engine (Library)                  │
│                                             │
│  Create Organization                        │
│  [Name] [Create]                           │
│                                             │
└─────────────────────────────────────────────┘
```

To create an organization:

1. Navigate to **Organizations** in the sidebar
2. Click **Create Organization**
3. Enter a name and description
4. Add repositories by URL

### Repositories

The repository view shows all stacks and their status:

```
┌─────────────────────────────────────────────┐
│  stkd                                       │
├─────────────────────────────────────────────┤
│  main                                       │
│  ├── feature/auth-models      #42 [merged]  │
│  │    └── feature/auth-api     #43 [open]   │
│  │         └── feature/auth-ui #44 [open]   │
│  ├── feature/payment-models   #45 [open]    │
│  │    └── feature/payment-api #46 [open]    │
│  └── fix/login-redirect       #47 [draft]   │
│                                             │
│  [Filter: ______] [Status: All ▼]           │
│                                             │
└─────────────────────────────────────────────┘
```

**Filters:**
- By author
- By status (open, merged, closed, draft)
- By label
- By date range

### Stack Detail View

Click any stack to see details:

```
┌─────────────────────────────────────────────┐
│  feature/payment-models                       │
├─────────────────────────────────────────────┤
│  Stack                                      │
│  main → payment-models → payment-api          │
│                                             │
│  PRs:                                       │
│  #45 payment-models   [open] [2 approvals]  │
│  #46 payment-api      [open] [1 approval]   │
│                                             │
│  Actions:                                   │
│  [Submit] [Land] [Sync] [Delete]            │
│                                             │
│  Timeline:                                  │
│  • 2h ago — Alice created branch            │
│  • 1h ago — Alice submitted PR #45          │
│  • 45m ago — Bob approved PR #45            │
│  • 30m ago — Alice submitted PR #46         │
│                                             │
└─────────────────────────────────────────────┘
```

---

## OAuth Setup

The dashboard uses OAuth for authentication. You need to create an OAuth app with your provider.

### GitHub OAuth App

1. Go to **Settings → Developer settings → OAuth Apps → New OAuth App**
2. Set **Authorization callback URL** to `http://localhost:3000/auth/github/callback`
3. Copy **Client ID** and **Client Secret** to `server.toml`

### GitLab OAuth App

1. Go to **Applications** in your GitLab profile or group
2. Create a new application with scopes: `read_user`, `read_repository`, `write_repository`
3. Set **Redirect URI** to `http://localhost:3000/auth/gitlab/callback`
4. Copy **Application ID** and **Secret** to `server.toml`

### Logging In

```bash
# The dashboard redirects to provider login
# After authorization, you're redirected back with a session cookie
```

---

## Stack Visualization in the Web UI

The dashboard renders stacks as interactive trees:

### Visual Elements

- **Trunk** — Root branch (usually `main` or `master`)
- **Nodes** — Branches with PR numbers and status
- **Edges** — Parent-child relationships
- **Colors** — Status indicators (green=merged, blue=open, gray=draft, red=closed)

### Interactions

| Action | Result |
|--------|--------|
| Click node | Open PR detail view |
| Hover node | Show branch info tooltip |
| Drag node | Reorder stack (if permissions allow) |
| Right-click | Context menu (sync, land, delete) |
| Scroll | Zoom in/out on large stacks |

### Large Stack Handling

For stacks with 10+ branches, the UI:
- Collapses branches beyond a depth limit
- Provides a "Expand all" toggle
- Shows summary stats instead of full tree

---

## When to Use Dashboard vs CLI/TUI

| Use Dashboard When... | Use CLI/TUI When... |
|----------------------|---------------------|
| Browsing across multiple repositories | Running quick commands |
| Reviewing team activity | In your terminal workflow |
| Managing organizations | Scripting or automation |
| Visualizing complex stack structures | Limited by SSH access |
| Sharing stack links with stakeholders | Working offline |

---

## Dashboard Tips

1. **Bookmark stacks**: Save frequently accessed stack URLs
2. **Set up notifications**: Configure Slack/Discord webhooks for PR events
3. **Use dark mode**: Toggle in settings for low-light environments
4. **Export data**: Download stack metadata as JSON for reporting
5. **Keyboard shortcuts**: Press `?` in the dashboard for shortcut help

---

## API Access

The dashboard exposes a REST API for integrations:

```bash
# Get stacks for a repository
curl -H "Authorization: Bearer $TOKEN" \
  https://stkd.company.com/api/repos/neul-labs/stkd/stacks

# Get details for a specific stack
curl -H "Authorization: Bearer $TOKEN" \
  https://stkd.company.com/api/stacks/123

# Trigger a sync
curl -X POST -H "Authorization: Bearer $TOKEN" \
  https://stkd.company.com/api/stacks/123/sync
```

See the API reference at `/api/docs` when running the server.
