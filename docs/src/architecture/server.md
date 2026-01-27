# Web Server Architecture

The `stkd-server` crate provides an Axum-based REST API server for the web dashboard.

## Architecture

```
┌─────────────────────────────────────────────┐
│              HTTP Requests                  │
└──────────────────────┬──────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────┐
│           Axum Router                       │
│  (CORS, Tracing, Compression)               │
└──────────────────────┬──────────────────────┘
                       │
      ┌────────────────┼────────────────┐
      │                │                │
      ▼                ▼                ▼
┌──────────┐    ┌──────────┐    ┌──────────┐
│  /api/*  │    │   /ws    │    │ /static  │
│  REST    │    │WebSocket │    │  Files   │
└────┬─────┘    └────┬─────┘    └──────────┘
     │               │
     ▼               ▼
┌──────────────────────────────────────┐
│           AppState                    │
│  (DatabasePool, Config, Providers)    │
└──────────────────────────────────────┘
```

## Server Setup

```rust
pub async fn build_app(config: ServerConfig) -> Result<Router> {
    // Create database pool
    let pool = create_pool(&config.database).await?;
    pool.migrate().await?;

    // Create app state
    let state = AppState::new(pool, config.clone());

    // Build router
    let app = Router::new()
        .nest("/api", api_routes())
        .route("/ws", get(ws_handler))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    Ok(app)
}
```

## Authentication

### OAuth Flow

```
1. Client → /api/auth/oauth/github/start
   ← Returns redirect URL

2. User authenticates with GitHub

3. GitHub → /api/auth/oauth/github/callback?code=xxx
   Server exchanges code for token
   Creates/updates user in database
   ← Returns JWT

4. Client stores JWT, sends in Authorization header
```

### JWT Authentication

```rust
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
}

// Extract authenticated user from request
#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract and validate JWT from Authorization header
        let token = extract_bearer_token(&parts.headers)?;
        let claims = validate_jwt(&token, &state.config.jwt_secret)?;
        Ok(AuthUser::from(claims))
    }
}
```

### Protected Routes

```rust
async fn get_current_user(
    State(state): State<AppState>,
    auth: AuthUser,  // Automatically validated
) -> ApiResult<Json<UserResponse>> {
    let user = state.pool.users()
        .get_by_id(auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(user.into()))
}
```

## API Routes

### Authentication

```rust
Router::new()
    .route("/auth/oauth/:provider/start", post(start_oauth))
    .route("/auth/oauth/:provider/callback", get(oauth_callback))
    .route("/auth/logout", post(logout))
    .route("/auth/me", get(get_current_user))
```

### Organizations

```rust
Router::new()
    .route("/orgs", get(list_orgs).post(create_org))
    .route("/orgs/:slug", get(get_org).patch(update_org).delete(delete_org))
    .route("/orgs/:slug/members", get(list_members))
    .route("/orgs/:slug/members/invite", post(invite_member))
    .route("/orgs/:slug/repos", get(list_repos).post(connect_repo))
```

### Repositories

```rust
Router::new()
    .route("/repos/:id", get(get_repo).delete(disconnect_repo))
    .route("/repos/:id/sync", post(sync_repo))
    .route("/repos/:id/stacks", get(list_stacks))
```

### Webhooks

```rust
Router::new()
    .route("/webhooks/github", post(handle_github_webhook))
    .route("/webhooks/gitlab", post(handle_gitlab_webhook))
```

## AppState

Shared application state available to all handlers:

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<dyn DatabasePool>,
    pub config: Arc<ServerConfig>,
    pub github: Option<Arc<GitHubProvider>>,
    pub gitlab: Option<Arc<GitLabProvider>>,
}

impl AppState {
    pub fn new(pool: Box<dyn DatabasePool>, config: ServerConfig) -> Self {
        let github = config.github_client_id.as_ref().map(|_| {
            Arc::new(GitHubProvider::new(/* ... */))
        });

        Self {
            pool: Arc::from(pool),
            config: Arc::new(config),
            github,
            gitlab: None,
        }
    }
}
```

## Error Handling

```rust
pub enum ApiError {
    NotFound,
    Unauthorized,
    Forbidden,
    BadRequest(String),
    Internal(String),
    Database(DbError),
    Provider(ProviderError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            // ...
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
```

## WebSocket

Real-time updates for the dashboard:

```rust
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (sender, mut receiver) = socket.split();

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse command (subscribe, unsubscribe)
            }
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }
}
```

### Message Types

```rust
pub enum WsMessage {
    Subscribe { channel: String },
    Unsubscribe { channel: String },
    Event { channel: String, event: String, data: Value },
}

// Channels: "repo:uuid", "org:uuid"
// Events: "mr:updated", "branch:synced", "ci:status_changed"
```

## Webhooks

### GitHub

```rust
async fn handle_github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> ApiResult<()> {
    // Verify signature
    let signature = headers.get("X-Hub-Signature-256")
        .ok_or(ApiError::Unauthorized)?;
    verify_github_signature(&body, signature, &state.config.github_webhook_secret)?;

    // Parse event
    let event_type = headers.get("X-GitHub-Event")
        .ok_or(ApiError::BadRequest("Missing event type".into()))?;

    match event_type.to_str()? {
        "pull_request" => handle_pr_event(&state, &body).await?,
        "push" => handle_push_event(&state, &body).await?,
        "status" => handle_status_event(&state, &body).await?,
        _ => {}
    }

    Ok(())
}
```

### GitLab

```rust
async fn handle_gitlab_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> ApiResult<()> {
    // Verify token
    let token = headers.get("X-Gitlab-Token")
        .ok_or(ApiError::Unauthorized)?;
    verify_gitlab_token(token, &state.config.gitlab_webhook_secret)?;

    // Parse event
    let payload: GitLabWebhook = serde_json::from_str(&body)?;

    match payload.object_kind.as_str() {
        "merge_request" => handle_mr_event(&state, &payload).await?,
        "push" => handle_push_event(&state, &payload).await?,
        "pipeline" => handle_pipeline_event(&state, &payload).await?,
        _ => {}
    }

    Ok(())
}
```

## Configuration

```rust
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub jwt_secret: String,
    pub database: DatabaseConfig,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub gitlab_client_id: Option<String>,
    pub gitlab_client_secret: Option<String>,
    pub gitlab_url: String,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        // Load from environment variables
    }
}
```
