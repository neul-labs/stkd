# Configuration Reference

Complete reference for Stack configuration options.

## Configuration File

Stack stores configuration in `.git/stack/config.json`:

```json
{
  "trunk": "main",
  "remote": "origin",
  "provider": {
    "type": "github",
    "owner": "username",
    "repo": "project",
    "host": "github.com"
  }
}
```

## Core Settings

### trunk

The main branch that stacks target.

| Property | Value |
|----------|-------|
| Type | string |
| Default | Auto-detected (`main` or `master`) |
| Command | `gt config trunk <value>` |

### remote

The Git remote to push to and fetch from.

| Property | Value |
|----------|-------|
| Type | string |
| Default | `origin` |
| Command | `gt config remote <value>` |

## Provider Settings

### provider.type

The Git hosting provider.

| Property | Value |
|----------|-------|
| Type | `github` \| `gitlab` |
| Default | Auto-detected from remote URL |
| Command | `gt config provider.type <value>` |

### provider.host

The provider hostname (for self-hosted instances).

| Property | Value |
|----------|-------|
| Type | string |
| Default | Provider's default (`github.com`, `gitlab.com`) |
| Command | `gt config provider.host <value>` |

### provider.owner

Repository owner/organization.

| Property | Value |
|----------|-------|
| Type | string |
| Default | Auto-detected from remote URL |

### provider.repo

Repository name.

| Property | Value |
|----------|-------|
| Type | string |
| Default | Auto-detected from remote URL |

## Example Configurations

### GitHub.com

```json
{
  "trunk": "main",
  "remote": "origin",
  "provider": {
    "type": "github",
    "owner": "myorg",
    "repo": "myproject",
    "host": "github.com"
  }
}
```

### GitHub Enterprise

```json
{
  "trunk": "main",
  "remote": "origin",
  "provider": {
    "type": "github",
    "owner": "myorg",
    "repo": "myproject",
    "host": "github.mycompany.com"
  }
}
```

### GitLab.com

```json
{
  "trunk": "main",
  "remote": "origin",
  "provider": {
    "type": "gitlab",
    "owner": "mygroup",
    "repo": "myproject",
    "host": "gitlab.com"
  }
}
```

### Self-hosted GitLab

```json
{
  "trunk": "develop",
  "remote": "origin",
  "provider": {
    "type": "gitlab",
    "owner": "myteam",
    "repo": "myproject",
    "host": "gitlab.internal.company.com"
  }
}
```

## Viewing Configuration

```bash
# View all settings
gt config

# View specific setting
gt config trunk
gt config provider.type
```

## Modifying Configuration

```bash
# Set a value
gt config trunk main
gt config provider.host gitlab.mycompany.com

# Or edit the file directly
vim .git/stack/config.json
```

## Global vs Repository Configuration

Currently, all configuration is per-repository. There is no global configuration file.

## Migration

When Stack's configuration format changes, it automatically migrates old configurations on first run.
