# Migrating from Graphite

Stack is designed to be workflow-compatible with Graphite. If you're currently using Graphite (`gt`), moving to Stack is straightforward.

---

## Why Migrate?

| Graphite | Stack |
|----------|-------|
| Closed source | Open source (Apache-2.0) |
| Requires account | No account needed |
| Cloud-dependent | Self-hostable |
| GitHub only | GitHub + GitLab |
| SaaS pricing | Free |

Stack gives you full control over your data and works with both GitHub and GitLab.

---

## Command Compatibility

Stack aims for command-level compatibility with Graphite. Most commands work identically:

| Graphite | Stack | Status |
|----------|-------|--------|
| `gt init` | `gt init` | Identical |
| `gt create <name>` | `gt create <name>` | Identical |
| `gt checkout <name>` | `gt checkout <name>` | Identical |
| `gt up` / `gt down` | `gt up` / `gt down` | Identical |
| `gt top` / `gt bottom` | `gt top` / `gt bottom` | Identical |
| `gt log` | `gt log` | Identical |
| `gt submit` | `gt submit` | Identical |
| `gt sync` | `gt sync` | Identical |
| `gt restack` | `gt restack` | Identical |
| `gt land` | `gt land` | Identical |
| `gt modify` | `gt modify` | Identical |
| `gt undo` / `gt redo` | `gt undo` / `gt redo` | Identical |
| `gt track` | `gt track` | Identical |
| `gt auth` | `gt auth` | Identical |
| `gt config` | `gt config` | Identical |

---

## Migration Steps

### 1. Install Stack

```bash
# Via cargo
cargo install stkd-cli

# Or from source
git clone https://github.com/neul-labs/stkd
cd stkd
cargo install --path crates/stkd-cli
```

Verify the binary name:

```bash
gt --version
# Should show Stack's version
```

### 2. Authenticate

```bash
# Graphite credentials won't work — Stack uses its own auth
gt auth login github

# Or for GitLab
gt auth login gitlab
```

Stack stores credentials separately from Graphite.

### 3. Initialize Your Repository

If your repo was already initialized with Graphite, Stack should work immediately:

```bash
cd your-repo
gt log
# Should show your existing stacks
```

If Stack says "not initialized":

```bash
gt init
```

Stack reads Graphite's metadata format (`.git/graphite/` or `.git/stack/`) when available.

### 4. Verify Your Stacks

```bash
# List all tracked branches
gt log --all

# Check a specific stack
gt checkout feature/your-branch
gt log
```

Branch parent relationships should be preserved.

### 5. Submit a Test PR

Make a small change and submit to verify everything works:

```bash
gt checkout feature/test-branch
# Edit a file...
git add .
gt modify
gt submit
```

Check that the PR was created with the correct base branch.

---

## Workflow Differences

### Authentication

- **Graphite**: OAuth through Graphite's servers
- **Stack**: Direct OAuth with GitHub/GitLab, or personal access tokens

### Web Dashboard

- **Graphite**: Cloud-hosted at graphite.dev
- **Stack**: Self-hosted with `stkd-server` (optional)

Run your own dashboard:

```bash
stkd-server
```

Then visit `http://localhost:3000`.

### Merge Queue

- **Graphite**: Built-in merge queue (cloud)
- **Stack**: Land PRs individually with `gt land`, or use provider merge queues

### Team Features

- **Graphite**: Team management, PR analytics (cloud)
- **Stack**: No built-in team management; use provider features (GitHub teams, GitLab groups)

---

## Data Migration

### Branch Metadata

Stack reads Graphite's metadata if it exists:

```
.git/graphite/   # Graphite's metadata
.git/stkd/       # Stack's metadata (preferred)
```

If both exist, Stack prefers `.git/stkd/`.

To migrate explicitly:

```bash
# Stack should auto-detect, but if not:
gt init

# Re-track your branches
gt track feature/branch-1 --parent main
gt track feature/branch-2 --parent feature/branch-1
# ... etc
```

### Authentication

Graphite and Stack store credentials separately. You'll need to re-authenticate:

```bash
# Remove old Graphite credentials (optional)
rm ~/.config/graphite/credentials

# Authenticate with Stack
gt auth login github
```

---

## Team Migration Strategy

### Gradual Migration

1. **One developer** tries Stack on a non-critical repo
2. **Small team** adopts Stack for one project
3. **Full team** switches after validation

### Parallel Usage

Graphite and Stack can coexist on the same machine:

```bash
# Graphite binary (if installed via npm)
npx gt log

# Stack binary
gt log
```

Just be careful not to mix them on the same repository.

### Communication

When migrating a team:

1. Announce the migration plan
2. Update CI/CD if it references Graphite-specific features
3. Update documentation and runbooks
4. Set a cutoff date for Graphite usage

---

## Troubleshooting Migration Issues

### "Not initialized" on a Graphite repo

```bash
gt init
# Stack will detect existing Graphite metadata
```

### Branch parents are wrong

```bash
# Fix parent relationship
gt track feature/wrong-parent --parent correct-parent
gt restack
```

### PRs show wrong base branch

```bash
gt sync
gt submit
```

### Auth fails

```bash
# Re-authenticate
gt auth logout github
gt auth login github
```

---

## Feature Comparison Matrix

| Feature | Graphite | Stack |
|---------|----------|-------|
| CLI (`gt`) | Yes | Yes |
| GitHub support | Yes | Yes |
| GitLab support | No | Yes |
| Self-hosted | No | Yes |
| Open source | No | Yes |
| Web dashboard | Yes (cloud) | Yes (self-hosted) |
| Merge queue | Yes | Via provider |
| Undo/redo | Yes | Yes |
| Watch mode | Yes | Yes |
| PR templates | Yes | Yes |
| Draft PRs | Yes | Yes |
| Reviewer assignment | Yes | Yes |
| Labels | Yes | Yes |
| MCP/AI integration | No | Yes |
| TUI mode | No | Yes |

---

## Getting Help

- [Stack Issues](https://github.com/neul-labs/stkd/issues)
- [Stack Discussions](https://github.com/neul-labs/stkd/discussions)
- [FAQ](../reference/faq.md)
