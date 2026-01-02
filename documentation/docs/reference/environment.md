# Environment Variables

Environment variables that affect Stack's behavior.

## Authentication

### GITHUB_TOKEN

GitHub personal access token or OAuth token.

```bash
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
```

Takes precedence over stored credentials.

### GITLAB_TOKEN

GitLab personal access token.

```bash
export GITLAB_TOKEN=glpat-xxxxxxxxxxxxxxxxxxxx
```

Takes precedence over stored credentials.

## Debugging

### RUST_LOG

Control log verbosity.

```bash
# Enable debug logging
export RUST_LOG=debug

# Enable trace logging (very verbose)
export RUST_LOG=trace

# Filter by module
export RUST_LOG=stack_core=debug,stack_github=trace
```

### RUST_BACKTRACE

Show backtraces on errors.

```bash
export RUST_BACKTRACE=1
```

## Git Configuration

Stack respects standard Git environment variables:

### GIT_DIR

Override the Git directory location.

```bash
export GIT_DIR=/path/to/.git
```

### GIT_WORK_TREE

Override the working tree location.

```bash
export GIT_WORK_TREE=/path/to/worktree
```

### GIT_AUTHOR_NAME / GIT_AUTHOR_EMAIL

Override commit author information.

```bash
export GIT_AUTHOR_NAME="Your Name"
export GIT_AUTHOR_EMAIL="you@example.com"
```

## Editor

### EDITOR / VISUAL

Editor for commit messages and interactive operations.

```bash
export EDITOR=vim
# or
export VISUAL="code --wait"
```

## Example: CI Environment

For CI/CD pipelines:

```bash
# .github/workflows/ci.yml
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  RUST_LOG: info
```

## Example: Development

For local development:

```bash
# ~/.bashrc or ~/.zshrc
export GITHUB_TOKEN=ghp_your_token
export RUST_LOG=debug
export EDITOR=nvim
```

## Precedence

For authentication, precedence is:

1. Environment variables (highest)
2. Stored credentials (`~/.config/stack/credentials.json`)
3. Git credential helpers (lowest)
