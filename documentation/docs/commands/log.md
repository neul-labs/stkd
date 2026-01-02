# gt log

View the current stack structure.

## Usage

```bash
gt log [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--short` | Short format (branch names only) |
| `--long` | Long format with PR status and details |

## Aliases

- `gt ls` - Alias for `gt log --short`
- `gt ll` - Alias for `gt log --long`

## Examples

### Default View

```bash
gt log
```

Output:
```
  main
   └── feature/auth-models (#123 ✅)
        └── feature/auth-api (#124 🔄)
             └── feature/auth-ui ← you are here
```

### Short Format

```bash
gt ls
```

Output:
```
feature/auth-models
feature/auth-api
feature/auth-ui ←
```

### Long Format

```bash
gt ll
```

Output:
```
  main
   │
   └── feature/auth-models
   │   PR #123: Add user models
   │   Status: ✅ Approved, CI passing
   │   +142 -23  3 files changed
   │
   └── feature/auth-api
   │   PR #124: Add authentication API
   │   Status: 🔄 Review pending
   │   +89 -12  2 files changed
   │
   └── feature/auth-ui ← you are here
       No PR submitted
       +45 -0  1 file changed
```

## Status Icons

| Icon | Meaning |
|------|---------|
| ✅ | PR approved and ready to merge |
| 🔄 | PR open, review pending |
| ❌ | PR has requested changes |
| 🚧 | PR is a draft |
| ⏳ | CI running |
| 💥 | CI failed |

## Related Commands

- [`gt status`](../commands/index.md) - Detailed status of current branch
- [`gt info`](../commands/index.md) - Current branch info
