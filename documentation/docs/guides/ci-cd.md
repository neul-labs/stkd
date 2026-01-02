# CI/CD Integration

Using Stack with continuous integration and deployment.

## How Stack Works with CI

Each branch in a stack triggers its own CI run:

```
main
 └── feature/models    PR #1 → CI Run #1
      └── feature/api  PR #2 → CI Run #2
           └── feature/ui PR #3 → CI Run #3
```

## CI Status in Stack

View CI status with:

```bash
gt log --long
```

Output shows CI status:

```
feature/models (#101) ✅ CI passed
feature/api (#102) ⏳ CI running
feature/ui (#103) 💥 CI failed
```

## Best Practices

### Test Dependencies Correctly

Each PR should pass CI independently:

```yaml
# .github/workflows/test.yml
on:
  pull_request:
    branches: [main, 'feature/**']

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm test
```

### Fast CI for Faster Stacks

Slow CI blocks the whole stack. Optimize:

- Use caching
- Run tests in parallel
- Skip unnecessary checks on dependent PRs

### Branch Protection

Configure branch protection to require CI:

```yaml
# GitHub branch protection
- Require status checks to pass
- Require branches to be up to date
```

## GitHub Actions Example

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Needed for accurate diffs

      - name: Setup
        run: npm ci

      - name: Lint
        run: npm run lint

      - name: Test
        run: npm test

      - name: Build
        run: npm run build
```

## GitLab CI Example

```yaml
stages:
  - test
  - build

test:
  stage: test
  script:
    - npm ci
    - npm test
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == "main"

build:
  stage: build
  script:
    - npm run build
  rules:
    - if: $CI_COMMIT_BRANCH == "main"
```

## Handling CI Failures

When CI fails on a branch in your stack:

```bash
# Go to the failing branch
gt checkout feature/failing-branch

# Fix the issue
vim src/broken-test.rs

# Update the commit
git add .
gt modify

# Restack downstream branches
gt restack

# Push all updates
gt submit
```

## Auto-merge on CI Pass

Some teams auto-merge when CI passes:

### GitHub

Enable "Auto-merge" on PRs, then:

```bash
gt submit
# Enable auto-merge on each PR in GitHub UI
```

### GitLab

Use "Merge when pipeline succeeds":

```bash
gt submit
# Enable MWPS on each MR in GitLab UI
```

## Tips

1. **Keep CI fast** - Under 10 minutes ideal
2. **Cache dependencies** - Don't download on every run
3. **Test the right things** - Focus on what changed
4. **Parallelize** - Run independent tests concurrently
