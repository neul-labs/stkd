# gt config

View or edit Stack configuration.

## Usage

```bash
gt config [OPTIONS] [key] [value]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[key]` | Configuration key to get/set |
| `[value]` | Value to set |

## Examples

### View All Configuration

```bash
gt config
```

### Get a Specific Value

```bash
gt config trunk
```

### Set a Value

```bash
gt config trunk main
```

## Configuration Keys

| Key | Description | Default |
|-----|-------------|---------|
| `trunk` | Trunk branch name | `main` or `master` |
| `remote` | Remote name | `origin` |
| `provider.type` | Git provider | auto-detected |
| `provider.host` | Provider hostname | provider default |

## Configuration File

Configuration is stored in `.git/stack/config.json`:

```json
{
  "trunk": "main",
  "remote": "origin",
  "provider": {
    "type": "github",
    "owner": "username",
    "repo": "project"
  }
}
```

## Provider Configuration

Provider settings are typically auto-detected from your remote URL. Override if needed:

```bash
gt config provider.type gitlab
gt config provider.host gitlab.mycompany.com
```

## Related Commands

- [`gt init`](../commands/index.md) - Initialize with configuration wizard
- [`gt auth`](auth.md) - Authentication settings
