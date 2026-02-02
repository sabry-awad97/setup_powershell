# PowerShell Profile Reference

This document provides a complete reference for the PowerShell profile created by the setup tool.

## Profile Location

The profile is located at:

```powershell
$PROFILE
```

Typical path:

```
C:\Users\<YourUsername>\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
```

## Complete Profile Template

```powershell
# ===========================
# Modern PowerShell 7 Profile
# ===========================

# --- Import Modules ---
Import-Module posh-git

# Oh-My-Posh prompt theme (change "paradox.omp.json" to your favorite)
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\paradox.omp.json" | Invoke-Expression

# --- PSReadLine Settings ---
Set-PSReadLineOption -PredictionSource History
Set-PSReadLineOption -PredictionViewStyle InlineView
Set-PSReadLineOption -Colors @{ "InlinePrediction" = 'Cyan' }
Set-PSReadLineOption -EditMode Windows

# History search with arrow keys
Set-PSReadLineKeyHandler -Key UpArrow -Function HistorySearchBackward
Set-PSReadLineKeyHandler -Key DownArrow -Function HistorySearchForward

# Syntax colors
Set-PSReadLineOption -Colors @{
    "Command"   = 'Yellow'
    "Parameter" = 'Green'
    "String"    = 'Magenta'
    "Operator"  = 'DarkCyan'
    "Variable"  = 'White'
}

# --- Aliases ---
Set-Alias ll Get-ChildItem
Set-Alias la "Get-ChildItem -Force"

# --- Git Shortcuts (functions instead of aliases) ---
function gs { git status }
function gc { git commit @args }     # usage: gc -m "msg"
function gp { git push @args }
function gl { git log --oneline --graph --decorate --all }
function gco { git checkout @args }
function gb { git branch @args }
function gd { git diff @args }

# --- Environment ---
$env:POSH_GIT_ENABLED = $true
```

## Section-by-Section Breakdown

### 1. Module Imports

```powershell
Import-Module posh-git
```

**Purpose:** Loads the posh-git module for Git integration.

**What it provides:**

- Git status in prompt (branch name, changes)
- Tab completion for Git commands
- Git aliases and functions

**Customization:**
Add more modules:

```powershell
Import-Module Terminal-Icons  # File icons in ls
Import-Module z               # Smart directory navigation
```

### 2. Oh-My-Posh Configuration

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\paradox.omp.json" | Invoke-Expression
```

**Purpose:** Initializes Oh-My-Posh with the Paradox theme.

**Components:**

- `oh-my-posh init pwsh` - Initializes Oh-My-Posh for PowerShell
- `--config "$env:POSH_THEMES_PATH\paradox.omp.json"` - Specifies theme file
- `| Invoke-Expression` - Executes the initialization script

**Theme Location:**
Themes are stored in: `$env:POSH_THEMES_PATH`

Typical path:

```
C:\Users\<User>\AppData\Local\Programs\oh-my-posh\themes\
```

**Customization:**
Change theme:

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\agnoster.omp.json" | Invoke-Expression
```

Use custom theme:

```powershell
oh-my-posh init pwsh --config "C:\path\to\my-theme.omp.json" | Invoke-Expression
```

### 3. PSReadLine Settings

#### Prediction Source

```powershell
Set-PSReadLineOption -PredictionSource History
```

**Options:**

- `History` - Predictions from command history (default)
- `HistoryAndPlugin` - History + plugin predictions
- `None` - Disable predictions

**What it does:**
Shows suggestions based on your previous commands as you type.

#### Prediction View Style

```powershell
Set-PSReadLineOption -PredictionViewStyle InlineView
```

**Options:**

- `InlineView` - Shows prediction as gray text inline (default)
- `ListView` - Shows predictions in a dropdown list

**Example (InlineView):**

```powershell
git s█tatus  # Gray "tatus" appears as you type "git s"
```

#### Inline Prediction Color

```powershell
Set-PSReadLineOption -Colors @{ "InlinePrediction" = 'Cyan' }
```

**Available Colors:**
Black, DarkBlue, DarkGreen, DarkCyan, DarkRed, DarkMagenta, DarkYellow, Gray, DarkGray, Blue, Green, Cyan, Red, Magenta, Yellow, White

**Customization:**

```powershell
Set-PSReadLineOption -Colors @{ "InlinePrediction" = 'DarkGray' }
```

#### Edit Mode

```powershell
Set-PSReadLineOption -EditMode Windows
```

**Options:**

- `Windows` - Windows-style key bindings (default)
- `Emacs` - Emacs-style key bindings (Ctrl+A, Ctrl+E, etc.)
- `Vi` - Vi-style key bindings (Esc for command mode)

**Key Bindings by Mode:**

