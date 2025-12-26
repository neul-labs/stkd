# gt completions

Generate shell completions for Stack.

## Usage

```bash
gt completions <SHELL>
```

## Description

The `completions` command generates shell completion scripts for various shells. These completions enable tab-completion for all Stack commands and options.

## Supported Shells

| Shell | Value |
|-------|-------|
| Bash | `bash` |
| Zsh | `zsh` |
| Fish | `fish` |
| PowerShell | `powershell` |
| Elvish | `elvish` |

## Installation

### Bash

```bash
# Add to ~/.bashrc
gt completions bash >> ~/.bashrc

# Or install system-wide
gt completions bash | sudo tee /etc/bash_completion.d/gt

# Reload
source ~/.bashrc
```

### Zsh

```bash
# Add to ~/.zshrc
gt completions zsh >> ~/.zshrc

# Or add to fpath (recommended)
gt completions zsh > ~/.zsh/completions/_gt

# Make sure fpath includes the directory
# Add to ~/.zshrc:
# fpath=(~/.zsh/completions $fpath)
# autoload -Uz compinit && compinit

# Reload
source ~/.zshrc
```

### Fish

```bash
# Install to completions directory
gt completions fish > ~/.config/fish/completions/gt.fish

# Completions are automatically loaded
```

### PowerShell

```powershell
# Add to your PowerShell profile
gt completions powershell >> $PROFILE

# Reload profile
. $PROFILE
```

## What Gets Completed

With completions installed, pressing Tab will complete:

- **Commands**: `gt sub<TAB>` → `gt submit`
- **Options**: `gt submit --<TAB>` shows all options
- **Option values**: `gt submit --method <TAB>` shows merge/squash/rebase

## Examples

```bash
# Complete command
gt cre<TAB>
# → gt create

# Complete options
gt submit --dr<TAB>
# → gt submit --draft

# Show all subcommands
gt <TAB><TAB>
# Shows: create, submit, sync, land, log, ...
```

## Updating Completions

After updating Stack, regenerate completions to include new commands:

```bash
# Bash
gt completions bash > /etc/bash_completion.d/gt

# Zsh
gt completions zsh > ~/.zsh/completions/_gt

# Fish
gt completions fish > ~/.config/fish/completions/gt.fish
```

## Troubleshooting

### Completions Not Working

1. Ensure the completion script is sourced:
   ```bash
   # Bash
   source ~/.bashrc

   # Zsh
   compinit
   ```

2. Check if completions are loaded:
   ```bash
   # Bash
   complete -p gt

   # Zsh
   echo $fpath
   ```

### Zsh: Command Not Found

Add to `~/.zshrc`:
```bash
autoload -Uz compinit && compinit
```

## See Also

- [Installation](../installation/README.md) - Full installation instructions
- [gt config](./config.md) - Configure Stack settings
