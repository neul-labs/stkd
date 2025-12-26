# GitHub Provider

Stack has full support for GitHub, including GitHub Enterprise.

## Authentication

### OAuth (Recommended)

```bash
gt auth login --github
```

This opens a browser for secure OAuth authentication.

### Personal Access Token

```bash
gt auth login --github --token ghp_xxxxxxxxxxxx
```

Required scopes:
- `repo` - Full repository access
- `read:org` - Read organization membership (for org repos)

### GitHub Enterprise

```bash
gt auth login --github --host github.company.com
```

## Features

### Pull Requests

- Create PRs with correct base branches
- Update PR title, body, and base
- Close and reopen PRs
- Draft PRs

### CI/CD Integration

Stack can check GitHub Actions status:

```bash
gt status --fetch

# Output:
# Pull Request
#   → #42
#   CI: ✓ All checks passed
```

### Reviews

Request reviews and check approval status:

```bash
# Check review status
gt status --verbose

# Output:
# Reviews:
#   ✓ Approved by octocat
#   ⚠ Changes requested by contributor
```

### Labels

PRs can be labeled automatically based on configuration.

### Milestones

Assign PRs to milestones for release tracking.

## Merge Methods

GitHub supports three merge methods:

| Method | Command | Description |
|--------|---------|-------------|
| Squash | `gt land --method squash` | Combine commits (default) |
| Merge | `gt land --method merge` | Create merge commit |
| Rebase | `gt land --method rebase` | Rebase commits |

## API Rate Limits

GitHub has API rate limits:

- **Authenticated**: 5,000 requests/hour
- **Unauthenticated**: 60 requests/hour

Stack uses authenticated requests. If you hit limits:

```bash
# Check rate limit status
gh api rate_limit

# Wait for reset or use a different token
```

## Enterprise Configuration

For GitHub Enterprise Server:

```toml
# .stack/config
[provider]
type = "github"
api_url = "https://github.company.com/api/v3"
web_url = "https://github.company.com"
```

## Troubleshooting

### "Bad credentials"

Your token may be expired:

```bash
gt auth logout --github
gt auth login --github
```

### "Resource not accessible"

Check token scopes. Regenerate with required scopes:
- `repo`
- `read:org`

### "API rate limit exceeded"

Wait for the rate limit to reset, or use a token with higher limits.

### PR not updating

Force push may be needed:

```bash
gt submit  # Uses --force-with-lease
```
