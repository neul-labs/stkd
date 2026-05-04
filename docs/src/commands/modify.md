# gt modify

Amend the current branch's commits.

## Synopsis

```bash
gt modify [OPTIONS]
```

## Description

Amends the most recent commit on the current branch. This is useful for fixing up the last commit before submitting.

## Options

| Option | Description |
|--------|-------------|
| `--message <MSG>` | New commit message |
| `--no-edit` | Keep the current commit message |

## Examples

```bash
# Amend the last commit
gt modify

# Amend with a new message
gt modify --message "Fix typo in auth flow"
```

## See Also

- [`gt squash`](./squash.md) — Squash commits in the current branch
- [`gt fold`](./fold.md) — Fold staged changes into a previous commit
