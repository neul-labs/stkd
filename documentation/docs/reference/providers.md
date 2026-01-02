# Provider Reference

Stack supports multiple Git hosting providers.

## Supported Providers

| Provider | Status | Self-Hosted |
|----------|--------|-------------|
| GitHub | Full Support | Yes |
| GitLab | Full Support | Yes |
| Gitea | Planned | - |
| Bitbucket | Considering | - |

## GitHub

### Features

- Pull request creation and management
- Review requests
- CI status (GitHub Actions)
- Labels and milestones
- Draft PRs
- Auto-merge

### Authentication

```bash
# OAuth (recommended)
gt auth login github

# Personal Access Token
gt auth login github --token
```

### Required Scopes

For OAuth:
- `repo` - Repository access
- `read:user` - User profile

For PAT:
- `repo` - Full control of private repositories

### GitHub Enterprise

```bash
gt auth login github --host github.mycompany.com
```

## GitLab

### Features

- Merge request creation and management
- Approval requests
- Pipeline status (GitLab CI)
- Labels and milestones
- Draft MRs
- Merge when pipeline succeeds

### Authentication

```bash
gt auth login gitlab
```

You'll be prompted for a personal access token.

### Required Scopes

- `api` - Full API access
- `read_user` - Read user info

### Self-Hosted GitLab

```bash
gt auth login gitlab --host gitlab.mycompany.com
```

## Provider Detection

Stack auto-detects the provider from your remote URL:

| URL Pattern | Provider |
|-------------|----------|
| `github.com` | GitHub |
| `gitlab.com` | GitLab |
| `*.github.*` | GitHub Enterprise |
| `*/gitlab/*` | GitLab |

Override if needed:

```bash
gt config provider.type github
gt config provider.host github.mycompany.com
```

## Feature Comparison

| Feature | GitHub | GitLab |
|---------|--------|--------|
| PR/MR Creation | ✅ | ✅ |
| Draft Support | ✅ | ✅ |
| Review Requests | ✅ | ✅ |
| CI Status | ✅ | ✅ |
| Labels | ✅ | ✅ |
| Milestones | ✅ | ✅ |
| Auto-merge | ✅ | ✅ |
| Squash Merge | ✅ | ✅ |
| Rebase Merge | ✅ | ✅ |

## API Rate Limits

### GitHub

- Authenticated: 5,000 requests/hour
- Stack typically uses 2-10 requests per operation

### GitLab

- Authenticated: 2,000 requests/minute
- Stack typically uses 2-10 requests per operation

## Troubleshooting

### "Not authenticated" errors

```bash
gt auth status  # Check current auth
gt auth logout github
gt auth login github
```

### "Rate limit exceeded"

Wait for the rate limit to reset, or use a different token.

### "Permission denied"

Ensure your token has the required scopes.
