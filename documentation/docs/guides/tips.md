# Tips & Tricks

Power user tips for getting the most out of Stack.

## Navigation Shortcuts

### Quick Stack Overview

```bash
gt ls          # Short branch list
gt ll          # Long format with PR status
gt log         # Default tree view
```

### Jump Around

```bash
gt top         # Go to newest branch
gt bottom      # Go to oldest branch
gt down 3      # Skip 3 branches down
gt checkout <partial-name>  # Fuzzy match
```

## Efficient Editing

### Amend Workflow

```bash
# Quick amend cycle
vim file.rs
git add -A
gt modify
gt restack
```

### Partial Staging

```bash
# Stage specific hunks
git add -p

# Then amend
gt modify
```

### Fixup Commits

```bash
# Add to a specific commit
git add forgotten-file.txt
gt fold --into HEAD~2
```

## Submission Tips

### Preview Before Submit

```bash
gt submit --dry-run
```

### Partial Submission

```bash
# Just the current branch
gt submit --only .

# From branch to tip
gt submit --from feature/ready

# Up to a branch
gt submit --to feature/reviewed
```

### Draft Mode

```bash
# Submit as drafts
gt submit --draft

# Convert to ready
gt submit  # Re-submit without --draft
```

## Sync Strategies

### Morning Routine

```bash
gt checkout main
gt sync
# Now you're up to date
```

### Watch for Merges

```bash
# Auto-sync when PRs merge
gt sync --watch

# With faster polling
gt sync --watch --interval 30
```

## Undo Mistakes

### Undo Last Action

```bash
gt undo
```

### Undo Multiple

```bash
gt undo 3  # Undo last 3 operations
```

### Redo if Needed

```bash
gt redo
```

## Shell Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
# Quick aliases
alias gs='gt status'
alias gl='gt log'
alias gll='gt ll'
alias gup='gt up'
alias gdn='gt down'
alias gtop='gt top'
alias gbot='gt bottom'
alias gsync='gt sync'
alias gsub='gt submit'

# Workflow aliases
alias gnew='gt create'
alias gdone='gt submit && gt sync'
```

## Git Integration

### Check Diff Against Parent

```bash
# See what you've changed
git diff $(gt info --parent)..HEAD
```

### Interactive Rebase (Carefully)

```bash
# For complex history editing
git rebase -i $(gt info --parent)
# Then update Stack
gt restack
```

## Templates

### List Available

```bash
gt create --list-templates
```

### Create Custom Stack

```bash
gt create --template feature my-feature
# Creates: my-feature/types, my-feature/impl, my-feature/tests
```

## Debugging

### Verbose Output

```bash
gt --debug sync
```

### Check Configuration

```bash
gt config
```

### Auth Status

```bash
gt auth status
```

## Performance Tips

### Large Repositories

```bash
# Shallow fetch for faster sync
git config --global fetch.depth 50
```

### Many Branches

```bash
# Prune old branches
git fetch --prune

# Clean up merged branches
gt sync  # Removes landed branches
```

## Common Patterns

### Bug Fix in Stack

```bash
# Find bug while working on feature/c
gt checkout feature/a  # Bug is here
git add .
gt modify
gt restack  # Update b and c
gt submit
```

### Split a Branch

```bash
# feature/too-big needs splitting
gt checkout feature/too-big

# Create new branch for part of it
gt create feature/part-1
# Move some commits...

gt checkout feature/too-big
# Rename to part-2
gt rename feature/part-2
```

### Reorder Stack

```bash
# Want to land feature/c before feature/b
gt checkout feature/c
# Manual rebase onto a
git rebase feature/a
# Update Stack tracking
gt track --parent feature/a
```
