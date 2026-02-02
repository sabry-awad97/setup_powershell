# PowerShell 7 Setup Automation

A Rust CLI tool that automates the installation and configuration of PowerShell 7 on Windows with modern terminal enhancements.

## What It Does

This tool automatically sets up a modern PowerShell 7 environment with:

- **PowerShell 7** - Latest version installed from GitHub releases
- **Oh-My-Posh** - Beautiful prompt themes for your terminal
- **PSReadLine** - Enhanced command-line editing with history search and syntax highlighting
- **posh-git** - Git status integration in your prompt
- **Custom Profile** - Pre-configured with useful aliases and settings

## Features

- 🚀 **One-Command Setup** - Run once and get a fully configured PowerShell environment
- 📦 **Automatic Installation** - Downloads and installs PowerShell 7 if not present
- 🎨 **Beautiful Prompts** - Oh-My-Posh with the Paradox theme out of the box
- ⚡ **Git Shortcuts** - Quick aliases for common Git commands (gs, gc, gp, gl, gco, gb, gd)
- 🔍 **Smart History** - Search command history with arrow keys
- 🌈 **Syntax Highlighting** - Color-coded commands, parameters, and strings
- ⚙️ **Parallel Installation** - Installs modules concurrently for speed
- 💪 **Fully Customizable** - Easy to modify the generated profile to your preferences

## Why Use This?

Setting up a modern PowerShell environment manually involves:

- Finding and downloading the right PowerShell 7 installer
- Installing multiple PowerShell modules one by one
- Installing Oh-My-Posh via winget
- Writing a custom profile with proper configuration
- Configuring PSReadLine settings and key bindings
- Setting up Git aliases and custom shortcuts

This tool does all of that in one command, saving you 30+ minutes of setup time.

## Quick Start

### Prerequisites

- Windows 10 or Windows 11
- [Rust toolchain](https://rustup.rs/) installed
- [winget](https://learn.microsoft.com/en-us/windows/package-manager/winget/) (Windows Package Manager)
- Administrator rights (for PowerShell 7 installation)

### Installation

1. Clone this repository:

```cmd
git clone https://github.com/yourusername/setup_powershell.git
cd setup_powershell
```

2. Build and run:

```cmd
cargo run --release
```

3. If PowerShell 7 wasn't previously installed, restart your terminal and run again:

```cmd
cargo run --release
```

4. Launch PowerShell 7 to see your new environment:

```cmd
pwsh
```

## What Gets Installed

| Component      | Purpose                             |
| -------------- | ----------------------------------- |
| PowerShell 7   | Modern cross-platform PowerShell    |
| PSReadLine     | Enhanced command-line editing       |
| posh-git       | Git integration in prompt           |
| Oh-My-Posh     | Prompt theming engine               |
| Custom Profile | Pre-configured settings and aliases |

## Git Aliases

The tool configures these convenient Git shortcuts:

- `gs` → `git status`
- `gc -m "message"` → `git commit -m "message"`
- `gp` → `git push`
- `gl` → `git log --oneline --graph --decorate --all`
- `gco branch` → `git checkout branch`
- `gb` → `git branch`
- `gd` → `git diff`

## Custom Aliases

- `ll` → List files (Get-ChildItem)
- `la` → List all files including hidden (Get-ChildItem -Force)

## Documentation

- [Installation Guide](INSTALLATION.md) - Detailed setup instructions
- [Usage Guide](USAGE.md) - How to use the tool and configured features
- [Customization Guide](CUSTOMIZATION.md) - Customize your profile
- [Troubleshooting](TROUBLESHOOTING.md) - Common issues and solutions
- [Contributing](CONTRIBUTING.md) - How to contribute to this project
- [Architecture](docs/ARCHITECTURE.md) - Technical implementation details

## Screenshots

### Before

Standard PowerShell prompt with no enhancements.

### After

Oh-My-Posh themed prompt with Git integration, syntax highlighting, and enhanced editing.

_Screenshots coming soon_

## Platform Support

This tool is designed specifically for **Windows 10 and Windows 11**. PowerShell 7 itself is cross-platform, but this automation tool uses Windows-specific installation methods (msiexec, winget).

## License

MIT License - See LICENSE file for details

## Acknowledgments

- [PowerShell Team](https://github.com/PowerShell/PowerShell) - For PowerShell 7
- [Oh-My-Posh](https://ohmyposh.dev/) - For beautiful prompt themes
- [PSReadLine](https://github.com/PowerShell/PSReadLine) - For enhanced editing
- [posh-git](https://github.com/dahlbyk/posh-git) - For Git integration
