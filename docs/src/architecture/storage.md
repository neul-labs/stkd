# Data Storage

Stack stores metadata about tracked branches locally in your repository.

## Storage Location

Branch metadata is stored in:

```
.git/stack/
├── config           # Repository configuration
├── branches/        # One file per tracked branch
│   ├── feature%2Fauth.json
│   ├── feature%2Fapi.json
│   └── ...
└── state            # Current operation state (if any)
```

## Branch Metadata

Each tracked branch has a JSON file:

```json
{
  "name": "feature/auth",
  "parent": "main",
  "children": ["feature/auth-ui"],
  "merge_request_id": 42,
  "merge_request_url": "https://github.com/owner/repo/pull/42",
  "provider": "github",
  "base_commit": "abc123",
  "head_commit": "def456",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T14:22:00Z",
  "status": "submitted"
}
```

### Fields

| Field | Description |
|-------|-------------|
| `name` | Branch name |
| `parent` | Parent branch name |
| `children` | Child branch names |
| `merge_request_id` | PR/MR number (provider-agnostic) |
| `merge_request_url` | Web URL to the PR/MR |
| `provider` | Which provider ("github", "gitlab") |
| `base_commit` | Parent's HEAD when branch was created |
| `head_commit` | Current branch HEAD |
| `created_at` | When tracking started |
| `updated_at` | Last modification time |
| `status` | active, submitted, merged, closed |

## Configuration

Repository configuration in `.git/stack/config`:

```toml
[core]
trunk = "main"
remote = "origin"

[provider]
type = "github"
owner = "username"
repo = "reponame"

[ui]
color = true
```

## Operation State

During multi-step operations (like restack), state is saved:

```json
{
  "operation": "restack",
  "started_at": "2024-01-15T10:30:00Z",
  "branches": ["feature/a", "feature/b"],
  "current_index": 1,
  "original_branch": "feature/b"
}
```

This allows operations to be resumed after conflicts.

## Credential Storage

Credentials are stored separately from the repository:

| Platform | Location |
|----------|----------|
| Linux | `~/.config/stack/credentials.json` |
| macOS | `~/.config/stack/credentials.json` |
| Windows | `%APPDATA%\stack\credentials.json` |

Format:

```json
{
  "github": {
    "github.com": {
      "type": "oauth",
      "token": "gho_xxxx",
      "expires_at": "2024-02-15T10:30:00Z"
    }
  },
  "gitlab": {
    "gitlab.com": {
      "type": "personal_access_token",
      "token": "glpat-xxxx"
    }
  }
}
```

## File Name Encoding

Branch names are URL-encoded for filesystem safety:

- `feature/auth` → `feature%2Fauth.json`
- `fix/bug#123` → `fix%2Fbug%23123.json`

## Atomicity

Updates use atomic write operations:

1. Write to temporary file
2. Rename to final location
3. Old file is replaced atomically

This prevents corruption from interrupted writes.

## Backward Compatibility

The storage format includes version markers:

```json
{
  "version": 2,
  "name": "feature/auth",
  ...
}
```

Stack can migrate old formats automatically.

## Gitignore

By default, Stack data is not ignored. If you prefer:

```gitignore
# .gitignore
.git/stack/
```

But note that sharing stack data can be useful for team collaboration.

## Cleanup

To remove all Stack data from a repository:

```bash
rm -rf .git/stack/
```

Branches remain but are no longer tracked by Stack.
