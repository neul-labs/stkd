# Stack Roadmap

This document outlines the planned development roadmap for Stack.

## Current Status

**Version**: 0.6.0 (In Progress)

Stack has a solid foundation with full GitHub and GitLab support, plus advanced
features including undo/redo, stack templates, and auto-sync with watch mode.
Recent focus has been on security hardening and test infrastructure.
Next milestone: v1.0.0 stable release.

## Version Roadmap

### v0.2.0 - Provider Abstraction (Complete)

**Goal**: Establish pluggable provider architecture

- [x] Create `stack-provider-api` crate with traits
- [x] Implement provider-agnostic types (MergeRequest, Pipeline, etc.)
- [x] Refactor `stack-github` to implement provider traits
- [x] Add enhanced GitHub features (reviews, labels, pipelines)
- [x] Update CLI to use provider abstraction
- [x] Configuration migration (v1 → v2)

### v0.3.0 - GitLab Support (Complete)

**Goal**: Full GitLab support with feature parity

- [x] Create `stack-gitlab` crate
- [x] Implement core MR operations
- [x] Add pipeline status integration
- [x] Add approval/review support
- [x] Support GitLab-specific features (fast-forward merge)
- [x] Self-hosted GitLab support

### v0.4.0 - Documentation & Polish (Complete)

**Goal**: Production-ready documentation and UX

- [x] mdBook documentation site
- [x] Comprehensive rustdoc for all public APIs
- [x] Shell completions (bash, zsh, fish)
- [x] Improved error messages with recovery hints
- [x] Progress indicators for long operations
- [x] Configuration wizard

### v0.5.0 - Advanced Features (Complete)

**Goal**: Power user features

- [x] Partial stack submission (`--only`, `--from`, `--to` flags)
- [x] Dry-run mode for sync, restack, submit, land
- [x] Undo/redo support (`gt undo`, `gt redo`)
- [x] Stack templates (`gt create --template <name>`, `gt create --list-templates`)
- [x] Branch protection integration (provider API support)
- [x] Auto-restack on PR merge (`gt sync --watch`)

### v0.6.0 - Security & Testing (In Progress)

**Goal**: Production-ready security and test coverage

- [x] Security: JWT secret validation (requires env var or explicit dev mode)
- [x] Security: OAuth CSRF protection with state validation
- [x] Fix all compiler warnings across workspace
- [x] Integration test infrastructure (18 CLI end-to-end tests)
- [x] 98 total tests passing across all crates
- [ ] Database integration tests
- [ ] Provider mock tests with wiremock

### v1.0.0 - Stable Release

**Goal**: Stable API and feature set

- [ ] API stabilization (no breaking changes)
- [ ] Performance optimization
- [x] Comprehensive test coverage (98 tests)
- [x] Security audit (JWT + OAuth hardened)
- [ ] Multi-platform binaries
- [ ] Package manager distribution

## Future Considerations

### Additional Providers

| Provider | Priority | Status |
|----------|----------|--------|
| Gitea | Medium | Planned |
| Bitbucket | Low | Considering |
| Azure DevOps | Low | Considering |

### Integration Features

- **CI/CD Integration**: Trigger pipelines, view status
- **Issue Tracking**: Link stacks to issues
- **Code Review**: Request reviewers, view comments
- **Merge Queue**: Integration with GitHub merge queue

### Workflow Enhancements

- **Team Features**: Shared stacks, handoff workflows
- **Visualization**: Web UI for stack visualization
- **Analytics**: Stack metrics and patterns
- **IDE Integration**: VS Code extension

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./docs/src/contributing/README.md) for guidelines.

### Priority Areas

1. **Testing**: Database and provider mock tests
2. **Documentation**: Improve guides and examples
3. **Providers**: Add Gitea support
4. **Performance**: Optimize sync and restack operations

### How to Get Involved

1. Check [open issues](https://github.com/yourusername/stack/issues)
2. Join discussions on feature design
3. Submit pull requests
4. Report bugs and suggest features

## Release Schedule

We aim for monthly releases with the following pattern:

- **Minor versions** (0.x.0): New features, may have breaking changes
- **Patch versions** (0.x.y): Bug fixes, no breaking changes

After v1.0.0:
- **Major versions** (x.0.0): Breaking changes
- **Minor versions** (x.y.0): New features, backward compatible
- **Patch versions** (x.y.z): Bug fixes

## Feedback

Your feedback shapes our roadmap! Let us know what features matter most:

- [GitHub Discussions](https://github.com/yourusername/stack/discussions)
- [GitHub Issues](https://github.com/yourusername/stack/issues)

---

*Last updated: January 2026*
