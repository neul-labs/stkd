# Stack Roadmap

This document outlines the planned development roadmap for Stack.

## Current Status

**Version**: 0.1.0 (Alpha)

Stack is in active development. Core features work but the API may change.

## Version Roadmap

### v0.2.0 - Provider Abstraction (In Progress)

**Goal**: Establish pluggable provider architecture

- [x] Create `stack-provider-api` crate with traits
- [x] Implement provider-agnostic types (MergeRequest, Pipeline, etc.)
- [x] Refactor `stack-github` to implement provider traits
- [x] Add enhanced GitHub features (reviews, labels, pipelines)
- [ ] Update CLI to use provider abstraction
- [ ] Configuration migration (v1 → v2)

### v0.3.0 - GitLab Support

**Goal**: Full GitLab support with feature parity

- [ ] Create `stack-gitlab` crate
- [ ] Implement core MR operations
- [ ] Add pipeline status integration
- [ ] Add approval/review support
- [ ] Support GitLab-specific features (fast-forward merge)
- [ ] Self-hosted GitLab support

### v0.4.0 - Documentation & Polish

**Goal**: Production-ready documentation and UX

- [x] mdBook documentation site
- [ ] Comprehensive rustdoc for all public APIs
- [ ] Shell completions (bash, zsh, fish)
- [ ] Improved error messages
- [ ] Progress indicators for long operations
- [ ] Configuration wizard

### v0.5.0 - Advanced Features

**Goal**: Power user features

- [ ] Partial stack submission
- [ ] Stack templates
- [ ] Branch protection integration
- [ ] Auto-restack on PR merge
- [ ] Dry-run mode
- [ ] Undo/redo support

### v1.0.0 - Stable Release

**Goal**: Stable API and feature set

- [ ] API stabilization (no breaking changes)
- [ ] Performance optimization
- [ ] Comprehensive test coverage
- [ ] Security audit
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

1. **GitLab Provider**: Help us achieve feature parity
2. **Documentation**: Improve guides and examples
3. **Testing**: Increase test coverage
4. **Providers**: Add support for new platforms

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

*Last updated: January 2025*
