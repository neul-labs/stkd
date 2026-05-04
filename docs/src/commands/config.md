# gt config

View or edit Stack configuration.

## Synopsis

```bash
gt config [OPTIONS] [KEY] [VALUE]
```

## Description

Manages Stack configuration stored in `.git/stkd/config.json`. Without arguments, displays the current configuration. With a key and value, sets the specified option.

## Arguments

| Argument | Description |
|----------|-------------|
| `<KEY>` | Configuration key to view or set |
| `<VALUE>` | Value to set (omit to view current value) |

## Options

| Option | Description |
|--------|-------------|
| `--unset` | Remove the specified key |
| `--json` | Output as JSON |

## Examples

```bash
# View all configuration
gt config

# View a specific key
gt config trunk

# Set a key
gt config trunk main

# Remove a key
gt config --unset submit.draft
```

## Configuration Keys

| Key | Description | Default |
|-----|-------------|---------|
| `trunk` | Name of the trunk branch | Auto-detected |
| `remote` | Name of the git remote | `origin` |
| `submit.draft` | Create PRs/MRs as draft by default | `false` |
| `sync.delete_merged` | Delete local branches after merge | `true` |

## See Also

- [`gt auth`](./auth.md) — Authenticate with a provider
