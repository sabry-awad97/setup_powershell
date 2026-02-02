# Contributing to PowerShell 7 Setup Automation

Thank you for your interest in contributing! This guide will help you get started.

## Getting Started

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:

```cmd
git clone https://github.com/yourusername/setup_powershell.git
cd setup_powershell
```

3. Add upstream remote:

```cmd
git remote add upstream https://github.com/originalowner/setup_powershell.git
```

### Build from Source

Ensure you have Rust installed, then build:

```cmd
cargo build --release
```

The compiled binary will be in `target/release/setup_powershell.exe`.

### Run the Tool

Test your changes:

```cmd
cargo run --release
```

Or run the compiled binary directly:

```cmd
.\target\release\setup_powershell.exe
```

## Project Structure

```
setup_powershell/
├── src/
│   └── main.rs              # Main application code
├── Cargo.toml               # Rust dependencies and metadata
├── Cargo.lock               # Locked dependency versions
├── README.md                # Project overview
├── INSTALLATION.md          # Installation guide
├── USAGE.md                 # Usage instructions
├── CUSTOMIZATION.md         # Customization guide
├── TROUBLESHOOTING.md       # Troubleshooting guide
├── CONTRIBUTING.md          # This file
└── docs/
    ├── ARCHITECTURE.md      # Technical architecture
    └── PROFILE_REFERENCE.md # Profile configuration reference
```

### src/main.rs Overview

The main application file contains:

**Constants:**

- `GITHUB_RELEASES` - URL for PowerShell releases
- `PROFILE_CONTENT` - Template for the PowerShell profile

**Main Function:**

- `main()` - Entry point, orchestrates the setup process

**Core Functions:**

- `is_pwsh_available()` - Checks if PowerShell 7 is installed
- `download_and_install_powershell()` - Downloads and installs PowerShell 7
- `get_powershell_profile_path()` - Gets the profile file location
- `install_module()` - Installs a PowerShell module
- `install_oh_my_posh()` - Installs Oh-My-Posh via winget
- `run_pwsh_command()` - Executes PowerShell commands
- `download_file()` - Downloads files with streaming

**Key Dependencies:**

- `tokio` - Async runtime for concurrent operations
- `reqwest` - HTTP client for downloading files
- `anyhow` - Error handling with context
- `colored` - Terminal color output
- `futures-util` - Stream utilities for downloads

## Development Workflow

### 1. Create a Feature Branch

```cmd
git checkout -b feature/your-feature-name
```

Use descriptive branch names:

- `feature/add-theme-selection` - New features
- `fix/module-install-error` - Bug fixes
- `docs/update-readme` - Documentation updates
- `refactor/improve-error-handling` - Code refactoring

### 2. Make Your Changes

Edit the code, following the coding standards below.

### 3. Test Your Changes

**Manual Testing:**

Test on a clean system if possible, or:

1. Uninstall PowerShell 7 (if testing installation)
2. Remove your profile: `Remove-Item $PROFILE`
3. Run the tool: `cargo run --release`
4. Verify all features work

**Test Scenarios:**

- [ ] Fresh installation (no PowerShell 7)
- [ ] Existing PowerShell 7 installation
- [ ] Module installation
- [ ] Oh-My-Posh installation
- [ ] Profile creation
- [ ] Profile loading without errors
- [ ] Git aliases work
- [ ] Custom aliases work
- [ ] PSReadLine features work
- [ ] Oh-My-Posh theme displays correctly

### 4. Commit Your Changes

Use clear, descriptive commit messages:

```cmd
git add .
git commit -m "Add theme selection feature"
```

**Commit Message Format:**

```
<type>: <short summary>

<optional detailed description>

<optional footer>
```

**Types:**

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, no logic change)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

**Examples:**

```
feat: Add interactive theme selection

Allow users to choose Oh-My-Posh theme during setup
instead of defaulting to paradox.

Closes #42
```

```
fix: Handle network timeout during download

Add retry logic and better error messages when
PowerShell download fails due to network issues.

Fixes #38
```

### 5. Push and Create Pull Request

```cmd
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:

- Clear title describing the change
- Description of what changed and why
- Reference to related issues (if any)
- Screenshots (if UI/output changed)

## Coding Standards

### Rust Formatting

Use `rustfmt` to format code:

```cmd
cargo fmt
```

This ensures consistent code style across the project.

### Linting

Run Clippy to catch common mistakes:

```cmd
cargo clippy
```

Fix any warnings before submitting.

### Error Handling

Use `anyhow::Result` for functions that can fail:

```rust
async fn my_function() -> Result<()> {
    // ... code that might fail
    Ok(())
}
```

Add context to errors:

```rust
fs::write(&path, content)
    .await
    .context("Failed to write profile file")?;