| Action            | Windows        | Emacs         | Vi (Insert) |
| ----------------- | -------------- | ------------- | ----------- |
| Beginning of line | Home           | Ctrl+A        | Home        |
| End of line       | End            | Ctrl+E        | End         |
| Delete word       | Ctrl+Backspace | Alt+Backspace | Ctrl+W      |
| Delete to end     | Ctrl+End       | Ctrl+K        | -           |

#### History Search Key Bindings

```powershell
Set-PSReadLineKeyHandler -Key UpArrow -Function HistorySearchBackward
Set-PSReadLineKeyHandler -Key DownArrow -Function HistorySearchForward
```

**What it does:**

- Type beginning of command (e.g., `git`)
- Press ↑ to search backward through history for commands starting with "git"
- Press ↓ to search forward

**Example:**

```powershell
git ↑  # Cycles through: git push, git commit, git status, etc.
```

**Customization:**
Use Ctrl+Up/Down instead:

```powershell
Set-PSReadLineKeyHandler -Key Ctrl+UpArrow -Function HistorySearchBackward
Set-PSReadLineKeyHandler -Key Ctrl+DownArrow -Function HistorySearchForward
```

#### Syntax Colors

```powershell
Set-PSReadLineOption -Colors @{
    "Command"   = 'Yellow'
    "Parameter" = 'Green'
    "String"    = 'Magenta'
    "Operator"  = 'DarkCyan'
    "Variable"  = 'White'
}
```

**Color Categories:**

| Category  | Example                  | Default Color |
| --------- | ------------------------ | ------------- |
| Command   | `Get-ChildItem`          | Yellow        |
| Parameter | `-Force`                 | Green         |
| String    | `"hello"`                | Magenta       |
| Operator  | `\|`, `>`, `-eq`         | DarkCyan      |
| Variable  | `$env:PATH`              | White         |
| Comment   | `# comment`              | DarkGreen     |
| Keyword   | `if`, `else`, `function` | Green         |
| Number    | `42`, `3.14`             | White         |

**Customization Example (Dark Theme):**

```powershell
Set-PSReadLineOption -Colors @{
    "Command"   = 'Cyan'
    "Parameter" = 'DarkCyan'
    "String"    = 'DarkGreen'
    "Operator"  = 'DarkGray'
    "Variable"  = 'Green'
    "Comment"   = 'DarkGray'
}
```

### 4. Aliases

#### Simple Aliases

```powershell
Set-Alias ll Get-ChildItem
Set-Alias la "Get-ChildItem -Force"
```

**What they do:**

- `ll` - List files and directories
- `la` - List all files including hidden

**Usage:**

```powershell
ll          # Lists files in current directory
ll C:\      # Lists files in C:\
la          # Lists all files including hidden
```

**Customization:**
Add more aliases:

```powershell
Set-Alias g git
Set-Alias k kubectl
Set-Alias d docker
Set-Alias v vim
Set-Alias .. cd..
```

**Note:** Simple aliases can't accept parameters. For aliases with parameters, use functions.

### 5. Git Shortcuts (Functions)

#### gs - Git Status

```powershell
function gs { git status }
```

**Usage:**

```powershell
gs  # Shows git status
```

**Equivalent to:**

```powershell
git status
```

#### gc - Git Commit

```powershell
function gc { git commit @args }
```

**Usage:**

```powershell
gc -m "Add new feature"
gc -m "Fix bug" --amend
```

**What `@args` does:**
Passes all arguments to the git command.

**Equivalent to:**

```powershell
git commit -m "Add new feature"
git commit -m "Fix bug" --amend
```

#### gp - Git Push

```powershell
function gp { git push @args }
```

**Usage:**

```powershell
gp                  # Push to default remote/branch
gp origin main      # Push to specific remote/branch
gp --force          # Force push
```

#### gl - Git Log (Pretty)

```powershell
function gl { git log --oneline --graph --decorate --all }
```

**Usage:**

```powershell
gl  # Shows pretty git log
```

**What it shows:**

- `--oneline` - One commit per line
- `--graph` - ASCII graph of branches
- `--decorate` - Show branch and tag names
- `--all` - Show all branches

**Example output:**

```
* a1b2c3d (HEAD -> main, origin/main) Add feature
* e4f5g6h Fix bug
* i7j8k9l Initial commit
```

#### gco - Git Checkout

```powershell
function gco { git checkout @args }
```

**Usage:**

```powershell
gco feature-branch      # Switch to branch
gco -b new-branch       # Create and switch to new branch
gco main                # Switch to main branch
```

#### gb - Git Branch

```powershell
function gb { git branch @args }
```

**Usage:**

```powershell
gb                  # List branches
gb new-feature      # Create new branch
gb -d old-branch    # Delete branch
```

#### gd - Git Diff

```powershell
function gd { git diff @args }
```

**Usage:**

```powershell
gd                  # Show unstaged changes
gd --cached         # Show staged changes
gd main..feature    # Compare branches
gd src/main.rs      # Diff specific file
```

### 6. Environment Variables

```powershell
$env:POSH_GIT_ENABLED = $true
```

