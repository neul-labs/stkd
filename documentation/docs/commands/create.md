# gt create

Create a new branch on top of the current branch.

## Usage

```bash
gt create [OPTIONS] <name>
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<name>` | Name for the new branch |

## Options

| Option | Description |
|--------|-------------|
| `--from-trunk` | Create from trunk instead of current branch |
| `-t, --template <name>` | Use a template to create multiple branches |
| `--list-templates` | List available templates |

## Examples

### Basic Usage

Create a new branch on top of the current branch:

```bash
gt create feature/login
```

### Create from Trunk

Create a branch directly from main/master:

```bash
gt create --from-trunk feature/new-feature
```

### Using Templates

List available templates:

```bash
gt create --list-templates
```

Create from a template:

```bash
gt create --template feature my-feature
```

This creates a stack of branches based on the template pattern.

## Behavior

1. Ensures working directory is clean
2. Creates a new Git branch from current HEAD
3. Tracks the branch in Stack's metadata
4. Switches to the new branch

## Related Commands

- [`gt delete`](delete.md) - Delete a branch
- [`gt rename`](rename.md) - Rename a branch
- [`gt track`](track.md) - Track an existing branch
