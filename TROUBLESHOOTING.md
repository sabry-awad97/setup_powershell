# Troubleshooting Guide

This guide helps you resolve common issues with the PowerShell 7 Setup Automation tool and the configured environment.

## PowerShell 7 Not Found After Installation

### Symptoms

After running the setup tool and installing PowerShell 7, you see:

```
'pwsh' is not recognized as an internal or external command
```

### Root Cause

PowerShell 7 installation updates the system PATH environment variable, but your current terminal session has a cached copy of the old PATH. The `pwsh` command won't be available until you start a new session.

### Solutions

#### Solution 1: Restart Terminal (Recommended)

1. Close your current terminal window
2. Open a new Command Prompt or PowerShell window
3. Navigate back to the project directory
4. Run the tool again: `cargo run --release`

#### Solution 2: Restart Your PC

A full restart ensures all PATH changes are applied system-wide:

1. Restart your computer
2. Open a terminal
3. Verify: `pwsh --version`

#### Solution 3: Manual PATH Update (Advanced)

Update PATH in the current session:

```cmd
refreshenv
```

Or manually add PowerShell 7 to PATH:

```cmd
set PATH=%PATH%;C:\Program Files\PowerShell\7
```

### Verification

Check if PowerShell 7 is now available:

```cmd
pwsh --version
```

Expected output: `PowerShell 7.x.x`

---

## Module Installation Failures

### Symptoms

You see errors like:

```
WARNING: Unable to download from URI
Install-Module: Unable to install, repository not trusted
```

### Common Causes

1. **Network connectivity issues** - Can't reach PowerShell Gallery
2. **PSGallery not trusted** - Security policy blocks untrusted repositories
3. **Permission issues** - Insufficient rights to install modules

### Solutions

#### Solution 1: Trust PowerShell Gallery

```powershell
Set-PSRepository -Name PSGallery -InstallationPolicy Trusted
```

Then run the setup tool again.

#### Solution 2: Check Network Connection

Verify you can reach PowerShell Gallery:

```powershell
Test-NetConnection www.powershellgallery.com -Port 443
```

If this fails, check your firewall or proxy settings.

#### Solution 3: Manual Module Installation

Install modules manually:

```powershell
Install-Module PSReadLine -Force -Scope CurrentUser -SkipPublisherCheck
Install-Module posh-git -Force -Scope CurrentUser -SkipPublisherCheck
```

#### Solution 4: Use Different Scope

If CurrentUser scope fails, try AllUsers (requires admin):

```powershell
Install-Module PSReadLine -Force -Scope AllUsers
Install-Module posh-git -Force -Scope AllUsers
```

### Verification

Check if modules are installed:

```powershell
Get-Module -ListAvailable PSReadLine, posh-git
```

Both should be listed.

---

## Oh-My-Posh Not Working

### Symptoms

- Prompt is not themed (still shows `PS>`)
- Error: `oh-my-posh: command not found`
- Prompt shows garbled characters or boxes

### Cause 1: Oh-My-Posh Installation Failed

#### Solution: Manual Installation

Install Oh-My-Posh manually via winget:

```cmd
winget install JanDeDobbeleer.OhMyPosh -s winget
```

Or via PowerShell:

```powershell
winget install JanDeDobbeleer.OhMyPosh
```

#### Alternative: Install via Scoop

If winget doesn't work:

```powershell
scoop install https://github.com/JanDeDobbeleer/oh-my-posh/releases/latest/download/oh-my-posh.json
```

### Cause 2: PATH Not Updated

#### Solution: Restart Terminal

Close and reopen your terminal after Oh-My-Posh installation.

#### Solution: Manual PATH Update

Add Oh-My-Posh to PATH:

```powershell
$env:PATH += ";$env:LOCALAPPDATA\Programs\oh-my-posh\bin"
```

### Cause 3: Missing Nerd Fonts

Oh-My-Posh themes use special glyphs that require Nerd Fonts.

#### Solution: Install a Nerd Font

