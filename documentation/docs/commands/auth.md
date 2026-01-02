# gt auth

Manage authentication with Git providers.

## Usage

```bash
gt auth <command>
```

## Commands

| Command | Description |
|---------|-------------|
| `login` | Authenticate with a provider |
| `logout` | Remove stored credentials |
| `status` | Show authentication status |

## Login

```bash
gt auth login <provider> [OPTIONS]
```

### Providers

- `github` - GitHub.com
- `gitlab` - GitLab.com

### Options

| Option | Description |
|--------|-------------|
| `--host <host>` | Custom hostname for self-hosted instances |
| `--token` | Use token-based auth instead of OAuth |

### Examples

```bash
# GitHub OAuth (recommended)
gt auth login github

# GitHub with PAT
gt auth login github --token

# Self-hosted GitLab
gt auth login gitlab --host gitlab.company.com
```

## Logout

```bash
gt auth logout [provider]
```

### Options

| Option | Description |
|--------|-------------|
| `--all` | Logout from all providers |

### Examples

```bash
# Logout from GitHub
gt auth logout github

# Logout from everything
gt auth logout --all
```

## Status

```bash
gt auth status
```

Shows authentication status for all configured providers.

## Related

- [Authentication Guide](../getting-started/authentication.md) - Detailed authentication setup
- [Environment Variables](../reference/environment.md) - Token environment variables
