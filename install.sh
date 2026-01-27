#!/usr/bin/env bash
#
# Stkd Installation Script
# https://github.com/neul-labs/stkd
#
# This script installs the Stkd CLI (gt) for managing stacked pull requests.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/neul-labs/stkd/main/install.sh | bash
#
# Options:
#   --help              Show this help message
#   --version VERSION   Install a specific version (default: latest)
#   --no-completions    Skip shell completion installation
#   --source            Force build from source
#

set -euo pipefail

# Configuration
REPO="neul-labs/stkd"
BINARY_NAME="gt"
INSTALL_DIR="${HOME}/.local/bin"
VERSION=""
FORCE_SOURCE=false
INSTALL_COMPLETIONS=true

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_banner() {
    echo -e "${BLUE}"
    echo "      _   _       _ "
    echo "  ___| |_| | ____| |"
    echo " / __| __| |/ / _\` |"
    echo " \\__ \\ |_|   < (_| |"
    echo " |___/\\__|_|\\_\\__,_|"
    echo -e "${NC}"
    echo "Stacked diffs. Simplified."
    echo ""
}

info() {
    echo -e "${BLUE}==>${NC} $1"
}

success() {
    echo -e "${GREEN}==>${NC} $1"
}

warn() {
    echo -e "${YELLOW}Warning:${NC} $1"
}

error() {
    echo -e "${RED}Error:${NC} $1" >&2
    exit 1
}

show_help() {
    cat << EOF
Stkd Installation Script

Usage:
  ./install.sh [options]

Options:
  --help              Show this help message
  --version VERSION   Install a specific version (default: latest)
  --no-completions    Skip shell completion installation
  --source            Force build from source (requires Rust)

Examples:
  ./install.sh                    # Install latest release
  ./install.sh --version v0.1.0   # Install specific version
  ./install.sh --source           # Build from source

For more information, visit https://docs.neullabs.com/stkd
EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_help
            ;;
        --version|-v)
            VERSION="$2"
            shift 2
            ;;
        --no-completions)
            INSTALL_COMPLETIONS=false
            shift
            ;;
        --source)
            FORCE_SOURCE=true
            shift
            ;;
        *)
            error "Unknown option: $1. Use --help for usage."
            ;;
    esac
done

detect_os() {
    local os
    os="$(uname -s)"
    case "$os" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "darwin" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)       error "Unsupported operating system: $os" ;;
    esac
}

detect_arch() {
    local arch
    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64)  echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *)             error "Unsupported architecture: $arch" ;;
    esac
}

