# Customization Guide

This guide explains how to customize your PowerShell 7 environment after running the setup tool.

## Profile Structure Overview

Your PowerShell profile is located at `$PROFILE` and contains several sections:

```powershell
# ===========================
# Modern PowerShell 7 Profile
# ===========================

# --- Import Modules ---
# Loads required PowerShell modules

# --- Oh-My-Posh Configuration ---
# Initializes prompt theming

# --- PSReadLine Settings ---
# Configures command-line editing behavior

# --- Aliases ---
# Simple command shortcuts

# --- Git Shortcuts (functions) ---
# Git command aliases with parameter support

# --- Environment ---
# Environment variable configuration
```

### Opening Your Profile

To edit your profile:

```powershell
code $PROFILE  # VS Code
notepad $PROFILE  # Notepad
```

### Reloading After Changes

After editing, reload your profile:

```powershell
. $PROFILE
```

## Changing Oh-My-Posh Themes

### Listing Available Themes

Oh-My-Posh comes with many built-in themes. To see them all:

```powershell
Get-PoshThemes
```

This displays all themes with previews.

### Previewing a Specific Theme

To preview a single theme:

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\agnoster.omp.json" | Invoke-Expression
```

Replace `agnoster` with any theme name.

### Changing Your Theme

1. Open your profile:

```powershell
code $PROFILE
```

2. Find this line:

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\paradox.omp.json" | Invoke-Expression
```

3. Replace `paradox.omp.json` with your preferred theme:

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\agnoster.omp.json" | Invoke-Expression
```

4. Save and reload:

```powershell
. $PROFILE
```

### Popular Themes

- **paradox** - Default, clean and informative
- **agnoster** - Classic, widely used
- **powerlevel10k_rainbow** - Colorful and feature-rich
- **jandedobbeleer** - Minimal and elegant
- **atomic** - Compact and modern
- **night-owl** - Dark theme optimized for readability

### Using a Custom Theme

You can create your own theme:

1. Export an existing theme as a starting point:

```powershell
oh-my-posh config export --output ~/my-theme.omp.json
```

2. Edit `my-theme.omp.json` to customize colors, segments, etc.

3. Update your profile to use it:

```powershell
oh-my-posh init pwsh --config "~/my-theme.omp.json" | Invoke-Expression
```

See [Oh-My-Posh documentation](https://ohmyposh.dev/docs/configuration/overview) for theme customization details.

## Customizing PSReadLine

### Changing Color Schemes

The default profile uses these colors:

```powershell
Set-PSReadLineOption -Colors @{
    "Command"   = 'Yellow'
    "Parameter" = 'Green'
    "String"    = 'Magenta'
    "Operator"  = 'DarkCyan'
    "Variable"  = 'White'
}
```

**Available colors:**
Black, DarkBlue, DarkGreen, DarkCyan, DarkRed, DarkMagenta, DarkYellow, Gray, DarkGray, Blue, Green, Cyan, Red, Magenta, Yellow, White

**Example - Dark theme:**

```powershell
Set-PSReadLineOption -Colors @{
    "Command"   = 'Cyan'
    "Parameter" = 'DarkCyan'
    "String"    = 'DarkGreen'
    "Operator"  = 'DarkGray'
    "Variable"  = 'Green'
}
```

### Changing Prediction Colors

The inline prediction color is set to Cyan:

```powershell
Set-PSReadLineOption -Colors @{ "InlinePrediction" = 'Cyan' }
```

Change it to any color:

```powershell
Set-PSReadLineOption -Colors @{ "InlinePrediction" = 'DarkGray' }
```

### Changing Prediction Behavior

**Prediction sources:**

```powershell
# History only (default)
Set-PSReadLineOption -PredictionSource History

# History + plugins
Set-PSReadLineOption -PredictionSource HistoryAndPlugin

# Disable predictions
Set-PSReadLineOption -PredictionSource None
```

**Prediction view style:**

```powershell
# Inline (default) - shows prediction as gray text
Set-PSReadLineOption -PredictionViewStyle InlineView

