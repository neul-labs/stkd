#!/usr/bin/env bash
#
# Stack Demo Script
# https://github.com/neul-labs/stack
#
# This script demonstrates the Stack workflow using a temporary repository.
#
# Usage:
#   ./demo.sh          # Interactive mode (pauses between steps)
#   ./demo.sh --auto   # Automated mode (no pauses, for CI)
#

set -euo pipefail

# Configuration
AUTO_MODE=false
DEMO_DIR=""
GT_BIN=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --auto|-a)
            AUTO_MODE=true
            shift
            ;;
        --help|-h)
            echo "Stack Demo Script"
            echo ""
            echo "Usage:"
            echo "  ./demo.sh          # Interactive mode"
            echo "  ./demo.sh --auto   # Automated mode (no pauses)"
            echo ""
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

cleanup() {
    if [[ -n "$DEMO_DIR" ]] && [[ -d "$DEMO_DIR" ]]; then
        rm -rf "$DEMO_DIR" 2>/dev/null || true
    fi
}

trap cleanup EXIT

print_banner() {
    clear
    echo -e "${BLUE}${BOLD}"
    echo "  ____  _             _      ____"
    echo " / ___|| |_ __ _  ___| | __ |  _ \\  ___ _ __ ___   ___"
    echo " \\___ \\| __/ _\` |/ __| |/ / | | | |/ _ \\ '_ \` _ \\ / _ \\"
    echo "  ___) | || (_| | (__|   <  | |_| |  __/ | | | | | (_) |"
    echo " |____/ \\__\\__,_|\\___|_|\\_\\ |____/ \\___|_| |_| |_|\\___/"
    echo -e "${NC}"
    echo -e "${DIM}Stacked diffs. Simplified.${NC}"
    echo ""
}

