# Stack Templates Deep Dive

Templates let you create multi-branch stacks from predefined patterns. Instead of creating branches one by one, templates generate an entire stack structure in one command.

---

## What Are Templates?

A template is a recipe for a stack. When you apply a template, Stack creates multiple branches with specific names and parent relationships.

```bash
# Without a template
gt create feature/api
gt create feature/api-tests
gt create feature/api-docs

# With a template
gt create --template api my-feature
# Creates: my-feature, my-feature/tests, my-feature/docs
```

---

## Built-in Templates

Stack ships with several built-in templates for common patterns.

### API Feature Template

```bash
gt create --template api payment-processing
```

Creates:

```
main
 └── payment-processing              # Models and types
      └── payment-processing/tests     # Unit and integration tests
           └── payment-processing/docs  # API documentation
```

### UI Feature Template

```bash
gt create --template ui user-profile
```

Creates:

```
main
 └── user-profile              # Component structure
      └── user-profile/stories   # Storybook stories
           └── user-profile/tests # Component tests
```

### Bug Fix Template

```bash
gt create --template bugfix login-timeout
```

Creates:

```
main
 └── fix/login-timeout       # The fix itself
      └── fix/login-timeout/tests  # Regression test
```

### Refactor Template

```bash
gt create --template refactor auth-service
```

Creates:

```
main
 └── refactor/auth-service       # Core refactoring
      └── refactor/auth-service/migration  # Migration scripts
           └── refactor/auth-service/tests  # Updated tests
```

### Listing Templates

```bash
# Show all available templates
gt create --list-templates

# Output:
# api      - API feature (models, tests, docs)
# ui       - UI feature (component, stories, tests)
# bugfix   - Bug fix (fix, regression test)
# refactor - Refactoring (refactor, migration, tests)
# docs     - Documentation (guide, examples)
```

---

## Template Storage

### Built-in Templates

Built-in templates ship with Stack and are embedded in the binary. You don't need to install anything to use them.

### User Templates

Store custom templates in:

```
~/.config/stkd/templates/
├── api.yaml
├── ui.yaml
└── custom-feature.yaml
```

Or in the project directory:

```
project-root/
├── .stkd/
│   └── templates/
│       ├── onboarding.yaml
│       └── experiment.yaml
```

Stack searches for templates in this order:
1. `.stkd/templates/` (project-specific)
2. `~/.config/stkd/templates/` (user-specific)
3. Built-in templates

---

## Creating Custom Templates

### Template Syntax

Templates are YAML files with this structure:

```yaml
# ~/.config/stkd/templates/api.yaml
name: api
description: API feature with models, tests, and documentation

branches:
  - name: "{{name}}"
    description: "Core {{name}} models and endpoints"

  - name: "{{name}}/tests"
    parent: "{{name}}"
    description: "Unit and integration tests for {{name}}"

  - name: "{{name}}/docs"
    parent: "{{name}}"
    description: "API documentation for {{name}}"
```

### Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{{name}}` | The base name provided by the user | `payment-processing` |
| `{{author}}` | Git user name | `Alice` |
| `{{date}}` | Current date | `2024-01-15` |
| `{{year}}` | Current year | `2024` |
| `{{ticket}}` | Ticket number from branch name | `PROJ-123` |

### Advanced Template Example

```yaml
# ~/.config/stkd/templates/full-stack.yaml
name: full-stack
description: Full-stack feature with backend and frontend

variables:
  - name: frontend
    description: Frontend framework
    default: react
    choices: [react, vue, svelte]

branches:
  - name: "{{name}}/models"
    description: "Data models and migrations"
    commit_message: "Add {{name}} data models"

  - name: "{{name}}/api"
    parent: "{{name}}/models"
    description: "REST/GraphQL API endpoints"
    commit_message: "Add {{name}} API endpoints"

  - name: "{{name}}/frontend"
    parent: "{{name}}/api"
    description: "{{frontend}} UI components"
    commit_message: "Add {{name}} {{frontend}} components"

  - name: "{{name}}/integration"
    parent: "{{name}}/frontend"
    description: "End-to-end integration tests"
    commit_message: "Add {{name}} integration tests"
```

Using it:

```bash
$ gt create --template full-stack user-dashboard
? Frontend framework: (react)
# Creates 4-branch stack
```

### Template with Files

Templates can scaffold initial files:

```yaml
# ~/.config/stkd/templates/api.yaml
name: api
description: API feature with scaffolded files

branches:
  - name: "{{name}}"
    files:
      - path: "src/models/{{snake_name}}.rs"
        template: |
          pub struct {{PascalName}} {
              pub id: i64,
          }
      - path: "src/routes/{{snake_name}}.rs"
        template: |
          use axum::{
              routing::get,
              Router,
          };

  - name: "{{name}}/tests"
    parent: "{{name}}"
    files:
      - path: "tests/{{snake_name}}.rs"
        template: |
          #[tokio::test]
          async fn test_{{snake_name}}_list() {
              // TODO
          }
```

Variable filters:

| Filter | Input | Output |
|--------|-------|--------|
| `snake_name` | `user-profile` | `user_profile` |
| `PascalName` | `user-profile` | `UserProfile` |
| `kebab-name` | `user_profile` | `user-profile` |
| `camelName` | `user-profile` | `userProfile` |

---

## Template Best Practices

### Keep Templates Focused

Good templates create 2-4 branches. More than that becomes unwieldy:

```yaml
# Good: 3 branches, clear purpose
branches:
  - name: "{{name}}"
  - name: "{{name}}/tests"
  - name: "{{name}}/docs"

# Avoid: 8 branches, too granular
branches:
  - name: "{{name}}/models"
  - name: "{{name}}/validators"
  - name: "{{name}}/controllers"
  # ... etc
```

### Use Descriptive Commit Messages

```yaml
branches:
  - name: "{{name}}"
    commit_message: "Add {{name}} data models and types"
```

This pre-fills the commit when you run `gt modify`, saving time.

### Project-Specific Templates

Store team conventions in the repo:

```yaml
# .stkd/templates/backend-feature.yaml
name: backend-feature
description: Standard backend feature structure for our team

branches:
  - name: "feature/{{name}}"
    commit_message: "[{{ticket}}] Add {{name}} core logic"

  - name: "feature/{{name}}/tests"
    parent: "feature/{{name}}"
    commit_message: "[{{ticket}}] Add {{name}} tests"

  - name: "feature/{{name}}/migrations"
    parent: "feature/{{name}}"
    commit_message: "[{{ticket}}] Add {{name}} database migrations"
```

Commit `.stkd/templates/` to version control so the whole team shares the same templates.

---

## Template Use Cases

### Onboarding New Features

New team members use templates to follow team conventions:

```bash
# Alice shows Bob the template workflow
gt create --template backend-feature user-preferences
# Bob gets the standard structure without asking
```

### Experiments

Quickly scaffold experimental work that might be abandoned:

```bash
gt create --template experiment new-cache-layer
# Creates experiment/new-cache-layer and experiment/new-cache-layer/benchmarks
```

### Hotfix Workflow

Standardize emergency fixes:

```bash
gt create --template hotfix production-crash
# Creates: fix/production-crash, fix/production-crash/tests, fix/production-crash/rollback
```

---

## Managing Templates

### Validate a Template

```bash
# Check if a template is valid without creating branches
gt create --template api test-name --dry-run
```

### Update Templates

```bash
# Refresh built-in templates (after Stack upgrade)
gt templates --update

# List template sources
gt templates --sources
```

### Sharing Templates

Share templates across your team by committing them to the repo:

```bash
# Add to your repo
git add .stkd/templates/
git commit -m "Add team Stack templates"

# Team members get them automatically
gt create --list-templates
# Shows built-in + project-specific templates
```

---

## Template Tips

1. **Start with built-ins**: Learn the patterns before creating custom templates
2. **Commit project templates**: Share conventions with your team
3. **Keep it small**: 2-4 branches per template is the sweet spot
4. **Use variables**: `{{name}}`, `{{ticket}}`, `{{date}}` make templates reusable
5. **Document your templates**: Add a `description` field explaining when to use each
6. **Test templates**: Use `--dry-run` to verify before sharing
7. **Evolve over time**: Update templates as team conventions change