# List view - shows predictions in a dropdown list
Set-PSReadLineOption -PredictionViewStyle ListView
```

### Customizing Key Bindings

The default profile uses arrow keys for history search. You can add more bindings:

**Ctrl+D to delete character:**

```powershell
Set-PSReadLineKeyHandler -Key Ctrl+d -Function DeleteChar
```

**Ctrl+W to delete word:**

```powershell
Set-PSReadLineKeyHandler -Key Ctrl+w -Function BackwardKillWord
```

**Ctrl+Arrow for word navigation:**

```powershell
Set-PSReadLineKeyHandler -Key Ctrl+LeftArrow -Function BackwardWord
Set-PSReadLineKeyHandler -Key Ctrl+RightArrow -Function ForwardWord
```

**Ctrl+R for history search (interactive):**

```powershell
Set-PSReadLineKeyHandler -Key Ctrl+r -Function ReverseSearchHistory
```

See all available functions:

```powershell
Get-PSReadLineKeyHandler
```

### Changing Edit Mode

The default is Windows mode. You can switch to Emacs or Vi:

```powershell
# Emacs mode (Ctrl+A, Ctrl+E, etc.)
Set-PSReadLineOption -EditMode Emacs

# Vi mode (Esc for command mode)
Set-PSReadLineOption -EditMode Vi

# Windows mode (default)
Set-PSReadLineOption -EditMode Windows
```

### Bell Behavior

Disable the bell sound:

```powershell
Set-PSReadLineOption -BellStyle None
```

Options: `None`, `Visual`, `Audible`

## Adding Custom Aliases

### Simple Aliases

Add aliases to your profile:

```powershell
# Navigation shortcuts
Set-Alias .. cd..
Set-Alias ... cd..\..

# Common commands
Set-Alias g git
Set-Alias k kubectl
Set-Alias d docker
Set-Alias v vim

# Open current directory in Explorer
Set-Alias open explorer
```

### Aliases with Parameters (Functions)

For aliases that need parameters, use functions:

```powershell
# Quick directory navigation
function docs { Set-Location ~/Documents }
function proj { Set-Location ~/Projects }
function dl { Set-Location ~/Downloads }

# Create and enter directory
function mkcd($dir) {
    New-Item -ItemType Directory -Path $dir -Force
    Set-Location $dir
}

# Find files by name
function ff($name) {
    Get-ChildItem -Recurse -Filter "*$name*" -ErrorAction SilentlyContinue
}

# Find in files (grep equivalent)
function grep($pattern, $path = ".") {
    Select-String -Path "$path\*" -Pattern $pattern
}

# Quick edit profile
function ep { code $PROFILE }

# Reload profile
function rp { . $PROFILE }
```

### Advanced Function Examples

**Touch command (create empty file):**

```powershell
function touch($file) {
    if (Test-Path $file) {
        (Get-Item $file).LastWriteTime = Get-Date
    } else {
        New-Item -ItemType File -Path $file | Out-Null
    }
}
```

**Which command (find executable location):**

```powershell
function which($command) {
    Get-Command $command | Select-Object -ExpandProperty Source
}
```

**Kill process by name:**

```powershell
function killp($name) {
    Get-Process $name -ErrorAction SilentlyContinue | Stop-Process -Force
}
```

## Customizing Git Aliases

### Modifying Existing Aliases

The default Git aliases are functions. To modify them, edit your profile:

```powershell
# Change 'gs' to show short status
function gs { git status -s }

# Add 'gaa' for git add all
function gaa { git add --all }

# Add 'gcm' for commit with message
function gcm($msg) { git commit -m $msg }

# Add 'gpl' for git pull
function gpl { git pull @args }

# Add 'gf' for git fetch
function gf { git fetch @args }
```

### Advanced Git Functions

**Git commit with auto-generated message:**

```powershell
function gac($msg) {
    git add --all
    git commit -m $msg
}
```

**Git push with upstream:**

```powershell
function gpu {
    $branch = git branch --show-current
    git push -u origin $branch
}
```

**Git undo last commit (keep changes):**

```powershell
function gundo { git reset --soft HEAD~1 }
```

**Git amend last commit:**

```powershell
function gamend { git commit --amend --no-edit }
```

## Module Management

### Installing Additional Modules

Install modules from PowerShell Gallery:

```powershell
Install-Module ModuleName -Scope CurrentUser
```

**Useful modules:**

```powershell
# Terminal icons for ls
Install-Module -Name Terminal-Icons -Scope CurrentUser

# Better directory navigation
Install-Module -Name z -Scope CurrentUser

# Azure CLI integration
Install-Module -Name Az -Scope CurrentUser

