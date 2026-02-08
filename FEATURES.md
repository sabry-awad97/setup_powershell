# PowerShell Setup Tool - Features

## 🎯 Overview

A professional PowerShell 7 setup tool with interactive configuration, theme selection, and plugin management.

## ✨ Key Features

### 1. **Interactive Profile Selection**

Choose from pre-configured profiles or create your own:

- **Minimal** - Basic setup with essential features only
  - Theme: Pure (minimal)
  - Plugins: PSReadLine, posh-git
  - No custom aliases

- **Developer** - Full-featured setup for developers
  - Theme: Paradox
  - Plugins: PSReadLine, posh-git, Terminal-Icons, PSFzf, z
  - Git shortcuts and custom aliases
  - **Example use case**: Software developers who work with git, need quick navigation, and want a beautiful terminal

- **Work** - Professional setup with productivity tools
  - Theme: JanDeDobbeleer
  - Plugins: PSReadLine, posh-git, Terminal-Icons, PSFzf
  - Custom aliases enabled

- **Custom** - Choose your own theme and plugins
  - Select from 20+ themes
  - Pick individual plugins
  - Full customization

### 2. **Theme Selector** 🎨

Interactive menu with 20+ Oh-My-Posh themes:

- paradox, agnoster, atomic, blue-owl, bubbles
- capr4n, clean-detailed, craver, dracula, gruvbox
- jandedobbeleer, material, montys, night-owl
- powerlevel10k_rainbow, pure, robbyrussell
- sonicboom_dark, star, tokyo

Each theme includes a description to help you choose.

**Theme Preview Examples:**

```
# Paradox Theme (Developer default)
 drsab   setup_powershell   main ≡  ~1                    in pwsh at 14:23:45

# Pure Theme (Minimal)
❯

# Dracula Theme
🦇 drsab@DESKTOP ~/Projects/my-app main*

# Tokyo Night Theme
🌃 ~/Projects ⎇ main [+2 ~1 -0]
```

### 3. **Plugin System** 🔌

Select from curated PowerShell modules:

**Core Plugins:**

- **PSReadLine** - Enhanced command line editing with IntelliSense
- **posh-git** - Git status integration in prompt

**Optional Plugins:**

- **Terminal-Icons** - Beautiful file and folder icons in directory listings

  ```powershell
  # Before: Plain text listing
  PS> ls
  Mode    LastWriteTime    Length Name
  ----    -------------    ------ ----
  d----   1/1/2024         -      src
  -a---   1/1/2024         1234   README.md

  # After: With icons and colors
  PS> ls
  📁 src/
  📄 README.md
  🐍 script.py
  ⚛️ App.jsx
  🎨 styles.css
  ```

- **PSFzf** - Fuzzy finder integration for quick file/command search

  ```powershell
  # Press Ctrl+R to search command history
  # Type: git
  # Instantly shows all git commands you've run

  # Press Ctrl+T to search files
  # Type: config
  # Shows all files with "config" in the name

  # Example: Find and edit a file quickly
  PS> code (fzf)  # Opens fuzzy finder, select file, opens in VS Code
  ```

- **z** - Smart directory jumping based on frequency

  ```powershell
  # Traditional way (tedious)
  PS> cd C:\Users\drsab\Documents\Projects\my-awesome-app\src\components

  # With z (instant)
  PS> z components  # Jumps directly there
  PS> z awesome     # Also works with partial names
  PS> z app src     # Multiple keywords

  # It learns your habits
  PS> z proj        # Goes to your most-visited project folder
  ```

### 4. **Smart Installation**

- Checks if components are already installed
- Skips reinstallation of existing packages
- Shows clear status indicators (✓ already installed)
- Parallel installation for speed

### 5. **Automatic Font Configuration**

- Installs MesloLGM Nerd Font via Oh-My-Posh
- Automatically updates Windows Terminal settings
- Fixes icon display issues (no more question marks!)

### 6. **PowerShell 7 Auto-Install**

- Detects if PowerShell 7 is missing
- Downloads and installs latest version
- Falls back to Windows PowerShell if needed

## 🚀 Usage

```bash
cargo run --release
```

