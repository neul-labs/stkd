# Package Managers

Stack is available through various package managers for different platforms.

## Homebrew (macOS/Linux)

```bash
# Install
brew install yourusername/tap/stack

# Update
brew upgrade stack
```

## Cargo (Cross-platform)

```bash
# Install
cargo install stack-cli

# Update
cargo install stack-cli --force
```

## Binary Download

Pre-built binaries are available for each release:

### Download Links

Visit the [Releases](https://github.com/yourusername/stack/releases) page and download the appropriate binary for your platform:

| Platform | Architecture | Download |
|----------|--------------|----------|
| Linux | x86_64 | `stack-x86_64-unknown-linux-gnu.tar.gz` |
| Linux | aarch64 | `stack-aarch64-unknown-linux-gnu.tar.gz` |
| macOS | x86_64 | `stack-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon | `stack-aarch64-apple-darwin.tar.gz` |
| Windows | x86_64 | `stack-x86_64-pc-windows-msvc.zip` |

### Installation Steps

**Linux/macOS:**

```bash
# Download and extract
curl -LO https://github.com/yourusername/stack/releases/latest/download/stack-x86_64-unknown-linux-gnu.tar.gz
tar xzf stack-x86_64-unknown-linux-gnu.tar.gz

# Move to PATH
sudo mv gt /usr/local/bin/

# Verify
gt --version
```

**Windows (PowerShell):**

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/yourusername/stack/releases/latest/download/stack-x86_64-pc-windows-msvc.zip" -OutFile "stack.zip"

# Extract
Expand-Archive -Path "stack.zip" -DestinationPath "C:\Program Files\Stack"

# Add to PATH (run as Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\Stack", "Machine")
```

## Nix (NixOS/Nix)

```nix
# flake.nix
{
  inputs.stack.url = "github:yourusername/stack";

  # Use as:
  # stack.packages.x86_64-linux.default
}
```

Or with nix-env:

```bash
nix-env -iA nixpkgs.stack
```

## Arch Linux (AUR)

```bash
# Using yay
yay -S stack-git

# Using paru
paru -S stack-git
```

## Verifying Downloads

All release binaries are signed. You can verify the signature:

```bash
# Download the signature
curl -LO https://github.com/yourusername/stack/releases/latest/download/stack-x86_64-unknown-linux-gnu.tar.gz.sig

# Verify (requires GPG and our public key)
gpg --verify stack-x86_64-unknown-linux-gnu.tar.gz.sig stack-x86_64-unknown-linux-gnu.tar.gz
```
