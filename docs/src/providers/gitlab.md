# GitLab Provider

Stack supports GitLab.com and self-hosted GitLab instances.

## Status

**Beta**: Core features work, but some advanced features are still in development.

## Authentication

### Personal Access Token

```bash
gt auth login --gitlab --token glpat-xxxxxxxxxxxx
```

Required scopes:
- `api` - Full API access
- `read_user` - Read user information

### Self-Hosted GitLab

```bash
gt auth login --gitlab --host gitlab.company.com
```

## Features

### Merge Requests

- Create MRs with correct target branches
- Update MR title, description, and target
- Close and reopen MRs
- Draft/WIP MRs

### Pipelines

Stack can check GitLab CI pipeline status:

```bash
gt status --fetch

# Output:
# Merge Request
#   → !42
#   Pipeline: ✓ passed
```

### Approvals

Check approval status based on project settings:

```bash
gt status --verbose

# Output:
# Approvals: 1/2 required
#   ✓ Approved by alice
```

### Labels

Apply labels to MRs:

```toml
# .stack/config
[gitlab]
default_labels = ["stack", "needs-review"]
```

### Milestones

Assign MRs to milestones for release tracking.

## Merge Methods

GitLab supports multiple merge methods:

| Method | Command | Description |
|--------|---------|-------------|
| Merge | `gt land --method merge` | Create merge commit |
| Squash | `gt land --method squash` | Squash commits |
| Rebase | `gt land --method rebase` | Rebase commits |
| Fast-Forward | `gt land --method ff` | Fast-forward only |

Note: Fast-forward merge is a GitLab-specific feature not available on GitHub.

## Configuration

### Basic Setup

```toml
# .stack/config
[provider]
type = "gitlab"
```

### Self-Hosted

```toml
[provider]
type = "gitlab"
api_url = "https://gitlab.company.com/api/v4"
web_url = "https://gitlab.company.com"
```

### Project ID

If auto-detection fails, specify the project:

```toml
[provider]
type = "gitlab"
owner = "group"
repo = "project"
# Or use project ID directly
# project_id = 12345
```

## Differences from GitHub

| Aspect | GitHub | GitLab |
|--------|--------|--------|
| MR naming | Pull Request | Merge Request |
| ID format | #42 | !42 |
| CI | GitHub Actions | GitLab CI |
| Merge method | Squash default | Configurable |

## Troubleshooting

### "401 Unauthorized"

Check your token:

```bash
gt auth logout --gitlab
gt auth login --gitlab --token <new-token>
```

### "Project not found"

Ensure you have access and the project path is correct:

```toml
[provider]
owner = "group/subgroup"  # Include full path
repo = "project"
```

### Pipeline status not showing

GitLab CI must be configured in the project. Check `.gitlab-ci.yml`.

### MR not targeting correct branch

Verify stack relationships:

```bash
gt log
gt info
```

## Limitations

Current limitations compared to GitHub:

- No OAuth device flow (token only)
- Limited webhook support
- Some approval rules not fully supported

These are being actively worked on.
