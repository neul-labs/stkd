# Authentication

Stack needs to authenticate with your Git provider to create and manage pull requests.

## GitHub

### OAuth (Recommended)

The easiest way to authenticate with GitHub:

```bash
gt auth login github
```

This opens your browser to complete OAuth authorization. Stack requests minimal permissions:

- `repo` - Access to repositories
- `read:user` - Read your profile info

### Personal Access Token

Alternatively, use a personal access token:

```bash
gt auth login github --token
```

You'll be prompted to enter your token. Create one at [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens).

Required scopes:

- `repo` - Full control of private repositories

## GitLab

### Personal Access Token

```bash
gt auth login gitlab
```

You'll be prompted for:

1. GitLab host (default: `gitlab.com`)
2. Personal access token

Create a token at **Settings > Access Tokens** with these scopes:

- `api` - Full API access
- `read_user` - Read user info

### Self-Hosted GitLab

For self-hosted GitLab instances:

```bash
gt auth login gitlab --host gitlab.mycompany.com
```

## Verify Authentication

Check your authentication status:

```bash
gt auth status
```

Output:
```
GitHub (github.com)
  Authenticated as: username
  Token: ghp_****...****

GitLab (gitlab.com)
  Not authenticated
```

## Multiple Accounts

Stack supports multiple provider accounts. The correct account is selected based on your repository's remote URL.

```bash
# Add GitHub Enterprise
gt auth login github --host github.mycompany.com

# Add self-hosted GitLab
gt auth login gitlab --host gitlab.mycompany.com
```

## Logout

Remove stored credentials:

```bash
# Logout from GitHub
gt auth logout github

# Logout from GitLab
gt auth logout gitlab

# Logout from all providers
gt auth logout --all
```

## Credential Storage

Credentials are stored securely in:

| OS | Location |
|----|----------|
| Linux | `~/.config/stack/credentials.json` |
| macOS | `~/.config/stack/credentials.json` |
| Windows | `%APPDATA%\stack\credentials.json` |

!!! warning "Security"
    The credentials file contains sensitive tokens. Ensure it's not readable by other users and not committed to version control.

## Environment Variables

You can also provide credentials via environment variables:

```bash
# GitHub
export GITHUB_TOKEN=ghp_your_token_here

# GitLab
export GITLAB_TOKEN=glpat-your_token_here
```

Environment variables take precedence over stored credentials.

## Troubleshooting

### "Authentication required" errors

Re-authenticate:

```bash
gt auth logout github
gt auth login github
```

### Token expired

GitHub OAuth tokens don't expire, but PATs might. Generate a new token and re-authenticate.

### Wrong account

If Stack is using the wrong account, logout and login with the correct one:

```bash
gt auth logout github
gt auth login github
```
