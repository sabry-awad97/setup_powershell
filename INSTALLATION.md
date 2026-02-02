# Installation Guide

This guide provides detailed instructions for installing and running the PowerShell 7 Setup Automation tool.

## Prerequisites

Before running this tool, ensure you have the following installed:

### 1. Windows Operating System

- **Windows 10** (version 1809 or later) or **Windows 11**
- 64-bit architecture (x64)

### 2. Rust Toolchain

The tool is written in Rust and must be compiled from source.

**Install Rust:**

1. Visit [https://rustup.rs/](https://rustup.rs/)
2. Download and run `rustup-init.exe`
3. Follow the installation prompts
4. Restart your terminal after installation

**Verify installation:**

```cmd
rustc --version
cargo --version
```

You should see version information for both commands.

### 3. Windows Package Manager (winget)

winget is used to install Oh-My-Posh. It comes pre-installed on Windows 11 and recent Windows 10 builds.

**Verify winget is available:**

```cmd
winget --version
```

**If winget is not installed:**

- Windows 11: It should be pre-installed
- Windows 10: Install [App Installer](https://www.microsoft.com/p/app-installer/9nblggh4nns1) from the Microsoft Store

### 4. Administrator Rights

Administrator privileges are required for:

- Installing PowerShell 7 (via msiexec)
- Installing Oh-My-Posh (via winget)

The tool will prompt for elevation when needed.

## System Requirements

- **Disk Space**: ~200 MB for PowerShell 7 and all components
- **RAM**: 512 MB minimum (for running the tool)
- **Network**: Internet connection required for downloading:
  - PowerShell 7 installer (~100 MB)
  - PowerShell modules from PSGallery
  - Oh-My-Posh via winget

## Installation Steps

### Step 1: Clone the Repository

```cmd
git clone https://github.com/yourusername/setup_powershell.git
cd setup_powershell
```

If you don't have Git installed, download the repository as a ZIP file and extract it.

### Step 2: Build the Tool

Build the release version for optimal performance:

```cmd
cargo build --release
```

This will compile the tool and place the executable in `target/release/setup_powershell.exe`.

The build process may take a few minutes on the first run as it downloads and compiles dependencies.

### Step 3: Run the Tool

Execute the tool:

```cmd
cargo run --release
```

**What happens during execution:**

1. **PowerShell 7 Check**: The tool checks if `pwsh` is available
2. **Download & Install** (if needed): Downloads the latest PowerShell 7 MSI and installs it
3. **Module Installation**: Installs PSReadLine and posh-git from PSGallery
4. **Oh-My-Posh Installation**: Installs Oh-My-Posh via winget
5. **Profile Creation**: Creates a custom PowerShell profile with configurations

### Step 4: Handle Two-Phase Installation (If Applicable)

If PowerShell 7 was not previously installed, you'll see this message:

```
✅ PowerShell 7 installation completed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ℹ PowerShell 7 has been installed but is not yet available in the current session.

To complete the setup:
  1. Open a new terminal window
  2. Run this program again: cargo run --release

💡 Alternatively, restart your PC for system-wide PATH updates.
```

**Why this happens:**

- PowerShell 7 installation updates the system PATH
- The current terminal session doesn't see the updated PATH
- A new terminal session will have access to `pwsh`

**What to do:**

1. Close your current terminal
2. Open a new terminal window
3. Navigate back to the project directory
4. Run `cargo run --release` again

The second run will detect PowerShell 7 and complete the module and profile setup.

## Post-Installation

### Verify Installation

After successful completion, verify everything is working:

**1. Check PowerShell 7 version:**

```cmd
pwsh --version
```

Expected output: `PowerShell 7.x.x`

**2. Launch PowerShell 7:**

```cmd
pwsh
```

You should see the Oh-My-Posh themed prompt.

**3. Check installed modules:**

```powershell
Get-Module -ListAvailable PSReadLine, posh-git
```

Both modules should be listed.

**4. Check Oh-My-Posh:**

```powershell
oh-my-posh --version
```

Should display the Oh-My-Posh version.

**5. Test Git aliases:**

```powershell
gs  # Should run 'git status'
```

### Profile Location

Your PowerShell profile is located at:

```
C:\Users\YourUsername\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
```

To find your exact profile path:

```powershell
$PROFILE
```

## Troubleshooting Installation Issues

### "pwsh is not recognized" after installation

**Solution:** Restart your terminal or PC to update the PATH environment variable.

### "cargo is not recognized"

**Solution:** Rust is not installed or not in PATH. Install Rust from [rustup.rs](https://rustup.rs/) and restart your terminal.

### "winget is not recognized"

**Solution:** Install App Installer from the Microsoft Store or update Windows.

### Module installation fails

**Solution:** PowerShell Gallery might not be trusted. Run:

```powershell
Set-PSRepository -Name PSGallery -InstallationPolicy Trusted
```

### Permission denied errors

**Solution:** Run your terminal as Administrator.

For more troubleshooting help, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).

## Alternative Installation Methods

### Using Pre-built Binary (Future)

Pre-built binaries may be provided in future releases. Check the [Releases](https://github.com/yourusername/setup_powershell/releases) page.

### Manual Installation Fallback

If the automated tool fails, you can manually:

1. Install PowerShell 7 from [GitHub releases](https://github.com/PowerShell/PowerShell/releases)
2. Install modules: `Install-Module PSReadLine, posh-git -Force`
3. Install Oh-My-Posh: `winget install JanDeDobbeleer.OhMyPosh`
4. Copy the profile content from `src/main.rs` (PROFILE_CONTENT constant) to your `$PROFILE`

## Next Steps

- Read the [Usage Guide](USAGE.md) to learn about all configured features
- Check out the [Customization Guide](CUSTOMIZATION.md) to personalize your setup
- See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) if you encounter issues
