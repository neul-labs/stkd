# Troubleshooting

Solutions to common issues with Stack.

## Installation Issues

### "command not found: gt"

The binary isn't in your PATH. After `cargo install`:

```bash
# Check if it's installed
ls ~/.cargo/bin/gt

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build errors

Ensure you have the required dependencies:

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# macOS
xcode-select --install

# Fedora
sudo dnf install gcc openssl-devel
```

## Initialization Issues

### "Not a git repository"

Stack must be run inside a Git repository:

```bash
cd /path/to/your/repo
gt init
```

### "Stack not initialized"

Initialize Stack first:

```bash
gt init
```

### "Already initialized"

This is just a warning. Your existing configuration is preserved.

## Authentication Issues

### "Authentication required"

Re-authenticate with your provider:

```bash
gt auth logout github
gt auth login github
```

### "Bad credentials"

Your token may be expired or revoked:

1. Generate a new token
2. Re-authenticate:

```bash
gt auth login github --token
```

### "Permission denied"

Ensure your token has the required scopes:

- GitHub: `repo`, `read:user`
- GitLab: `api`, `read_user`

### OAuth flow doesn't complete

Try token-based auth instead:

```bash
gt auth login github --token
```

## Sync Issues

### "Conflict during rebase"

Resolve conflicts manually:

```bash
# See conflicted files
git status

# Edit and resolve
vim conflicted-file.rs

# Mark as resolved
git add conflicted-file.rs

# Continue
gt continue
```

### "Branch has diverged"

Your local branch differs from remote:

```bash
# Option 1: Force push (if you're the only one working on it)
gt submit

# Option 2: Merge remote changes
git pull --rebase origin branch-name
```

### "Remote branch not found"

The branch may have been deleted:

```bash
# Check remote branches
git fetch --prune

# Remove local tracking if branch was landed
gt sync
```

## Submit Issues

### "PR creation failed"

Check:

1. Authentication: `gt auth status`
2. Network connectivity
3. Repository permissions

### "Base branch mismatch"

Run sync to update base branches:

```bash
gt sync
gt submit
```

### PRs show wrong diff

Ensure your stack is properly rebased:

```bash
gt restack
gt submit
```

## Performance Issues

### Stack is slow

For large repositories:

```bash
# Use shallow fetch
git config fetch.depth 100

# Prune old branches
git fetch --prune
```

### Many branches slow down operations

Clean up merged branches:

```bash
gt sync  # Removes landed branches
```

## Recovery

### Undo a mistake

```bash
gt undo
```

### Undo multiple operations

```bash
gt undo 3
```

### Abort current operation

```bash
gt abort
```

### Reset Stack completely

If Stack's state is corrupted:

```bash
# Remove Stack metadata (keeps Git branches)
rm -rf .git/stack

# Re-initialize
gt init

# Re-track your branches
gt track branch-1 --parent main
gt track branch-2 --parent branch-1
```

## Getting Help

### Enable debug logging

```bash
gt --debug <command>
```

### Check version

```bash
gt --version
```

### Report a bug

Include in your bug report:

1. Stack version
2. Git version (`git --version`)
3. Operating system
4. Full command and output
5. Steps to reproduce

Open issues at: [github.com/neul-labs/stkd/issues](https://github.com/neul-labs/stkd/issues)
