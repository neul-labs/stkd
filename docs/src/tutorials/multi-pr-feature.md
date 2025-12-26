# Multi-PR Feature Tutorial

This tutorial shows how to break down a large feature into reviewable pieces.

## Scenario

You're implementing a new notifications system that includes:

1. Database schema and models
2. Backend API endpoints
3. Real-time WebSocket support
4. Frontend UI components
5. Email integration

This is too much for one PR. Let's stack it.

## Planning the Stack

Before coding, plan your stack:

```
main
 └── notifications/models        # DB + models
      └── notifications/api      # REST endpoints
           └── notifications/ws  # WebSocket
                └── notifications/ui    # Frontend
                     └── notifications/email  # Email
```

Each branch should:
- Be independently reviewable
- Pass all tests
- Not break the build if landed alone

## Implementation

### Layer 1: Database

```bash
git checkout main && git pull
gt create notifications/models
```

```bash
# Add migrations
cat > migrations/001_notifications.sql << 'EOF'
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    message TEXT NOT NULL,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);
EOF

# Add model
cat > src/models/notification.rs << 'EOF'
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub message: String,
    pub read: bool,
}
EOF

git add .
git commit -m "Add notifications table and model"
```

### Layer 2: API

```bash
gt create notifications/api
```

```bash
# Add endpoints
cat > src/api/notifications.rs << 'EOF'
pub async fn list_notifications(user_id: i32) -> Vec<Notification> {
    // Implementation
}

pub async fn mark_read(id: i32) -> Result<()> {
    // Implementation
}
EOF

git add .
git commit -m "Add notification API endpoints"
```

### Layer 3: WebSocket

```bash
gt create notifications/ws
```

```bash
# Add WebSocket handler
cat > src/ws/notifications.rs << 'EOF'
pub async fn handle_connection(socket: WebSocket, user_id: i32) {
    // Real-time notification delivery
}
EOF

git add .
git commit -m "Add WebSocket notification support"
```

### Layer 4: Frontend

```bash
gt create notifications/ui
```

```bash
# Add React components
cat > src/components/NotificationBell.tsx << 'EOF'
export function NotificationBell() {
    const { notifications, markRead } = useNotifications();
    return (
        <Dropdown>
            {notifications.map(n => (
                <NotificationItem key={n.id} {...n} />
            ))}
        </Dropdown>
    );
}
EOF

git add .
git commit -m "Add notification UI components"
```

### Layer 5: Email

```bash
gt create notifications/email
```

```bash
# Add email templates
cat > src/email/notification.html << 'EOF'
<h1>New Notification</h1>
<p>{{ message }}</p>
EOF

git add .
git commit -m "Add email notification support"
```

## View the Complete Stack

```bash
gt log

# Output:
# ┌ ○ notifications/models [active]
# │ ○ notifications/api [active]
# │ ○ notifications/ws [active]
# │ ○ notifications/ui [active]
# └ ◉ notifications/email [active]
```

## Submit All

```bash
gt submit --stack --draft

# Creates 5 draft PRs with proper dependencies
```

## Parallel Development

While waiting for review on models, you can continue:

```bash
# You're still on notifications/email
# Continue adding features

git add .
git commit -m "Add email preferences"
gt submit
```

## Handling Feedback

When PR #1 (models) needs changes:

```bash
# Navigate down to models
gt bottom
# Or: git checkout notifications/models

# Make changes
vim src/models/notification.rs
git add .
git commit -m "Add notification type field"

# Push
gt submit

# Restack everything above
gt top
gt sync
gt submit --stack
```

## Parallel Review

PRs can be reviewed in parallel:
- Frontend team reviews notifications/ui
- Backend team reviews notifications/api
- DBA reviews notifications/models

But they must be **merged in order**:

1. notifications/models
2. notifications/api
3. notifications/ws
4. notifications/ui
5. notifications/email

## Landing Strategy

### Option 1: Land One at a Time

As each PR is approved:

```bash
git checkout notifications/models
gt land
# Wait for next approval...
```

### Option 2: Land All When Ready

When all are approved:

```bash
gt land --stack
```

## Best Practices for Multi-PR Features

1. **Keep layers thin**: Each PR should be < 400 lines
2. **Add tests at each layer**: Don't defer all testing to the end
3. **Document as you go**: Add docs with the code
4. **Enable feature flags**: Allow partial deployment
5. **Communicate the plan**: Let reviewers know the full scope

## Example Feature Flag

In the first layer:

```rust
// src/config.rs
pub const NOTIFICATIONS_ENABLED: bool = false;
```

In the last layer:

```rust
// Enable the feature
pub const NOTIFICATIONS_ENABLED: bool = true;
```

This allows merging partial implementations safely.
