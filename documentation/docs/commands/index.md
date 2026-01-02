# Command Reference

Stack provides commands organized into categories.

## Stack Management

Commands for creating and managing branches in your stack.

| Command | Description |
|---------|-------------|
| [`create`](create.md) | Create a new branch on top of current |
| [`delete`](delete.md) | Delete a branch from the stack |
| [`rename`](rename.md) | Rename the current branch |
| [`track`](track.md) | Start tracking an existing branch |

## Navigation

Commands for moving between branches in your stack.

| Command | Description |
|---------|-------------|
| [`up` / `down`](navigation.md) | Move up or down the stack |
| [`top` / `bottom`](top-bottom.md) | Jump to stack tip or root |
| [`checkout`](checkout.md) | Switch to a specific branch |
| [`log`](log.md) | View the stack structure |

## Workflow

Commands for syncing and managing your workflow.

| Command | Description |
|---------|-------------|
| [`sync`](sync.md) | Sync with remote and restack |
| [`restack`](restack.md) | Rebase branches onto updated parents |
| [`submit`](submit.md) | Create or update pull requests |
| [`land`](land.md) | Merge approved PRs |

## Editing

Commands for modifying commits and history.

| Command | Description |
|---------|-------------|
| [`modify`](modify.md) | Amend the current commit |
| [`squash`](squash.md) | Squash commits in current branch |
| [`fold`](fold.md) | Fold changes into a previous commit |
| [`split`](split.md) | Split a commit into multiple |

## Utilities

Utility and helper commands.

| Command | Description |
|---------|-------------|
| [`undo` / `redo`](undo-redo.md) | Undo or redo operations |
| [`config`](config.md) | View or edit configuration |
| [`auth`](auth.md) | Manage authentication |

## Global Options

These options work with any command:

```
--debug     Enable debug output
-q, --quiet Suppress non-essential output
-h, --help  Print help
-V, --version Print version
```

## Getting Help

Get help for any command:

```bash
gt help <command>
gt <command> --help
```
