# Package Managers

Stack is available through various package managers for different platforms.

## Homebrew (macOS / Linux)

```bash
# Install
brew install neul-labs/tap/stkd

# Update
brew upgrade stkd
```

## Cargo (Cross-platform)

```bash
# Install
cargo install stkd-cli

# Update
cargo install stkd-cli --force
```

## npm (Cross-platform)

```bash
# Install
npm install -g stkd-cli

# Update
npm update -g stkd-cli
```

## pip (Cross-platform)

```bash
# Install
pip install stkd-cli

# Update
pip install --upgrade stkd-cli
```

## Binary Download

Pre-built binaries are available for each release:

### Download Links

Visit the [Releases](https://github.com/neul-labs/stkd/releases) page and download the appropriate binary for your platform:

| Platform | Architecture | Download |
|----------|--------------|----------|
| Linux | x86_64 | `gt-x86_64-unknown-linux-gnu.tar.gz` |
| macOS | Apple Silicon | `gt-aarch64-apple-darwin.tar.gz` |
| Windows | x86_64 | `gt-x86_64-pc-windows-msvc.zip` |

### Installation Steps

**Linux/macOS:**

```bash
# Download and extract
curl -LO https://github.com/neul-labs/stkd/releases/latest/download/gt-x86_64-unknown-linux-gnu.tar.gz
tar xzf gt-x86_64-unknown-linux-gnu.tar.gz

# Move to PATH
sudo mv gt /usr/local/bin/

# Verify
gt --version
```

**Windows (PowerShell):**

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/neul-labs/stkd/releases/latest/download/gt-x86_64-pc-windows-msvc.zip" -OutFile "gt.zip"

# Extract
Expand-Archive -Path "gt.zip" -DestinationPath "C:\Program Files\Stack"

# Add to PATH (run as Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\Stack", "Machine")
```

## Nix (NixOS/Nix)

```nix
# flake.nix
{
  inputs.stack.url = "github:neul-labs/stkd";

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
curl -LO https://github.com/neul-labs/stkd/releases/latest/download/gt-x86_64-unknown-linux-gnu.tar.gz.sig

# Verify (requires GPG and our public key)
gpg --verify gt-x86_64-unknown-linux-gnu.tar.gz.sig gt-x86_64-unknown-linux-gnu.tar.gz
```
