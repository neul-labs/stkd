# gt auth

Manage authentication with Git hosting providers.

## Synopsis

```
gt auth [COMMAND]
```

## Commands

| Command | Description |
|---------|-------------|
| `login` | Authenticate with a provider |
| `logout` | Remove stored credentials |
| `status` | Show current authentication status |

## gt auth login

Authenticate with a Git hosting provider.

### Synopsis

```
gt auth login [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--github` | Authenticate with GitHub |
| `--gitlab` | Authenticate with GitLab |
| `--token <TOKEN>` | Use a personal access token |
| `--host <HOST>` | Specify a custom host (for enterprise) |

### Examples

#### OAuth Login (Recommended)

```bash
gt auth login --github

# Output:
# To authenticate, visit:
# https://github.com/login/device
#
# Enter code: ABC1-2345
#
# Waiting for authentication...
# ✓ Authenticated as octocat
```

#### Token Login

```bash
gt auth login --github --token ghp_xxxxxxxxxxxx

# Output:
# ✓ Authenticated as octocat
```

#### Enterprise/Self-Hosted

```bash
gt auth login --gitlab --host gitlab.company.com

# Output:
# To authenticate, visit:
# https://gitlab.company.com/-/profile/personal_access_tokens
#
# Enter your personal access token:
# ✓ Authenticated as jdoe
```

## gt auth logout

Remove stored credentials.

### Synopsis

```
gt auth logout [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--github` | Remove GitHub credentials |
| `--gitlab` | Remove GitLab credentials |
| `--all` | Remove all credentials |

### Examples

```bash
# Remove GitHub credentials
gt auth logout --github

# Remove all credentials
gt auth logout --all
```

## gt auth status

Show current authentication status.

### Synopsis

```
gt auth status
```

### Examples

```bash
gt auth status

# Output:
# GitHub: ✓ Authenticated as octocat
# GitLab: ✓ Authenticated as jdoe (gitlab.company.com)
```

Or if not authenticated:

```bash
gt auth status

# Output:
# GitHub: ✗ Not authenticated
# GitLab: ✗ Not authenticated
#
# Run 'gt auth login' to authenticate
```

## Token Scopes

### GitHub

Required scopes for personal access tokens:

- `repo` - Full repository access
- `read:org` - Read organization membership

### GitLab

Required scopes:

- `api` - Full API access
- `read_user` - Read user info

## Credential Storage

Credentials are stored securely in:

| Platform | Location |
|----------|----------|
| Linux | `~/.config/stack/credentials.json` |
| macOS | `~/.config/stack/credentials.json` |
| Windows | `%APPDATA%\stack\credentials.json` |

The credentials file is encrypted using your system keychain when available.

## Troubleshooting

### "Token expired"

```bash
gt auth logout --github
gt auth login --github
```

### "Insufficient scopes"

Your token may be missing required scopes. Generate a new token with the correct scopes.

### "Connection refused"

Check your network connection and any proxy settings:

```bash
export HTTPS_PROXY=http://proxy.company.com:8080
gt auth login
```

## Related Commands

- [gt submit](./submit.md) - Submit PRs (requires auth)
- [gt sync](./sync.md) - Sync with remote (requires auth for PR status)