section() {
    echo ""
    echo -e "${CYAN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}${BOLD}  $1${NC}"
    echo -e "${CYAN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

explain() {
    echo -e "${DIM}$1${NC}"
    echo ""
}

show_command() {
    echo -e "  ${YELLOW}\$${NC} ${BOLD}$1${NC}"
}

run_command() {
    local cmd="$1"
    local show_output="${2:-true}"

    show_command "$cmd"
    echo ""

    if [[ "$show_output" == "true" ]]; then
        eval "$cmd" 2>&1 | sed 's/^/    /'
    else
        eval "$cmd" > /dev/null 2>&1
    fi
    echo ""
}

pause() {
    if [[ "$AUTO_MODE" != "true" ]]; then
        echo -e "${DIM}Press Enter to continue...${NC}"
        read -r
    else
        sleep 0.5
    fi
}

find_gt_binary() {
    # Check common locations
    if [[ -x "./target/release/gt" ]]; then
        GT_BIN="./target/release/gt"
    elif [[ -x "./target/debug/gt" ]]; then
        GT_BIN="./target/debug/gt"
    elif command -v gt &> /dev/null; then
        GT_BIN="gt"
    else
        echo -e "${RED}Error: Could not find the 'gt' binary.${NC}"
        echo ""
        echo "Please build Stack first:"
        echo "  cargo build --release"
        echo ""
        exit 1
    fi

    # Make path absolute
    if [[ "$GT_BIN" != "gt" ]]; then
        GT_BIN="$(cd "$(dirname "$GT_BIN")" && pwd)/$(basename "$GT_BIN")"
    fi

    echo -e "${GREEN}Using:${NC} $GT_BIN"
}

setup_demo_repo() {
    section "Setting Up Demo Repository"

    explain "We'll create a temporary Git repository to demonstrate Stack's workflow."

    DEMO_DIR=$(mktemp -d)
    cd "$DEMO_DIR"

    run_command "git init demo-project"
    cd demo-project

    # Configure git for the demo
    git config user.email "demo@example.com"
    git config user.name "Demo User"

    # Create initial commit
    echo "# Demo Project" > README.md
    echo "" >> README.md
    echo "A sample project to demonstrate stacked diffs." >> README.md
    git add README.md
    git commit -m "Initial commit"

    echo -e "${GREEN}Created demo repository at:${NC} $(pwd)"
    pause
}

init_stack() {
    section "Step 1: Initialize Stack"

    explain "The 'gt init' command sets up Stack in your repository.
It detects your Git remote and configures the trunk branch (usually 'main')."

    run_command "$GT_BIN init --trunk main"

    explain "Stack stores its configuration in .git/stack/"
    run_command "ls -la .git/stack/"

    pause
}

create_first_branch() {
    section "Step 2: Create Your First Stacked Branch"

    explain "Let's create a feature branch. Stack tracks branch relationships automatically.
This is the BASE of our stack - it depends on main."

    run_command "$GT_BIN create feature/auth-base"

    # Add some code
    cat > auth.py << 'EOF'
"""Authentication module."""

class User:
    """Represents a user in the system."""

    def __init__(self, username: str, email: str):
        self.username = username
        self.email = email
        self.is_authenticated = False

    def __repr__(self):
        return f"User({self.username})"
EOF

    git add auth.py
    git commit -m "Add basic User class"

    explain "We've created a branch and added some code:"
    run_command "cat auth.py"

    pause
}

create_second_branch() {
    section "Step 3: Stack Another Branch on Top"

    explain "Now let's add OAuth support. This branch DEPENDS on the auth-base branch.
Stack knows that auth-oauth is built on top of auth-base."

    run_command "$GT_BIN create feature/auth-oauth"

    # Add OAuth code
    cat >> auth.py << 'EOF'

class OAuthProvider:
    """OAuth authentication provider."""

    def __init__(self, client_id: str, client_secret: str):
        self.client_id = client_id
        self.client_secret = client_secret

    def authenticate(self, user: User, token: str) -> bool:
        """Authenticate user with OAuth token."""
        # In real code, this would validate with the OAuth provider
        user.is_authenticated = True
        return True
EOF

    git add auth.py
    git commit -m "Add OAuth provider support"

    explain "Our stack now has two branches:"
    run_command "$GT_BIN log"

    pause
}

create_third_branch() {
    section "Step 4: Add One More Layer"

    explain "Let's add two-factor authentication. This depends on OAuth, which depends on auth-base.
We now have a 3-layer stack!"

    run_command "$GT_BIN create feature/auth-2fa"

    # Add 2FA code
    cat >> auth.py << 'EOF'

class TwoFactorAuth:
    """Two-factor authentication handler."""

    def __init__(self, user: User):
        self.user = user
        self.is_verified = False

    def send_code(self) -> str:
        """Send verification code to user."""
        import random
        code = str(random.randint(100000, 999999))
        print(f"Sending code to {self.user.email}")
        return code

    def verify(self, code: str, expected: str) -> bool:
        """Verify the 2FA code."""
        self.is_verified = (code == expected)
        return self.is_verified
EOF

    git add auth.py
    git commit -m "Add two-factor authentication"

    pause
}

show_stack() {
    section "Step 5: Visualize the Stack"

    explain "The 'gt log' command shows your entire stack as a tree.
The current branch is highlighted. Each branch shows its relationship to others."

    run_command "$GT_BIN log"

    explain "The stack structure is:"
    echo -e "
    ${CYAN}main${NC} (trunk)
      └── ${GREEN}feature/auth-base${NC}     <- Base authentication
            └── ${GREEN}feature/auth-oauth${NC}  <- OAuth layer (depends on auth-base)
                  └── ${GREEN}feature/auth-2fa${NC}   <- 2FA layer (depends on oauth)  ${YELLOW}← you are here${NC}
    "

    pause
}

navigate_stack() {
    section "Step 6: Navigate the Stack"

    explain "Stack provides commands to move between branches in your stack.
Let's navigate around!"

    explain "Move DOWN toward the base (main):"
    run_command "$GT_BIN down"
    run_command "git branch --show-current"

    explain "Move DOWN again:"
    run_command "$GT_BIN down"
    run_command "git branch --show-current"

    explain "Jump to the TOP of the stack:"
    run_command "$GT_BIN top"
    run_command "git branch --show-current"

    explain "Jump to the BOTTOM (closest to main):"
    run_command "$GT_BIN bottom"
    run_command "git branch --show-current"

    explain "Move UP toward the tip:"
    run_command "$GT_BIN up"
    run_command "git branch --show-current"

    pause
}

modify_middle_branch() {
    section "Step 7: Modify a Middle Branch"

    explain "One of Stack's killer features: modifying a branch in the MIDDLE of your stack.
Let's go back to auth-base and make a change."

    run_command "$GT_BIN checkout feature/auth-base"

    explain "Adding a method to the User class:"

    # Modify the User class
    cat > auth.py << 'EOF'
"""Authentication module."""

class User:
    """Represents a user in the system."""

    def __init__(self, username: str, email: str):
        self.username = username
        self.email = email
        self.is_authenticated = False

    def __repr__(self):
        return f"User({self.username})"

    def get_display_name(self) -> str:
        """Return display name for UI."""
        return self.username.title()
EOF

    git add auth.py
    git commit -m "Add get_display_name method"

    explain "We've modified auth-base. But auth-oauth and auth-2fa were built on the OLD version!
They need to be rebased to include our changes."

    run_command "$GT_BIN log"

    pause
}

restack() {
    section "Step 8: Restack - The Magic of Stacked Diffs"

    explain "The 'gt restack' command automatically rebases all dependent branches.
Watch what happens - Stack will rebase oauth, then 2fa, in the correct order!"

    run_command "$GT_BIN restack"

    explain "All branches are now up to date with the changes in auth-base!"
    run_command "$GT_BIN log"

    explain "Let's verify that auth-2fa has all the code from every layer:"
    run_command "$GT_BIN top"
    run_command "cat auth.py"

    pause
}

show_status() {
    section "Step 9: Check Stack Status"

    explain "The 'gt status' command shows detailed information about your stack,
including which branches have changes and PR status."

    run_command "$GT_BIN status"

    pause
}

final_summary() {
    section "Demo Complete!"

    echo -e "${GREEN}${BOLD}You've just learned the core Stack workflow:${NC}"
    echo ""
    echo -e "  ${CYAN}1.${NC} ${BOLD}gt init${NC}     - Initialize Stack in a repository"
    echo -e "  ${CYAN}2.${NC} ${BOLD}gt create${NC}   - Create stacked branches"
    echo -e "  ${CYAN}3.${NC} ${BOLD}gt log${NC}      - Visualize your stack"
    echo -e "  ${CYAN}4.${NC} ${BOLD}gt up/down${NC}  - Navigate between branches"
    echo -e "  ${CYAN}5.${NC} ${BOLD}gt modify${NC}   - Amend commits"
    echo -e "  ${CYAN}6.${NC} ${BOLD}gt restack${NC}  - Rebase dependent branches"
    echo ""
    echo -e "${BOLD}Next steps in a real project:${NC}"
    echo ""
    echo -e "  ${CYAN}gt auth login${NC}              - Authenticate with GitHub/GitLab"
    echo -e "  ${CYAN}gt submit --stack${NC}          - Create PRs for your entire stack"
    echo -e "  ${CYAN}gt sync${NC}                    - Sync with remote and restack"
    echo -e "  ${CYAN}gt land --stack${NC}            - Merge your stack when approved"
    echo ""
    echo -e "${DIM}Documentation: https://docs.neullabs.com/stack${NC}"
    echo -e "${DIM}Repository:    https://github.com/neul-labs/stack${NC}"
    echo ""

    if [[ "$AUTO_MODE" != "true" ]]; then
        echo -e "${DIM}The demo repository will be cleaned up when you press Enter.${NC}"
        read -r
    fi
}

main() {
    print_banner

    echo "This demo will walk you through the Stack workflow."
    echo "We'll create a temporary repository and show you how to:"
    echo ""
    echo "  - Create stacked branches"
    echo "  - Navigate your stack"
    echo "  - Modify branches and restack"
    echo ""

    if [[ "$AUTO_MODE" != "true" ]]; then
        echo -e "${DIM}Press Enter to begin...${NC}"
        read -r
    fi

    find_gt_binary
    setup_demo_repo
    init_stack
    create_first_branch
    create_second_branch
    create_third_branch
    show_stack
    navigate_stack
    modify_middle_branch
    restack
    show_status
    final_summary
}

main
