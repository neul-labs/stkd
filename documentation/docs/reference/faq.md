# Frequently Asked Questions

## General

### What is Stack?

Stack is a CLI tool for managing stacked pull requests (also called stacked diffs). It helps you break large changes into small, reviewable PRs that build on each other.

### How is Stack different from Graphite?

Stack is a Graphite-compatible open-source alternative. Key differences:

- **Open source** - MIT licensed, self-hostable
- **No account required** - Works directly with GitHub/GitLab
- **Local-first** - All data stored in your Git repository
- **Provider agnostic** - Supports multiple Git hosts

### Is Stack compatible with Graphite?

Stack is designed to be workflow-compatible with Graphite. If you're familiar with Graphite, Stack will feel familiar.

### Does Stack work with my existing Git workflow?

Yes! Stack works alongside standard Git. You can use `git` commands normally, and Stack tracks metadata in `.git/stack/`.

## Workflow

### Can I use Stack with existing branches?

Yes! Use `gt track` to add existing branches to your stack:

```bash
gt track my-existing-branch --parent main
```

### How do I remove a branch from Stack without deleting it?

Use `gt untrack`:

```bash
gt untrack branch-name
```

### Can I have multiple independent stacks?

Yes! Each stack is independent. Create a new stack by checking out trunk and creating a new branch:

```bash
gt checkout main
gt create new-stack-root
```

### What happens when I land the middle of a stack?

Stack handles this gracefully:

1. The middle PR merges
2. Child PRs automatically update their base branch
3. Run `gt sync` to rebase local branches

## Technical

### Where does Stack store its data?

Stack stores metadata in `.git/stack/`:

```
.git/stack/
├── config.json    # Repository configuration
├── state.json     # Current state
└── branches/      # Per-branch metadata
```

### Does Stack modify my Git history?

Stack uses standard Git operations (rebase, commit --amend). It doesn't do anything you couldn't do manually.

### Is Stack safe to use?

Yes! Stack includes:

- **Undo/redo** - Recover from mistakes
- **Dry-run mode** - Preview changes
- **Force-push safety** - Uses `--force-with-lease`

### Can I use Stack in CI/CD?

Yes! Set the `GITHUB_TOKEN` or `GITLAB_TOKEN` environment variable:

```bash
GITHUB_TOKEN=xxx gt submit
```

## Troubleshooting

### Stack says "not initialized"

Run `gt init` in your repository:

```bash
gt init
```

### My PRs show the wrong base branch

Run `gt sync` to update PR base branches:

```bash
gt sync
gt submit
```

### I get "authentication required" errors

Re-authenticate:

```bash
gt auth logout github
gt auth login github
```

### Rebase conflicts are frustrating

Tips for fewer conflicts:

1. Keep branches small and focused
2. Sync frequently (`gt sync`)
3. Land PRs promptly
4. Communicate with teammates

### How do I report a bug?

Open an issue on GitHub:

[https://github.com/dipankar/stack/issues](https://github.com/dipankar/stack/issues)

Include:

- Stack version (`gt --version`)
- Operating system
- Steps to reproduce
- Error messages
