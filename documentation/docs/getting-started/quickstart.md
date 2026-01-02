# Quick Start

Get up and running with Stack in 5 minutes.

## 1. Initialize Stack

Navigate to your Git repository and initialize Stack:

```bash
cd your-repo
gt init
```

Stack will detect your trunk branch (usually `main` or `master`) and remote.

## 2. Authenticate

Connect Stack to your Git provider:

=== "GitHub"

    ```bash
    gt auth login github
    ```

    This opens a browser for OAuth authentication.

=== "GitLab"

    ```bash
    gt auth login gitlab
    ```

    You'll be prompted for a personal access token.

## 3. Create a Branch

Create your first tracked branch:

```bash
gt create feature/my-feature
```

This creates a new branch on top of your current branch and tracks it in the stack.

## 4. Make Changes

Work normally with Git:

```bash
# Edit files...
git add .
git commit -m "Add new feature"
```

Or use Stack's modify command to amend:

```bash
# Edit files...
git add .
gt modify  # Amends the last commit
```

## 5. View Your Stack

See the current stack structure:

```bash
gt log
```

Output:
```
  main
   └── feature/my-feature ← you are here
```

## 6. Submit PRs

When you're ready, submit your stack as pull requests:

```bash
gt submit
```

Stack creates PRs with proper base branches and descriptions.

## 7. Keep in Sync

After changes are merged upstream:

```bash
gt sync
```

Stack fetches updates and rebases your branches automatically.

## Common Workflow

Here's a typical workflow:

```bash
# Start from main
gt checkout main
gt sync

# Create a stack of features
gt create feature/step-1
# ... work, commit ...

gt create feature/step-2
# ... work, commit ...

gt create feature/step-3
# ... work, commit ...

# Submit all PRs
gt submit

# After step-1 is merged
gt sync  # Rebases step-2 and step-3 onto main
```

## Next Steps

- [First Stack](first-stack.md) - Detailed tutorial on creating stacks
- [Commands](../commands/index.md) - Full command reference
- [Workflows](../guides/workflows.md) - Common workflow patterns
