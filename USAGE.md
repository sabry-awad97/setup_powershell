# Usage Guide

This guide explains how to use the PowerShell 7 Setup Automation tool and all the features it configures.

## Running the Tool

### Basic Usage

From the project directory, run:

```cmd
cargo run --release
```

### Expected Output

The tool provides colored, emoji-enhanced output to show progress:

```
🔍 Checking latest PowerShell release...
✅ Latest PowerShell: v7.4.1
⬇ Downloading PowerShell-7.4.1-win-x64.msi ...
✅ Downloaded to C:\Users\...\Temp\PowerShell-7.4.1-win-x64.msi
⚙ Installing PowerShell 7 (this may need admin rights)...
✅ PowerShell 7 installed.

📦 Installing PowerShell modules...
➡ Install-Module PSReadLine -Force -Scope CurrentUser
➡ Install-Module posh-git -Force -Scope CurrentUser
➡ winget install JanDeDobbeleer.OhMyPosh -s winget

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Setup completed successfully!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 Profile written to: C:\Users\YourName\Documents\PowerShell\Microsoft.PowerShell_profile.ps1

🔄 Restart PowerShell 7 (pwsh) to see the changes.
```

## First Run Experience

### When PowerShell 7 Is Not Installed

If PowerShell 7 is not already on your system, the tool will:

1. Detect that `pwsh` is not available
2. Download the latest PowerShell 7 MSI installer from GitHub
3. Install it using `msiexec` (requires admin rights)
4. Display a message about restarting

**You'll see this message:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PowerShell 7 installation completed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ℹ PowerShell 7 has been installed but is not yet available in the current session.

To complete the setup:
  1. Open a new terminal window
  2. Run this program again: cargo run --release

💡 Alternatively, restart your PC for system-wide PATH updates.
```

### Two-Phase Installation Process

**Why two phases?**

- PowerShell 7 installation updates the system PATH environment variable
- Your current terminal session has a snapshot of the old PATH
- The new `pwsh` command won't be available until you start a fresh terminal

**What to do:**

1. Close your current terminal
2. Open a new Command Prompt or PowerShell window
3. Navigate back to the project directory: `cd path\to\setup_powershell`
4. Run the tool again: `cargo run --release`

The second run will:

- Detect that PowerShell 7 is now available
- Install the PowerShell modules (PSReadLine, posh-git)
- Install Oh-My-Posh
- Create your custom profile

### When PowerShell 7 Is Already Installed

If `pwsh` is already available, the tool runs in a single phase:

- Installs modules
- Installs Oh-My-Posh
- Creates the profile
- Completes immediately

## What Gets Configured

### 1. PowerShell 7 Installation

- **Version**: Latest stable release from GitHub
- **Location**: `C:\Program Files\PowerShell\7\`
- **Command**: `pwsh` (added to PATH)

### 2. Module Installations

**PSReadLine** - Enhanced command-line editing

- Installed from PowerShell Gallery
- Scope: CurrentUser (no admin required)
- Provides history search, syntax highlighting, and predictions

**posh-git** - Git integration

- Installed from PowerShell Gallery
- Scope: CurrentUser
- Shows Git status in your prompt

### 3. Oh-My-Posh Installation

- Installed via winget
- Provides prompt theming
- Default theme: Paradox
- Themes location: `$env:POSH_THEMES_PATH`

### 4. Profile Creation

**Location**: `$PROFILE` (typically `C:\Users\YourName\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`)

**Contents**:

- Module imports (posh-git)
- Oh-My-Posh initialization
- PSReadLine configuration
- Custom aliases and functions

## Using the Configured Environment

### Launch PowerShell 7

After setup completes, launch PowerShell 7:

```cmd
pwsh
```

You'll see the Oh-My-Posh themed prompt instead of the standard `PS>` prompt.

### Git Aliases

The profile includes convenient Git shortcuts:

#### `gs` - Git Status

```powershell
gs
```

Equivalent to: `git status`

**Example output:**

```
On branch main
Your branch is up to date with 'origin/main'.