1. Download a Nerd Font from [nerdfonts.com](https://www.nerdfonts.com/)
   - Recommended: **CascadiaCode Nerd Font** or **FiraCode Nerd Font**

2. Install the font:
   - Extract the ZIP file
   - Right-click the `.ttf` files
   - Select "Install for all users"

3. Configure your terminal to use the font:
   - **Windows Terminal**: Settings → Profiles → Defaults → Appearance → Font face
   - **VS Code Terminal**: Settings → Terminal › Integrated: Font Family

4. Restart your terminal

### Verification

Check Oh-My-Posh version:

```powershell
oh-my-posh --version
```

Test theme rendering:

```powershell
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\paradox.omp.json" | Invoke-Expression
```

---

## Profile Not Loading

### Symptoms

- Customizations don't appear when starting PowerShell
- Aliases don't work
- Prompt is not themed

### Cause 1: Execution Policy

PowerShell's execution policy may block profile scripts.

#### Solution: Set Execution Policy

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

This allows local scripts to run while requiring remote scripts to be signed.

### Cause 2: Wrong Profile Location

You might be editing the wrong profile file.

#### Solution: Verify Profile Path

Check your profile location:

```powershell
$PROFILE
```

Ensure you're editing this exact file.

### Cause 3: Syntax Errors in Profile

Errors in your profile prevent it from loading.

#### Solution: Test Profile Manually

```powershell
. $PROFILE
```

This will show any errors. Fix them and try again.

#### Solution: Reset to Default Profile

If your profile is broken, restore the default:

1. Backup your current profile:

```powershell
Copy-Item $PROFILE "$PROFILE.backup"
```

2. Run the setup tool again to regenerate the profile:

```cmd
cargo run --release
```

### Cause 4: Profile Doesn't Exist

The profile file might have been deleted.

#### Solution: Recreate Profile

Run the setup tool again:

```cmd
cargo run --release
```

### Verification

Check if profile loads without errors:

```powershell
. $PROFILE
```

No output means success. Errors will be displayed if present.

---

## Administrator Rights Issues

### Symptoms

- "Access denied" errors during installation
- "You must be an administrator" messages
- Installation fails with permission errors

### Cause

PowerShell 7 installation and Oh-My-Posh installation require administrator privileges.

### Solutions

#### Solution 1: Run as Administrator

1. Right-click Command Prompt or PowerShell
2. Select "Run as administrator"
3. Navigate to the project directory
4. Run: `cargo run --release`

#### Solution 2: Use UAC Prompt

When the tool runs `msiexec` or `winget`, Windows will show a UAC prompt. Click "Yes" to allow.

#### Solution 3: Manual Installation Without Admin

If you can't get admin rights:

**PowerShell 7:**

- Download the ZIP version from [GitHub releases](https://github.com/PowerShell/PowerShell/releases)
- Extract to a user-writable location (e.g., `C:\Users\YourName\PowerShell7`)
- Add to PATH manually

**Oh-My-Posh:**

- Use Scoop (doesn't require admin):

```powershell
scoop install oh-my-posh
```

**Modules:**

- Already installed to CurrentUser scope (no admin needed)

### Limitations Without Admin Rights

- PowerShell 7 won't be in the system PATH for all users
- Oh-My-Posh may not be available system-wide
- Some features may not work as expected

---

## Git Aliases Not Working

### Symptoms

- `gs`, `gc`, `gp`, etc. show "command not found"
- Git commands work, but aliases don't

### Cause 1: Git Not Installed

The aliases require Git to be installed.

#### Solution: Install Git

Download and install Git from [git-scm.com](https://git-scm.com/download/win).

After installation, restart your terminal.

### Cause 2: Git Not in PATH

Git is installed but not in the PATH.

#### Solution: Add Git to PATH

1. Find Git installation directory (usually `C:\Program Files\Git\cmd`)
2. Add to PATH:

```powershell
$env:PATH += ";C:\Program Files\Git\cmd"
```

3. For permanent change:

```powershell
[System.Environment]::SetEnvironmentVariable('PATH', $env:PATH + ';C:\Program Files\Git\cmd', 'User')
```

### Cause 3: Profile Not Loaded

The aliases are defined in your profile, which may not be loading.

#### Solution: Check Profile Loading

See the "Profile Not Loading" section above.

### Verification

Test Git:

```powershell
git --version
```

Test alias:

```powershell
gs
```

Should run `git status`.

---

## Verification Commands

Use these commands to verify your setup:

### Check PowerShell Version

```powershell
$PSVersionTable.PSVersion
```

Expected: Version 7.x.x or higher

### Check Installed Modules

```powershell
Get-Module -ListAvailable PSReadLine, posh-git
```

Both modules should be listed.

### Check Oh-My-Posh

```powershell
oh-my-posh --version
```

Should display version number.

### Check Git

```powershell
git --version
```

Should display Git version.

### Test Profile Loading

```powershell
. $PROFILE
```

Should execute without errors.

### Check Execution Policy

```powershell
Get-ExecutionPolicy
```

Should be `RemoteSigned` or `Unrestricted`.

### Check PATH

```powershell
$env:PATH -split ';'
```

Should include:

- `C:\Program Files\PowerShell\7`
- `C:\Users\YourName\AppData\Local\Programs\oh-my-posh\bin`
- `C:\Program Files\Git\cmd`

---

## Common Error Messages

### "Cannot be loaded because running scripts is disabled"

**Solution:** Set execution policy:

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### "The term 'oh-my-posh' is not recognized"

**Solution:** Reinstall Oh-My-Posh or add to PATH. See "Oh-My-Posh Not Working" section.

### "Unable to find module repositories"

**Solution:** Register PowerShell Gallery:

```powershell
Register-PSRepository -Default
```

### "The specified module 'posh-git' was not loaded"

**Solution:** Install the module:

```powershell
Install-Module posh-git -Force -Scope CurrentUser
```

### "Access to the path is denied"

**Solution:** Run as administrator or use CurrentUser scope for modules.

---

## Still Having Issues?

If you're still experiencing problems:

1. **Check the logs**: Look for error messages in the tool's output
2. **Verify prerequisites**: Ensure Rust, winget, and Windows version meet requirements
3. **Try manual installation**: Follow the manual steps in [INSTALLATION.md](INSTALLATION.md)
4. **Reset your profile**: Backup and regenerate your profile
5. **Open an issue**: Report the problem on [GitHub Issues](https://github.com/yourusername/setup_powershell/issues)

When reporting issues, include:

- Windows version (`winver`)
- PowerShell version (`$PSVersionTable.PSVersion`)
- Error messages (full text)
- Steps to reproduce

---

## Uninstalling

If you want to remove the customizations:

### Remove Profile

```powershell
Remove-Item $PROFILE
```

### Uninstall Modules

```powershell
Uninstall-Module PSReadLine, posh-git
```

### Uninstall Oh-My-Posh

```powershell
winget uninstall JanDeDobbeleer.OhMyPosh
```

### Uninstall PowerShell 7

1. Open Windows Settings
2. Go to Apps → Installed apps
3. Find "PowerShell 7-x64"
4. Click "Uninstall"

Or via command line:

```cmd
msiexec /x {ProductCode} /quiet
```

### Backup Before Uninstalling

To backup your profile:

```powershell
Copy-Item $PROFILE "$HOME\PowerShell_Profile_Backup.ps1"
```

---

## Additional Resources

- [PowerShell Documentation](https://docs.microsoft.com/en-us/powershell/)
- [Oh-My-Posh Documentation](https://ohmyposh.dev/)
- [PSReadLine Documentation](https://github.com/PowerShell/PSReadLine)
- [posh-git Documentation](https://github.com/dahlbyk/posh-git)
- [Windows Terminal Documentation](https://docs.microsoft.com/en-us/windows/terminal/)