check_prerequisites() {
    info "Checking prerequisites..."

    # Check for Git
    if ! command -v git &> /dev/null; then
        error "Git is required but not installed. Please install Git 2.28+ and try again."
    fi

    # Check Git version (need 2.28+ for init.defaultBranch)
    local git_version
    git_version=$(git --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
    local git_major git_minor
    git_major=$(echo "$git_version" | cut -d. -f1)
    git_minor=$(echo "$git_version" | cut -d. -f2)

    if [[ "$git_major" -lt 2 ]] || { [[ "$git_major" -eq 2 ]] && [[ "$git_minor" -lt 28 ]]; }; then
        warn "Git version $git_version detected. Git 2.28+ is recommended for full compatibility."
    fi

    success "Prerequisites check passed"
}

get_latest_version() {
    info "Fetching latest version..."

    if command -v curl &> /dev/null; then
        VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/' || echo "")
    elif command -v wget &> /dev/null; then
        VERSION=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/' || echo "")
    fi

    if [[ -z "$VERSION" ]]; then
        warn "Could not determine latest version. Will attempt to build from source."
        return 1
    fi

    success "Latest version: $VERSION"
    return 0
}

try_download_binary() {
    local os="$1"
    local arch="$2"

    info "Attempting to download pre-built binary..."

    # Construct download URL
    local binary_name="${BINARY_NAME}-${VERSION}-${os}-${arch}"
    if [[ "$os" == "windows" ]]; then
        binary_name="${binary_name}.exe"
    fi

    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${binary_name}"
    local tmp_file
    tmp_file=$(mktemp)

    # Try to download
    local download_success=false
    if command -v curl &> /dev/null; then
        if curl -fsSL "$download_url" -o "$tmp_file" 2>/dev/null; then
            download_success=true
        fi
    elif command -v wget &> /dev/null; then
        if wget -q "$download_url" -O "$tmp_file" 2>/dev/null; then
            download_success=true
        fi
    fi

    if [[ "$download_success" == "true" ]] && [[ -s "$tmp_file" ]]; then
        # Verify it's a valid executable (not HTML error page)
        if file "$tmp_file" 2>/dev/null | grep -q "executable\|ELF\|Mach-O"; then
            mkdir -p "$INSTALL_DIR"
            mv "$tmp_file" "${INSTALL_DIR}/${BINARY_NAME}"
            chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
            success "Binary downloaded and installed to ${INSTALL_DIR}/${BINARY_NAME}"
            return 0
        fi
    fi

    rm -f "$tmp_file" 2>/dev/null || true
    warn "Pre-built binary not available for ${os}-${arch}"
    return 1
}

build_from_source() {
    info "Building from source..."

    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        error "Rust is required to build from source. Install it from https://rustup.rs"
    fi

    local rust_version
    rust_version=$(rustc --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
    local rust_major rust_minor
    rust_major=$(echo "$rust_version" | cut -d. -f1)
    rust_minor=$(echo "$rust_version" | cut -d. -f2)

    if [[ "$rust_major" -lt 1 ]] || { [[ "$rust_major" -eq 1 ]] && [[ "$rust_minor" -lt 70 ]]; }; then
        error "Rust 1.70+ is required. Found version $rust_version. Run 'rustup update' to upgrade."
    fi

    # Clone or use existing repo
    local build_dir
    if [[ -f "Cargo.toml" ]] && grep -q "stkd-cli" "Cargo.toml" 2>/dev/null; then
        # We're in the repo
        build_dir="."
        info "Building from current directory..."
    else
        # Clone the repo
        build_dir=$(mktemp -d)
        info "Cloning repository..."
        git clone --depth 1 "https://github.com/${REPO}.git" "$build_dir"
    fi

    # Build
    info "Compiling (this may take a few minutes)..."
    (cd "$build_dir" && cargo build --release -p stkd-cli)

    # Install
    mkdir -p "$INSTALL_DIR"
    cp "${build_dir}/target/release/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    # Cleanup if we cloned
    if [[ "$build_dir" != "." ]]; then
        rm -rf "$build_dir"
    fi

    success "Built and installed to ${INSTALL_DIR}/${BINARY_NAME}"
}

install_completions() {
    if [[ "$INSTALL_COMPLETIONS" != "true" ]]; then
        return
    fi

    info "Installing shell completions..."

    local gt_bin="${INSTALL_DIR}/${BINARY_NAME}"

    # Bash completions
    if [[ -d "${HOME}/.bash_completion.d" ]] || [[ -f "${HOME}/.bashrc" ]]; then
        mkdir -p "${HOME}/.bash_completion.d"
        "$gt_bin" completions bash > "${HOME}/.bash_completion.d/gt" 2>/dev/null || true
        success "Bash completions installed"
    fi

    # Zsh completions
    if [[ -d "${HOME}/.zsh/completions" ]] || [[ -f "${HOME}/.zshrc" ]]; then
        mkdir -p "${HOME}/.zsh/completions"
        "$gt_bin" completions zsh > "${HOME}/.zsh/completions/_gt" 2>/dev/null || true
        success "Zsh completions installed"
    fi

    # Fish completions
    if [[ -d "${HOME}/.config/fish/completions" ]]; then
        "$gt_bin" completions fish > "${HOME}/.config/fish/completions/gt.fish" 2>/dev/null || true
        success "Fish completions installed"
    fi
}

check_path() {
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        echo ""
        warn "${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "Add it to your shell profile:"
        echo ""
        echo "  # For bash (~/.bashrc or ~/.bash_profile)"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        echo "  # For zsh (~/.zshrc)"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        echo "  # For fish (~/.config/fish/config.fish)"
        echo "  set -gx PATH \$HOME/.local/bin \$PATH"
        echo ""
    fi
}

verify_installation() {
    info "Verifying installation..."

    local gt_bin="${INSTALL_DIR}/${BINARY_NAME}"

    if [[ ! -x "$gt_bin" ]]; then
        error "Installation failed: ${gt_bin} not found or not executable"
    fi

    local installed_version
    installed_version=$("$gt_bin" --version 2>/dev/null || echo "unknown")

    echo ""
    success "Stkd installed successfully!"
    echo ""
    echo "  Binary:  ${gt_bin}"
    echo "  Version: ${installed_version}"
    echo ""
    echo "Get started:"
    echo "  cd your-repo"
    echo "  gt init"
    echo "  gt auth login"
    echo ""
    echo "Documentation: https://docs.neullabs.com/stkd"
    echo ""
}

main() {
    print_banner

    local os arch
    os=$(detect_os)
    arch=$(detect_arch)

    info "Detected: ${os}-${arch}"

    check_prerequisites

    # Try binary download first (unless --source specified)
    if [[ "$FORCE_SOURCE" != "true" ]]; then
        if [[ -z "$VERSION" ]]; then
            get_latest_version || true
        fi

        if [[ -n "$VERSION" ]]; then
            if try_download_binary "$os" "$arch"; then
                install_completions
                check_path
                verify_installation
                exit 0
            fi
        fi
    fi

    # Fall back to source build
    build_from_source
    install_completions
    check_path
    verify_installation
}

main "$@"
