# gt fold

Fold staged changes into a previous commit.

## Usage

```bash
gt fold [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--into <commit>` | Target commit (default: HEAD) |

## Examples

### Fold into Last Commit

```bash
git add forgotten-file.txt
gt fold
```

### Fold into Specific Commit

```bash
git add related-change.txt
gt fold --into HEAD~2
```

## Behavior

1. Takes currently staged changes
2. Amends the specified commit to include them
3. Rebases any commits that come after

## Use Cases

- Adding forgotten files to previous commits
- Fixing typos in earlier commits
- Moving changes to a more appropriate commit

## Difference from Modify

| Command | Target |
|---------|--------|
| `gt modify` | Always HEAD |
| `gt fold` | Any commit in the branch |

## Related Commands

- [`gt modify`](modify.md) - Amend HEAD commit
- [`gt squash`](squash.md) - Combine multiple commits
- [`gt split`](split.md) - Split commits apart
