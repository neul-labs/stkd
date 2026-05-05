# Interactive Terminal UI

Stack includes a full interactive terminal UI (TUI) that lets you browse stacks, view status, and execute operations with keyboard-driven workflows.

---

## Launching the TUI

Run the `tui` command from any Stack-enabled repository:

```bash
gt tui
```

The TUI takes over your terminal with an alternate screen. When you exit, your terminal returns to normal.

!!! tip "Exit Cleanly"
    If Stack is not initialized in the current directory, the TUI exits immediately with a helpful error message before entering alternate screen mode. Run `gt init` first.

---

## Interface Overview

The TUI has three tabs, a status bar, and supports modals and notifications.

```
┌─────────────────────────────────────────────────────────────┐
│                    Stack TUI                                  │
│  [ Stacks ]  [ Status ]  [ Actions ]                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Trunk: main                                                │
│                                                             │
│  ─ ○ feature/auth-models #42 [open]                         │
│  └─◉ feature/auth-api   #43 [active]                        │
│     └── ○ feature/auth-ui #44 [draft]                       │
│                                                             │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  feature/auth-api    1 stack, 3 branches    ? help q quit  │
└─────────────────────────────────────────────────────────────┘
```

---

## Tabs

### Stacks Tab

The default view shows all your stacks as an interactive ASCII tree.

**Navigation:**

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down to next branch |
| `k` / `↑` | Move up to previous branch |
| `h` / `←` | Previous stack |
| `l` / `→` | Next stack |
| `Enter` | Checkout selected branch |

**Visual indicators:**

- `◉` — Current branch (where HEAD is)
- `○` — Other branch in the stack
- `─` / `├─` / `└─` — Tree connectors showing hierarchy
- `[active]` — The branch you're currently on
- `[selected]` — The branch your cursor is on
- `#42` — Merge request number (fetched from provider)
- `[open]` / `[merged]` / `[closed]` / `[draft]` — MR state (fetched with `g`)

### Status Tab

Shows detailed information about the selected branch:

- **Branch** — Name, parent branch, MR number and URL
- **Merge Request** — State, mergeable status, labels (fetched from provider)
- **Stack Position** — Position in stack, branches above and below
- **Working Tree** — Staged, modified, and untracked file counts

### Actions Tab

A quick reference of all available keyboard shortcuts.

---

## Keybindings

### Global

| Key | Action |
|-----|--------|
| `q` | Quit the TUI |
| `Ctrl+C` | Force quit |
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |
| `?` | Toggle help overlay |
| `g` | Refresh stacks and fetch MR status from provider |

### Stack Operations

| Key | Action |
|-----|--------|
| `c` | Create new branch (opens input modal) |
| `d` | Delete selected branch (confirmation modal) |
| `s` | Submit current branch/stack |
| `y` | Sync with remote |
| `r` | Restack branches |
| `u` | Move up one branch |
| `n` | Move down one branch |
| `t` | Jump to top of stack |
| `b` | Jump to bottom of stack |

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Previous stack |
| `l` / `→` | Next stack |
| `Enter` | Checkout selected branch |

---

## Performing Operations

### Creating a Branch

1. Press `c`
2. Type the branch name in the modal
3. Press `Enter` to confirm or `Esc` to cancel

Stack creates the branch on top of the currently selected branch and tracks it.

### Checking Out a Branch

1. Navigate to the branch with `j`/`k`
2. Press `Enter`

The branch is checked out and the stack tree updates to show the new active branch.

### Deleting a Branch

1. Navigate to the branch with `j`/`k`
2. Press `d`
3. Confirm with `y`/`Enter` or cancel with `n`/`Esc`

!!! warning "Cannot Delete Trunk"
    The trunk branch (e.g., `main`) cannot be deleted from the TUI.

### Submitting PRs

1. Navigate to the branch you want to submit
2. Press `s`

A spinner appears in the status bar while Stack pushes branches and creates/updates merge requests. When complete, a notification toast appears with the result:

```
✓ Submitted 2 MRs, updated 1 MR
```

!!! note "Provider Required"
    Submit requires authentication. If not authenticated, the TUI shows an error notification: "No provider configured. Run 'gt auth' first."

### Syncing

Press `y` to run `gt sync`:

- Fetches latest changes from remote
- Deletes merged branches
- Restacks dependent branches
- Shows a notification when complete

### Restacking

Press `r` to run `gt restack` on the current stack:

- Rebases all branches onto their updated parents
- Shows progress in the status bar
- If conflicts occur, exit the TUI and resolve with `gt continue`

### Fetching Provider Status

Press `g` to refresh:

- Reloads the stack structure from local metadata
- Fetches working tree status
- Queries the provider (GitHub/GitLab) for MR state, mergeable status, and labels

The MR state badges (`[open]`, `[merged]`, `[closed]`, `[draft]`) update in real time as results come in.

---

## Notifications

Toast notifications appear at the bottom of the screen for operation results:

```
┌────────────────────────────────────────────┐
│ ✓ Created branch 'feature/new-thing'       │
└────────────────────────────────────────────┘
```

- Green with checkmark — Success
- Red with X — Error

Notifications auto-dismiss after 5 seconds.

---

## Modals

### Confirm Modal

```
┌──────────────────────────┐
│      Delete Branch       │
│                          │
│ Are you sure you want    │
│ to delete 'feature/x'?   │
│                          │
│ y = yes  n = no          │
│ esc = cancel             │
└──────────────────────────┘
```

- `y` or `Enter` — Confirm
- `n` or `Esc` — Cancel

### Input Modal

```
┌──────────────────────────┐
│      Create Branch       │
│                          │
│ Enter branch name:       │
│                          │
│ > feature/my-branch      │
│                          │
│ Enter = confirm          │
│ Esc = cancel             │
└──────────────────────────┘
```

- Type the name
- `Enter` to confirm
- `Backspace` to delete characters
- `Esc` to cancel

### Progress Modal

Shown during async operations:

```
┌──────────────────────────┐
│      Submitting...       │
│                          │
│ ⠋ Submitting MRs...      │
│                          │
│ Please wait...           │
└──────────────────────────┘
```

---

## Help Overlay

Press `?` anywhere to show the help overlay:

```
┌──────────────────────────────────────────────┐
│              Stack TUI Help                  │
│                                              │
│ Navigation:                                  │
│   j/↓  Move down    k/↑  Move up            │
│   h/←  Prev stack   l/→  Next stack         │
│   Tab  Next tab     Shift+Tab  Prev tab    │
│                                              │
│ Actions:                                     │
│   Enter  Checkout   c  Create branch        │
│   d  Delete branch  s  Submit               │
│   y  Sync           r  Restack              │
│   u  Move up        n  Move down            │
│   t  Go to top      b  Go to bottom         │
│   g  Refresh        l  Land (merge)         │
│                                              │
│ Global:                                      │
│   q  Quit           ?  Toggle help           │
│   Ctrl+C  Force quit                         │
│                                              │
│  Press ? or Esc to close                    │
└──────────────────────────────────────────────┘
```

Press `?` or `Esc` again to close.

---

## TUI vs CLI

| Use TUI when... | Use CLI when... |
|-----------------|-----------------|
| Exploring your stack visually | Running commands in scripts |
| Quickly checking status | CI/CD pipelines |
| Interactive branch navigation | One-off commands |
| Browsing multiple stacks | Piping output to other tools |
| Getting a quick overview | Using `--json` or `--quiet` modes |

---

## Tips

- **Start with the Stacks tab** to get your bearings
- **Press `g` frequently** to keep MR status up to date
- **Use `Enter` to checkout** instead of typing branch names
- **The status bar shows spinner activity** during async operations
- **Errors appear as red toasts** — they don't block the UI
- **If the TUI feels stuck, press `q`** — the terminal restore is reliable