nothing to commit, working tree clean
```

#### `gc` - Git Commit

```powershell
gc -m "Add new feature"
```

Equivalent to: `git commit -m "Add new feature"`

#### `gp` - Git Push

```powershell
gp
```

Equivalent to: `git push`

You can also specify remote and branch:

```powershell
gp origin main
```

#### `gl` - Git Log (Pretty)

```powershell
gl
```

Equivalent to: `git log --oneline --graph --decorate --all`

Shows a beautiful graph of your commit history.

#### `gco` - Git Checkout

```powershell
gco feature-branch
```

Equivalent to: `git checkout feature-branch`

#### `gb` - Git Branch

```powershell
gb
```

Equivalent to: `git branch`

Lists all branches. Add arguments to create branches:

```powershell
gb new-feature
```

#### `gd` - Git Diff

```powershell
gd
```

Equivalent to: `git diff`

Shows unstaged changes. Add arguments for specific files:

```powershell
gd src/main.rs
```

### Custom Aliases

#### `ll` - List Files

```powershell
ll
```

Equivalent to: `Get-ChildItem`

Lists files and directories in the current location.

#### `la` - List All Files

```powershell
la
```

Equivalent to: `Get-ChildItem -Force`

Lists all files including hidden files and directories.

### History Search

PSReadLine is configured for smart history search:

**Arrow Key Search:**

1. Type the beginning of a command (e.g., `git`)
2. Press `↑` (Up Arrow) to search backward through history for commands starting with "git"
3. Press `↓` (Down Arrow) to search forward

**Example:**

```powershell
git ↑  # Cycles through: git push, git commit, git status, etc.
```

### Inline Predictions

PSReadLine shows predictions from your history as you type:

```powershell
git s█tatus  # Gray text shows predicted completion
```

Press `→` (Right Arrow) to accept the prediction.

### Syntax Highlighting

Commands, parameters, and strings are color-coded:

- **Commands**: Yellow (e.g., `Get-ChildItem`)
- **Parameters**: Green (e.g., `-Force`)
- **Strings**: Magenta (e.g., `"hello"`)
- **Operators**: Dark Cyan (e.g., `|`, `>`)
- **Variables**: White (e.g., `$env:PATH`)

### Oh-My-Posh Prompt

The Paradox theme shows:

- Current directory
- Git branch and status (if in a Git repository)
- Execution time of last command
- Error indicator (if last command failed)

**Example prompt:**

```
~/projects/myapp main ≡ ✓
❯
```

## Profile Location

### Finding Your Profile

To see where your profile is located:

```powershell
$PROFILE
```

**Typical location:**

```
C:\Users\YourName\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
```

### Editing Your Profile

Open the profile in your favorite editor:

```powershell
notepad $PROFILE
```

Or with VS Code:

```powershell
code $PROFILE
```

### Reloading Your Profile

After making changes, reload the profile without restarting PowerShell:

```powershell
. $PROFILE
```

The `.` (dot) is the source operator that executes the profile script.

## Verifying the Setup

### Check PowerShell Version

```powershell
$PSVersionTable.PSVersion
```

Should show version 7.x.x or higher.

### Check Installed Modules

```powershell
Get-Module -ListAvailable PSReadLine, posh-git
```

Both modules should be listed.

### Check Oh-My-Posh

```powershell
oh-my-posh --version
```

Should display the version number.

### Test Profile Loading

```powershell
. $PROFILE
```

Should execute without errors. If you see errors, check [TROUBLESHOOTING.md](TROUBLESHOOTING.md).

## Next Steps

- **Customize Your Setup**: See [CUSTOMIZATION.md](CUSTOMIZATION.md) to change themes, add aliases, and modify settings
- **Troubleshooting**: If something isn't working, check [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- **Learn More**: Explore the [Architecture Documentation](docs/ARCHITECTURE.md) to understand how it works

## Tips

- **Tab Completion**: PowerShell has excellent tab completion. Press `Tab` to cycle through completions.
- **Command History**: Press `Ctrl+R` to search command history interactively.
- **Clear Screen**: Use `cls` or `Ctrl+L` to clear the terminal.
- **Copy/Paste**: Right-click to paste in Windows Terminal. Use `Ctrl+Shift+C/V` for copy/paste.
- **Multiple Tabs**: Windows Terminal supports tabs. Press `Ctrl+Shift+T` for a new tab.