Follow the interactive prompts to:

1. Install PowerShell 7 (if needed)
2. Choose a profile preset
3. Select theme (if Custom profile)
4. Select plugins (if Custom profile)
5. Wait for installation
6. Restart your terminal

### Example Session

```
🚀 PowerShell Setup Tool
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 Choose a profile preset:
? Select profile ›
  Minimal - Basic setup with essential features only
❯ Developer - Full-featured setup for developers
  Work - Professional setup with productivity tools
  Custom - Choose your own theme and plugins

📦 Installing core components...
✓ oh-my-posh already installed
➡ oh-my-posh font install meslo
✅ Windows Terminal font updated!

🔌 Installing selected plugins...
✓ PSReadLine already installed
✓ posh-git already installed
➡ Install-Module Terminal-Icons -Force -Scope CurrentUser
➡ winget install fzf -s winget
✅ fzf installed successfully!
➡ Install-Module PSFzf -Force -Scope CurrentUser
➡ Install-Module z -Force -Scope CurrentUser

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Setup completed successfully!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 Profile: Developer
🎨 Theme: paradox
🔌 Plugins: PSReadLine, posh-git, Terminal-Icons, PSFzf, z

📄 Profile written to: C:\Users\drsab\Documents\PowerShell\Microsoft.PowerShell_profile.ps1

🔄 Restart PowerShell 7 (pwsh) to see the changes.
```

## 📦 What Gets Installed

### Core Components

- PowerShell 7 (if not present)
- Oh-My-Posh (prompt engine)
- MesloLGM Nerd Font (for icons)

### Profile-Specific

- Selected PowerShell modules
- Custom profile configuration
- Git shortcuts (if enabled)
- Custom aliases (if enabled)

## 🎨 Profile Configuration

The tool generates a PowerShell profile with:

- Module imports
- Oh-My-Posh theme initialization
- PSReadLine settings (history search, syntax colors)
- Custom aliases and git shortcuts (optional)
- Environment variables

### Example Generated Profile

```powershell
# ===========================
# Modern PowerShell 7 Profile
# ===========================

# --- Import Modules ---
Import-Module PSReadLine
Import-Module posh-git
if (Get-Module -ListAvailable -Name Terminal-Icons) { Import-Module Terminal-Icons }
if (Get-Module -ListAvailable -Name z) { Import-Module z }

# Oh-My-Posh prompt theme
if (Get-Command oh-my-posh -ErrorAction SilentlyContinue) {
    $configPath = "$env:POSH_THEMES_PATH\paradox.omp.json"
    if (Test-Path $configPath) {
        oh-my-posh init pwsh --config $configPath | Invoke-Expression
    } else {
        oh-my-posh init pwsh | Invoke-Expression
    }
}

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
function la { Get-ChildItem -Force }

# --- Git Shortcuts ---
function gs { git status }
function gcom { git commit @args }
function gpush { git push @args }
function gl { git log --oneline --graph --decorate --all }
function gco { git checkout @args }
function gb { git branch @args }
function gd { git diff @args }

# --- Environment ---
$env:POSH_GIT_ENABLED = $true
```

### Using the Git Shortcuts

```powershell
# Instead of typing full commands
PS> git status
PS> git commit -m "Add feature"
PS> git push origin main

# Use shortcuts
PS> gs                    # git status
PS> gcom -m "Add feature" # git commit
PS> gpush origin main     # git push
PS> gl                    # beautiful git log
PS> gco feature-branch    # git checkout
PS> gb                    # list branches
PS> gd                    # git diff
```

## 🔧 Technical Features

- **Async/parallel operations** - Fast installation
- **Error handling** - Graceful fallbacks
- **Idempotent** - Safe to run multiple times
- **Cross-version support** - Works with PowerShell 5.1 and 7+
- **Automatic Windows Terminal configuration** - JSON manipulation

## 📝 Generated Profile Location

- PowerShell 7: `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`
- Windows PowerShell: `~\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`

## 🎯 Future Enhancements

Potential additions:

- Backup/restore functionality
- Update command for modules
- Export/import configurations
- Dry-run mode
- Silent installation mode
- Health check command