```

Provide user-friendly error messages:

```rust
if !status.success() {
    anyhow::bail!("Installation failed. Please check your internet connection and try again.");
}
```

### Async/Await Patterns

Use async/await for I/O operations:

```rust
async fn download_file(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
```

Use `tokio::join!` for parallel operations:

```rust
let (r1, r2, r3) = tokio::join!(
    install_module("PSReadLine"),
    install_module("posh-git"),
    install_oh_my_posh()
);
```

### User Output

Use colored output for better UX:

```rust
println!("{} {}", "✅".green(), "Success message".green());
println!("{} {}", "❌".red(), "Error message".red());
println!("{} {}", "ℹ".blue(), "Info message".blue());
println!("{} {}", "⚠".yellow(), "Warning message".yellow());
```

Use emojis and formatting for clarity:

```rust
println!("\n{}", "━".repeat(60).bright_black());
println!("{} {}", "🔍".cyan(), "Checking for updates...".cyan());
```

### Documentation Comments

Add doc comments for public functions:

```rust
/// Downloads a file from the specified URL to the given path.
///
/// # Arguments
///
/// * `url` - The URL to download from
/// * `path` - The local path to save the file
///
/// # Errors
///
/// Returns an error if the download fails or the file cannot be written.
async fn download_file(url: &str, path: &Path) -> Result<()> {
    // implementation
}
```

### Constants

Use constants for configuration:

```rust
const GITHUB_RELEASES: &str = "https://github.com/PowerShell/PowerShell/releases/latest";
const DEFAULT_THEME: &str = "paradox";
```

## Testing

### Manual Testing Checklist

Before submitting a PR, test:

- [ ] **Fresh Installation**
  - Uninstall PowerShell 7
  - Run tool
  - Verify two-phase installation works
  - Verify all components install

- [ ] **Existing Installation**
  - With PowerShell 7 already installed
  - Run tool
  - Verify modules and profile are created

- [ ] **Profile Functionality**
  - Launch `pwsh`
  - Test Git aliases: `gs`, `gc`, `gp`, `gl`, `gco`, `gb`, `gd`
  - Test custom aliases: `ll`, `la`
  - Test history search with arrow keys
  - Verify syntax highlighting
  - Verify Oh-My-Posh theme displays

- [ ] **Error Handling**
  - Test with no internet connection
  - Test with insufficient permissions
  - Verify error messages are helpful

- [ ] **Edge Cases**
  - Test on Windows 10 and Windows 11
  - Test with existing profile (should overwrite)
  - Test with modules already installed

### Testing on Clean System

For thorough testing, use a VM or clean Windows installation:

1. Create a Windows VM (VirtualBox, Hyper-V, etc.)
2. Install Rust and winget
3. Clone the repository
4. Run the tool
5. Verify all functionality

## Submitting Issues

### Bug Reports

When reporting bugs, include:

**System Information:**

- Windows version: `winver`
- PowerShell version: `$PSVersionTable.PSVersion`
- Rust version: `rustc --version`

**Description:**

- What you expected to happen
- What actually happened
- Steps to reproduce

**Error Messages:**

- Full error output
- Screenshots if applicable

**Example:**

```markdown
## Bug Report

**Environment:**

- Windows 11 22H2
- PowerShell 7.4.1
- Rust 1.75.0

**Description:**
Module installation fails with "Unable to download" error.

**Steps to Reproduce:**

1. Run `cargo run --release`
2. Wait for PowerShell 7 installation
3. Module installation fails

**Error Message:**
```

WARNING: Unable to download from URI 'https://www.powershellgallery.com/...'

```

**Expected:**
Modules should install successfully.
```

### Feature Requests

When requesting features, include:

- **Use Case:** Why is this feature needed?
- **Proposed Solution:** How should it work?
- **Alternatives:** Other ways to achieve the same goal
- **Additional Context:** Any other relevant information

**Example:**

```markdown
## Feature Request

**Use Case:**
I want to choose a different Oh-My-Posh theme during setup instead of always getting paradox.

**Proposed Solution:**
Add an interactive prompt asking which theme to use, with a list of popular themes.

**Alternatives:**

- Command-line flag: `--theme agnoster`
- Config file with theme preference

**Additional Context:**
Many users prefer different themes, and changing it after setup requires editing the profile.
```

## Code Review Process

All contributions go through code review:

1. **Automated Checks:** CI runs `cargo fmt`, `cargo clippy`, and `cargo build`
2. **Manual Review:** Maintainers review code for quality and correctness
3. **Testing:** Changes are tested on different Windows versions
4. **Feedback:** Reviewers may request changes
5. **Approval:** Once approved, changes are merged

## Style Guide Summary

- Use `rustfmt` for formatting
- Run `clippy` and fix warnings
- Add context to errors with `.context()`
- Use colored output for user messages
- Write doc comments for public functions
- Test thoroughly before submitting
- Write clear commit messages
- Keep PRs focused on a single change

## Questions?

If you have questions:

- Check existing [Issues](https://github.com/yourusername/setup_powershell/issues)
- Read the [documentation](README.md)
- Open a new issue with the "question" label

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Thank You!

Your contributions make this project better for everyone. Thank you for taking the time to contribute!
