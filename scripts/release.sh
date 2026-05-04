#!/usr/bin/env bash
#
# release.sh - Bump version, publish crates, tag, and trigger CI/CD
#
# Usage:
#   ./scripts/release.sh [major|minor|patch|VERSION]
#
# Examples:
#   ./scripts/release.sh patch      # 0.1.0 -> 0.1.1
#   ./scripts/release.sh minor      # 0.1.0 -> 0.2.0
#   ./scripts/release.sh 1.0.0      # exact version
#
# The script will:
#   1. Run tests
#   2. Bump version in workspace Cargo.toml
#   3. Update version references in crate Cargo.toml files
#   4. Generate changelog entry
#   5. Commit and tag
#   6. Publish crates to crates.io in dependency order
#   7. Push tag to trigger CI/CD release builds
#

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Crate publish order (dependency order)
PUBLISH_ORDER=(
    "stkd-provider-api"
    "stkd-core"
    "stkd-db"
    "stkd-github"
    "stkd-gitlab"
    "stkd-engine"
    "stkd-server"
    "stkd-cli"
    "stkd-mcp"
)

error() {
    echo -e "${RED}error:${NC} $1" >&2
    exit 1
}

info() {
    echo -e "${BLUE}info:${NC} $1"
}

success() {
    echo -e "${GREEN}success:${NC} $1"
}

warn() {
    echo -e "${YELLOW}warn:${NC} $1"
}

get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

bump_version() {
    local current=$1
    local bump=$2

    local major=$(echo "$current" | cut -d. -f1)
    local minor=$(echo "$current" | cut -d. -f2)
    local patch=$(echo "$current" | cut -d. -f3)

    case "$bump" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            # Assume exact version
            if [[ ! "$bump" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                error "Invalid bump type or version: $bump"
            fi
            echo "$bump"
            return
            ;;
    esac

    echo "${major}.${minor}.${patch}"
}

# Check prerequisites
check_prerequisites() {
    info "Checking prerequisites..."

    if ! command -v cargo &> /dev/null; then
        error "cargo is not installed"
    fi

    if ! cargo login --list &> /dev/null 2>&1; then
        warn "Not logged into crates.io. Run 'cargo login' first."
        read -p "Continue anyway? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    if ! git diff --quiet HEAD; then
        error "Working directory is not clean. Commit or stash changes first."
    fi

    success "Prerequisites OK"
}

# Run tests
run_tests() {
    info "Running tests..."
    cargo test --workspace || error "Tests failed"
    success "Tests passed"
}

# Update version in workspace Cargo.toml
update_workspace_version() {
    local new_version=$1
    info "Updating workspace version to $new_version..."

    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \"[^\"]*\"/version = \"$new_version\"/" Cargo.toml
    else
        sed -i "s/^version = \"[^\"]*\"/version = \"$new_version\"/" Cargo.toml
    fi

    success "Workspace version updated"
}

# Update version references in crate Cargo.toml files
update_crate_versions() {
    local new_version=$1
    info "Updating crate dependency versions..."

    for crate_toml in crates/*/Cargo.toml; do
        # Update path dependency versions
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' -E "s/(stkd-[a-z0-9-]+) = \{ path = \"[^\"]+\", version = \"[^\"]+\" \}/\1 = { path = \"\..\/\1\", version = \"$new_version\" }/" "$crate_toml"
        else
            sed -i -E "s/(stkd-[a-z0-9-]+) = \{ path = \"[^\"]+\", version = \"[^\"]+\" \}/\1 = { path = \"\..\/\1\", version = \"$new_version\" }/" "$crate_toml"
        fi
    done

    success "Crate dependency versions updated"
}

# Generate changelog entry
generate_changelog() {
    local version=$1
    local date=$(date +%Y-%m-%d)

    info "Generating changelog entry..."

    local changelog_file="CHANGELOG.md"
    if [[ ! -f "$changelog_file" ]]; then
        echo "# Changelog" > "$changelog_file"
        echo "" >> "$changelog_file"
        echo "All notable changes to this project will be documented in this file." >> "$changelog_file"
        echo "" >> "$changelog_file"
    fi

    local temp_file=$(mktemp)
    {
        echo "## [$version] - $date"
        echo ""
        git log --pretty=format:"- %s" "$(git describe --tags --abbrev=0 2>/dev/null || echo HEAD~50)"..HEAD 2>/dev/null || true
        echo ""
        echo ""
        cat "$changelog_file"
    } > "$temp_file"

    mv "$temp_file" "$changelog_file"
    success "Changelog updated"
}

# Commit and tag
commit_and_tag() {
    local version=$1
    info "Committing and tagging v$version..."

    git add -A
    git commit -m "release: v$version

$(git log --pretty=format:'- %s' $(git describe --tags --abbrev=0 2>/dev/null || echo HEAD~50)..HEAD 2>/dev/null || true)

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
" || true

    git tag -a "v$version" -m "Release v$version" || error "Failed to create tag"
    success "Tagged v$version"
}

# Publish crates to crates.io
publish_crates() {
    local version=$1
    info "Publishing crates to crates.io..."

    for crate in "${PUBLISH_ORDER[@]}"; do
        info "Publishing $crate v$version..."

        cd "crates/$crate"

        if ! cargo publish --allow-dirty --no-verify 2>/dev/null; then
            warn "cargo publish failed for $crate, trying with verify..."
            cargo publish --allow-dirty || {
                warn "Failed to publish $crate. This may be because it's already published."
                cd "$REPO_ROOT"
                continue
            }
        fi

        cd "$REPO_ROOT"

        # Wait a moment for the index to update
        sleep 5

        success "$crate published"
    done
}

# Push to trigger CI/CD
push_and_trigger() {
    local version=$1
    info "Pushing to remote..."

    git push origin main
    git push origin "v$version"

    success "Pushed. CI/CD will build release binaries."
}

# Main
main() {
    local bump_type="${1:-patch}"

    echo ""
    echo -e "${BLUE}Stack Release Tool${NC}"
    echo ""

    check_prerequisites

    local current_version
    current_version=$(get_current_version)
    info "Current version: $current_version"

    local new_version
    new_version=$(bump_version "$current_version" "$bump_type")
    info "New version: $new_version"

    echo ""
    read -p "Proceed with release v$new_version? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        info "Aborted."
        exit 0
    fi

    run_tests
    update_workspace_version "$new_version"
    update_crate_versions "$new_version"
    generate_changelog "$new_version"
    commit_and_tag "$new_version"
    publish_crates "$new_version"
    push_and_trigger "$new_version"

    echo ""
    success "Release v$new_version complete!"
    echo ""
    echo "  crates.io:   https://crates.io/crates/stkd-cli/$new_version"
    echo "  GitHub tag:   https://github.com/neul-labs/stkd/releases/tag/v$new_version"
    echo ""
}

main "$@"
