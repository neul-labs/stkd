# gt init

Initialize Stack in the current Git repository.

## Synopsis

```bash
gt init [OPTIONS]
```

## Description

Creates the Stack metadata directory (`.git/stkd/`) and auto-detects configuration settings such as the trunk branch name, remote name, and provider from the repository's remotes.

This is the first command you should run when setting up Stack in a new repository.

## Options

| Option | Description |
|--------|-------------|
| `--trunk <BRANCH>` | Specify the trunk branch name (default: auto-detect) |
| `--remote <REMOTE>` | Specify the git remote name (default: origin) |
| `--yes` | Skip confirmation prompts |

## Examples

```bash
# Initialize with auto-detection
gt init

# Specify trunk branch explicitly
gt init --trunk main

# Specify a different remote
gt init --remote upstream
```

## See Also

- [`gt track`](./track.md) — Start tracking an existing branch
- [`gt create`](./create.md) — Create a new stacked branch
