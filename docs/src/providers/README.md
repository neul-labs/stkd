# Provider Support

Stack supports multiple Git hosting providers through a pluggable provider system.

## Supported Providers

| Provider | Status | Features |
|----------|--------|----------|
| [GitHub](./github.md) | Stable | Full support |
| [GitLab](./gitlab.md) | Beta | Core features |
| Gitea | Planned | - |
| Bitbucket | Planned | - |

## Provider Detection

Stack automatically detects your provider from the Git remote URL:

```bash
# GitHub
git@github.com:owner/repo.git
https://github.com/owner/repo.git

# GitLab
git@gitlab.com:owner/repo.git
https://gitlab.com/owner/repo.git
```

## Features by Provider

| Feature | GitHub | GitLab |
|---------|--------|--------|
| Create MR/PR | ✓ | ✓ |
| Update MR/PR | ✓ | ✓ |
| Merge MR/PR | ✓ | ✓ |
| Draft MRs | ✓ | ✓ |
| CI Status | ✓ | ✓ |
| Reviews | ✓ | ✓ |
| Labels | ✓ | ✓ |
| Milestones | ✓ | ✓ |
| Squash Merge | ✓ | ✓ |
| Rebase Merge | ✓ | ✓ |
| Fast-Forward | ✗ | ✓ |

## Configuration

### Manual Provider Selection

If auto-detection doesn't work, configure manually in `.stack/config`:

```toml
[provider]
type = "github"  # or "gitlab"
owner = "your-username"
repo = "your-repo"
```

### Enterprise/Self-Hosted

For self-hosted instances:

```toml
[provider]
type = "gitlab"
api_url = "https://gitlab.company.com/api/v4"
web_url = "https://gitlab.company.com"
```

## Authentication

Each provider has its own authentication method:

```bash
# GitHub
gt auth login --github

# GitLab
gt auth login --gitlab
```

See [Authentication](../commands/auth.md) for details.

## Implementing New Providers

If you want to add support for a new provider, see [Implementing a Provider](./implementing.md).