**Purpose:** Enables posh-git integration.

**Customization:**
Add more environment variables:

```powershell
$env:EDITOR = "code"                    # Default editor
$env:VISUAL = "code"                    # Visual editor
$env:PATH += ";C:\MyTools"              # Add to PATH
$env:POSH_GIT_ENABLED = $true           # Enable posh-git
```

## Advanced Customizations

### Adding Custom Functions

#### Navigation Shortcuts

```powershell
function docs { Set-Location ~/Documents }
function proj { Set-Location ~/Projects }
function dl { Set-Location ~/Downloads }
```

#### Utility Functions

```powershell
# Create and enter directory
function mkcd($dir) {
    New-Item -ItemType Directory -Path $dir -Force
    Set-Location $dir
}

# Find files by name
function ff($name) {
    Get-ChildItem -Recurse -Filter "*$name*" -ErrorAction SilentlyContinue
}

# Touch command (create empty file)
function touch($file) {
    if (Test-Path $file) {
        (Get-Item $file).LastWriteTime = Get-Date
    } else {
        New-Item -ItemType File -Path $file | Out-Null
    }
}

# Which command (find executable)
function which($command) {
    Get-Command $command | Select-Object -ExpandProperty Source
}
```

### Adding More Key Bindings

```powershell
# Ctrl+D to delete character
Set-PSReadLineKeyHandler -Key Ctrl+d -Function DeleteChar

# Ctrl+W to delete word
Set-PSReadLineKeyHandler -Key Ctrl+w -Function BackwardKillWord

# Ctrl+Left/Right for word navigation
Set-PSReadLineKeyHandler -Key Ctrl+LeftArrow -Function BackwardWord
Set-PSReadLineKeyHandler -Key Ctrl+RightArrow -Function ForwardWord

# Ctrl+R for reverse history search
Set-PSReadLineKeyHandler -Key Ctrl+r -Function ReverseSearchHistory
```

### Adding More Modules

```powershell
# Terminal icons for ls
Import-Module Terminal-Icons

# Smart directory navigation
Import-Module z

# Azure CLI integration
Import-Module Az

# Docker completion
Import-Module DockerCompletion
```

## Profile Loading Order

PowerShell loads profiles in this order:

1. **All Users, All Hosts**: `$PSHOME\Profile.ps1`
2. **All Users, Current Host**: `$PSHOME\Microsoft.PowerShell_profile.ps1`
3. **Current User, All Hosts**: `$HOME\Documents\PowerShell\Profile.ps1`
4. **Current User, Current Host**: `$HOME\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` ← This one

The setup tool creates the "Current User, Current Host" profile, which is the most commonly used.

## Debugging Profile Issues

### Test Profile Loading

```powershell
. $PROFILE
```

Shows any errors in your profile.

### Check Execution Policy

```powershell
Get-ExecutionPolicy
```

Should be `RemoteSigned` or `Unrestricted`.

### View Profile Content

```powershell
Get-Content $PROFILE
```

### Edit Profile

```powershell
code $PROFILE      # VS Code
notepad $PROFILE   # Notepad
```

### Reload Profile

```powershell
. $PROFILE
```

## Performance Considerations

### Slow Profile Loading

If your profile loads slowly:

1. **Remove unused modules:**

   ```powershell
   # Comment out modules you don't use
   # Import-Module SomeModule
   ```

2. **Lazy load modules:**

   ```powershell
   # Load module only when needed
   function Use-Az {
       Import-Module Az
   }
   ```

3. **Profile startup time:**
   ```powershell
   Measure-Command { . $PROFILE }
   ```

### Optimize Oh-My-Posh

Some themes are faster than others. Minimal themes load faster:

```powershell
# Fast themes
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\jandedobbeleer.omp.json" | Invoke-Expression
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\atomic.omp.json" | Invoke-Expression
```

## Backup and Restore

### Backup Profile

```powershell
Copy-Item $PROFILE "$HOME\PowerShell_Profile_Backup.ps1"
```

### Restore Profile

```powershell
Copy-Item "$HOME\PowerShell_Profile_Backup.ps1" $PROFILE
```

### Version Control

Consider storing your profile in Git:

```powershell
# Create symlink to Git repo
New-Item -ItemType SymbolicLink -Path $PROFILE -Target "C:\Git\dotfiles\profile.ps1"
```

## Additional Resources

- [PowerShell Documentation](https://docs.microsoft.com/en-us/powershell/)
- [PSReadLine Documentation](https://github.com/PowerShell/PSReadLine)
- [Oh-My-Posh Documentation](https://ohmyposh.dev/)
- [posh-git Documentation](https://github.com/dahlbyk/posh-git)

## See Also

- [CUSTOMIZATION.md](../CUSTOMIZATION.md) - Customization guide
- [USAGE.md](../USAGE.md) - Usage instructions
- [TROUBLESHOOTING.md](../TROUBLESHOOTING.md) - Common issues