# Docker completion
Install-Module -Name DockerCompletion -Scope CurrentUser
```

### Importing Modules in Profile

Add to your profile to auto-load modules:

```powershell
Import-Module Terminal-Icons
Import-Module z
```

### Updating Modules

Update all installed modules:

```powershell
Update-Module
```

Update a specific module:

```powershell
Update-Module PSReadLine
```

### Removing Modules

Uninstall a module:

```powershell
Uninstall-Module ModuleName
```

## Environment Variables

### Adding to Your Profile

Set environment variables in your profile:

```powershell
# Add to PATH
$env:PATH += ";C:\MyTools"

# Set custom variables
$env:EDITOR = "code"
$env:VISUAL = "code"

# Set default encoding
$PSDefaultParameterValues['*:Encoding'] = 'utf8'
```

### Persistent Environment Variables

For system-wide persistence, use:

```powershell
[System.Environment]::SetEnvironmentVariable('VARIABLE_NAME', 'value', 'User')
```

## Common Customization Examples

### Minimal Profile

If you prefer a minimal setup:

```powershell
# Minimal profile - just the essentials
Import-Module posh-git
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\jandedobbeleer.omp.json" | Invoke-Expression

Set-PSReadLineOption -PredictionSource History
Set-PSReadLineOption -PredictionViewStyle InlineView
```

### Power User Profile

For maximum productivity:

```powershell
# Import modules
Import-Module posh-git
Import-Module Terminal-Icons
Import-Module z

# Oh-My-Posh
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\powerlevel10k_rainbow.omp.json" | Invoke-Expression

# PSReadLine
Set-PSReadLineOption -PredictionSource HistoryAndPlugin
Set-PSReadLineOption -PredictionViewStyle ListView
Set-PSReadLineKeyHandler -Key Ctrl+d -Function DeleteChar
Set-PSReadLineKeyHandler -Key Ctrl+w -Function BackwardKillWord
Set-PSReadLineKeyHandler -Key Tab -Function MenuComplete

# Aliases
Set-Alias g git
Set-Alias k kubectl
Set-Alias d docker
function .. { Set-Location .. }
function ... { Set-Location ..\.. }
function ll { Get-ChildItem | Format-Table -AutoSize }
function la { Get-ChildItem -Force | Format-Table -AutoSize }

# Git shortcuts
function gs { git status -s }
function ga { git add @args }
function gc { git commit @args }
function gp { git push @args }
function gl { git log --oneline --graph --decorate --all -20 }
```

### Developer Profile

Optimized for software development:

```powershell
# Modules
Import-Module posh-git

# Theme
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\atomic.omp.json" | Invoke-Expression

# PSReadLine
Set-PSReadLineOption -PredictionSource History
Set-PSReadLineOption -EditMode Emacs

# Development aliases
function dev { Set-Location ~/Projects }
function build { dotnet build }
function test { dotnet test }
function run { dotnet run }

# Git workflow
function gs { git status }
function gd { git diff }
function gdc { git diff --cached }
function gco { git checkout @args }
function gcb($branch) { git checkout -b $branch }
function gp { git push }
function gpl { git pull }
function gf { git fetch }
function gm { git merge @args }
function gr { git rebase @args }

# Docker shortcuts
function dps { docker ps }
function di { docker images }
function dc { docker-compose @args }
function dcu { docker-compose up -d }
function dcd { docker-compose down }
```

## Troubleshooting Customizations

### Profile Errors on Startup

If your profile has errors, PowerShell will show them on startup. To debug:

```powershell
# Test profile manually
. $PROFILE

# Check for syntax errors
Test-Path $PROFILE
Get-Content $PROFILE | Out-Null
```

### Alias Conflicts

If an alias conflicts with an existing command:

```powershell
# Remove existing alias
Remove-Alias aliasname -Force

# Then set your custom alias
Set-Alias aliasname command
```

### Module Not Loading

If a module doesn't load:

```powershell
# Check if installed
Get-Module -ListAvailable ModuleName

# Try importing manually
Import-Module ModuleName -Verbose
```

## Next Steps

- See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
- Check [docs/PROFILE_REFERENCE.md](docs/PROFILE_REFERENCE.md) for complete profile documentation
- Visit [Oh-My-Posh docs](https://ohmyposh.dev/) for advanced theming
- Visit [PSReadLine docs](https://github.com/PowerShell/PSReadLine) for more editing options
