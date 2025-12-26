# Dashboard Deployment

Deploy the Stack dashboard for production use.

## Deployment Options

1. **Docker Compose** - Recommended for most deployments
2. **Manual** - Full control over infrastructure
3. **Cloud Platforms** - Deploy to Fly.io, Railway, etc.

## Docker Compose

The easiest way to deploy Stack dashboard.

### docker-compose.yml

```yaml
version: '3.8'

services:
  stack-dashboard:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgres://stack:stackpass@db:5432/stack
      - STACK_DB_BACKEND=postgres
      - HOST=0.0.0.0
      - PORT=3000
      - BASE_URL=https://stack.example.com
      - JWT_SECRET=${JWT_SECRET}
      - GITHUB_CLIENT_ID=${GITHUB_CLIENT_ID}
      - GITHUB_CLIENT_SECRET=${GITHUB_CLIENT_SECRET}
    depends_on:
      - db

  db:
    image: postgres:15-alpine
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_USER=stack
      - POSTGRES_PASSWORD=stackpass
      - POSTGRES_DB=stack

volumes:
  postgres_data:
```

### Dockerfile

```dockerfile
# Build backend
FROM rust:1.75-slim as backend-builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin stack-server

# Build frontend
FROM node:20-alpine as frontend-builder
WORKDIR /app/web
COPY web/package*.json ./
RUN npm ci
COPY web .
RUN npm run build

# Runtime
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=backend-builder /app/target/release/stack-server .
COPY --from=frontend-builder /app/web/dist ./static

EXPOSE 3000
CMD ["./stack-server"]
```

### Deploy

```bash
# Create .env with secrets
echo "JWT_SECRET=$(openssl rand -base64 32)" > .env
echo "GITHUB_CLIENT_ID=xxx" >> .env
echo "GITHUB_CLIENT_SECRET=xxx" >> .env

# Start
docker compose up -d
```

## Manual Deployment

### 1. Build

```bash
# Backend
cargo build --release --bin stack-server

# Frontend
cd web
npm ci
npm run build
```

### 2. Configure Reverse Proxy (nginx)

```nginx
server {
    listen 443 ssl http2;
    server_name stack.example.com;

    ssl_certificate /etc/ssl/certs/stack.crt;
    ssl_certificate_key /etc/ssl/private/stack.key;

    # Frontend static files
    location / {
        root /var/www/stack/static;
        try_files $uri $uri/ /index.html;
    }

    # API
    location /api {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # WebSocket
    location /ws {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### 3. Create Systemd Service

```ini
# /etc/systemd/system/stack-server.service
[Unit]
Description=Stack Dashboard Server
After=network.target postgresql.service

[Service]
Type=simple
User=stack
WorkingDirectory=/opt/stack
EnvironmentFile=/opt/stack/.env
ExecStart=/opt/stack/stack-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl enable stack-server
sudo systemctl start stack-server
```

## Cloud Platforms

### Fly.io

```toml
# fly.toml
app = "stack-dashboard"

[build]
  dockerfile = "Dockerfile"

[env]
  PORT = "8080"

[[services]]
  internal_port = 8080
  protocol = "tcp"

  [[services.ports]]
    port = 443
    handlers = ["tls", "http"]
```

```bash
fly launch
fly secrets set JWT_SECRET=$(openssl rand -base64 32)
fly secrets set GITHUB_CLIENT_ID=xxx
fly secrets set GITHUB_CLIENT_SECRET=xxx
fly deploy
```

### Railway

1. Connect your GitHub repository
2. Add environment variables in dashboard
3. Deploy

## Self-Hosted SQLite

For simple deployments, SQLite works well:

```yaml
# docker-compose.yml (SQLite version)
services:
  stack-dashboard:
    image: stack/dashboard:latest
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=sqlite:///data/stack.db
      - STACK_DB_BACKEND=sqlite
    volumes:
      - stack_data:/data

volumes:
  stack_data:
```

## Webhooks Setup

For real-time updates, configure webhooks:

### GitHub

1. Go to Repository → Settings → Webhooks
2. Add webhook:
   - URL: `https://stack.example.com/api/webhooks/github`
   - Content type: `application/json`
   - Events: Pull requests, Push, Status
3. Save

### GitLab

1. Go to Settings → Webhooks
2. Add webhook:
   - URL: `https://stack.example.com/api/webhooks/gitlab`
   - Trigger: Merge request events, Push events, Pipeline events
3. Save

## Health Checks

The server exposes a health endpoint:

```bash
curl https://stack.example.com/api/health
# {"status":"ok"}
```

## Backups

### PostgreSQL

```bash
# Backup
pg_dump -U stack stack > backup.sql

# Restore
psql -U stack stack < backup.sql
```

### SQLite

```bash
# Backup
cp /data/stack.db /backups/stack-$(date +%Y%m%d).db
```

## Monitoring

### Logs

```bash
# Docker
docker compose logs -f stack-dashboard

# Systemd
journalctl -u stack-server -f
```

### Metrics

The server can expose Prometheus metrics (planned feature).
