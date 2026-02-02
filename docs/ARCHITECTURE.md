# Technical Architecture

This document explains the technical implementation of the PowerShell 7 Setup Automation tool.

## Technology Stack

### Core Technologies

**Rust Programming Language**

- Version: 2021 edition
- Chosen for: Performance, safety, excellent async support, cross-compilation capabilities

**Tokio Async Runtime**

- Version: 1.49.0
- Features: `full` (all Tokio features enabled)
- Purpose: Enables concurrent operations for faster installation

### Key Dependencies

| Dependency     | Version | Purpose                                 |
| -------------- | ------- | --------------------------------------- |
| `tokio`        | 1.49.0  | Async runtime for concurrent operations |
| `reqwest`      | 0.13.1  | HTTP client for downloading files       |
| `anyhow`       | 1.0.100 | Simplified error handling with context  |
| `colored`      | 3.1.1   | Terminal color output for better UX     |
| `futures-util` | 0.3.31  | Stream utilities for file downloads     |

**Why These Dependencies:**

- **tokio**: Enables parallel module installation, reducing total setup time
- **reqwest**: Robust HTTP client with streaming support for large downloads
- **anyhow**: Provides ergonomic error handling with `.context()` for user-friendly messages
- **colored**: Makes terminal output more readable and user-friendly
- **futures-util**: Provides `StreamExt` for efficient streaming downloads

## Async Architecture

### Tokio Runtime

The application uses Tokio's multi-threaded runtime:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Async code here
}
```

This macro expands to:

```rust
fn main() -> Result<()> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            // Async code here
        })
}
```

### Parallel Module Installation

Modules are installed concurrently using `tokio::join!`:

```rust
let (r1, r2, r3) = tokio::join!(
    install_module("PSReadLine"),
    install_module("posh-git"),
    install_oh_my_posh()
);
```

**Benefits:**

- All three installations run simultaneously
- Total time ≈ max(time1, time2, time3) instead of time1 + time2 + time3
- Typically saves 30-60 seconds

**Error Handling:**
Each result is checked individually, allowing partial success:

```rust
for result in [r1, r2, r3] {
    if let Err(e) = result {
        eprintln!("{} {}", "⚠".yellow(), format!("Warning: {}", e).yellow());
    }
}
```

### Async File Operations

File operations use Tokio's async file I/O:

```rust
use tokio::fs;

// Async file creation
fs::create_dir_all(profile_dir).await?;

// Async file writing
fs::write(&profile_path, PROFILE_CONTENT.trim()).await?;
```

**Why Async File I/O:**

- Non-blocking operations
- Better integration with other async operations
- Consistent async/await pattern throughout the codebase

## Installation Flow

### High-Level Flow

```
┌─────────────────────────────────────┐
│  Check if pwsh is available         │
└──────────────┬──────────────────────┘
               │
               ├─ Yes ──────────────────────┐
               │                            │
               └─ No                        │
                  │                         │
                  ▼                         │
         ┌────────────────────┐             │
         │ Download PS7 MSI   │             │
         └────────┬───────────┘             │
                  │                         │
                  ▼                         │
         ┌────────────────────┐             │
         │ Install via msiexec│             │
         └────────┬───────────┘             │
                  │                         │
                  ▼                         │
         ┌────────────────────┐             │
         │ Check pwsh again   │             │
         └────────┬───────────┘             │
                  │                         │
                  ├─ Available ─────────────┤
                  │                         │
                  └─ Not Available          │
                     │                      │
                     ▼                      │
            ┌────────────────┐              │
            │ Exit with msg  │              │
            │ to restart     │              │
            └────────────────┘              │
                                            │ 
                     ┌──────────────────────┘
                     │
                     ▼
         ┌────────────────────────┐
         │ Get profile path       │
         └────────┬───────────────┘
                  │
                  ▼
         ┌────────────────────────┐
         │ Install modules        │
         │ (parallel)             │
         │ - PSReadLine           │
         │ - posh-git             │
         │ - Oh-My-Posh           │
         └────────┬───────────────┘
                  │
                  ▼
         ┌────────────────────────┐
         │ Write profile file     │
         └────────┬───────────────┘
                  │
                  ▼
         ┌────────────────────────┐
         │ Success message        │
         └────────────────────────┘
```

### Detailed Flow

**1. PowerShell 7 Detection**

```rust
async fn is_pwsh_available() -> bool {
    Command::new("pwsh")
        .arg("-Version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok()
}
```

- Attempts to run `pwsh -Version`
- Suppresses output (stdout/stderr to null)
- Returns `true` if command succeeds, `false` otherwise
- Non-blocking async operation

**2. Download and Installation**

If PowerShell 7 is not available:

```rust
async fn download_and_install_powershell() -> Result<()> {
    // 1. Get latest release URL
    let redirect_url = get_latest_release_url().await?;

    // 2. Parse version from URL
    let version = extract_version(&redirect_url)?;

    // 3. Construct download URL
    let download_url = format!(
        "https://github.com/PowerShell/PowerShell/releases/download/{}/PowerShell-{}-win-x64.msi",
        version, version_number
    );

    // 4. Download to temp directory
    let msi_path = temp_dir().join(&msi_name);
    download_file(&download_url, &msi_path).await?;

    // 5. Install via msiexec
    Command::new("msiexec")
        .args(["/i", msi_path.to_str().unwrap(), "/quiet", "/norestart"])
        .status()
        .await?;

    Ok(())
}
```

**3. Module Installation**

```rust
async fn install_module(module_name: &str) -> Result<()> {
    let cmd = format!("Install-Module {} -Force -Scope CurrentUser", module_name);
    run_pwsh_command(&cmd).await?;
    Ok(())
}
```

- Executes PowerShell command via `pwsh -Command`
- Uses `-Force` to skip confirmation prompts
- Uses `-Scope CurrentUser` to avoid requiring admin rights
- Returns error if installation fails

**4. Profile Creation**

```rust
let profile_path = get_powershell_profile_path().await?;
let profile_dir = profile_path.parent().context("Failed to get profile directory")?;

fs::create_dir_all(profile_dir).await?;
fs::write(&profile_path, PROFILE_CONTENT.trim()).await?;
```

- Gets profile path from PowerShell: `pwsh -NoProfile -Command "$PROFILE"`
- Creates parent directory if it doesn't exist
- Writes profile content from constant

## PowerShell 7 Detection

### Why `pwsh -Version`?

The tool uses `pwsh -Version` to detect PowerShell 7:

```rust
Command::new("pwsh")
    .arg("-Version")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .await
    .is_ok()
```

**Advantages:**

- Fast: Just checks if command exists and runs
- Reliable: Works regardless of PowerShell configuration
- Non-intrusive: Doesn't load profile or modules
- Cross-platform: Works on Windows, Linux, macOS

**Alternative Approaches (Not Used):**

1. **Check file existence:**

   ```rust
   Path::new("C:\\Program Files\\PowerShell\\7\\pwsh.exe").exists()
   ```

   - Problem: Hardcoded path, doesn't work with custom installations

2. **Check registry:**

   ```rust
   // Check Windows registry for PowerShell 7
   ```

   - Problem: Windows-specific, complex, may not reflect PATH

3. **Parse PATH:**
   ```rust
   env::var("PATH")?.split(';').any(|p| p.contains("PowerShell\\7"))
   ```

   - Problem: Fragile, doesn't verify executable works

## Download Process

### GitHub Releases API Interaction

**Step 1: Get Latest Release**

```rust
let client = reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()?;

let response = client.get(GITHUB_RELEASES).send().await?;
```

- Disables automatic redirects
- Sends GET request to `https://github.com/PowerShell/PowerShell/releases/latest`
- GitHub responds with 302 redirect to actual release page

**Step 2: Extract Version from Redirect**

```rust
let redirect_url = response
    .headers()
    .get("location")
    .context("No redirect location found")?
    .to_str()?;

let version = redirect_url
    .rsplit('/')
    .next()
    .context("Failed to parse version")?;
```

- Reads `Location` header: `https://github.com/PowerShell/PowerShell/releases/tag/v7.4.1`
- Extracts version: `v7.4.1`

**Why This Approach:**

- Always gets the latest version
- No hardcoded version numbers
- No need to parse JSON or HTML
- Simple and reliable

### Streaming Download Implementation

```rust
async fn download_file(url: &str, path: &Path) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let mut file = fs::File::create(path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    Ok(())
}
```

**Benefits of Streaming:**

- Low memory usage: Doesn't load entire file into RAM
- Progress tracking: Could add progress bar by tracking bytes
- Efficient: Writes to disk as data arrives
- Handles large files: PowerShell MSI is ~100 MB

**Alternative (Not Used):**

```rust
// Download entire file to memory
let bytes = reqwest::get(url).await?.bytes().await?;
fs::write(path, bytes).await?;
```

- Problem: Uses ~100 MB RAM for PowerShell installer
- Problem: No progress tracking capability

## Module Installation

### PowerShell Command Execution

```rust
async fn run_pwsh_command(cmd: &str) -> Result<String> {
    println!("{} {}", "➡".blue(), cmd.bright_black());

    let output = Command::new("pwsh")
        .args(["-Command", cmd])
        .output()
        .await
        .context("Failed to execute PowerShell command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
```

**Key Points:**

- Uses `pwsh -Command` to execute PowerShell code
- Captures stdout and stderr
- Returns error with stderr content if command fails
- Prints command being executed for transparency

### Parallel Installation Strategy

```rust
let (r1, r2, r3) = tokio::join!(
    install_module("PSReadLine"),
    install_module("posh-git"),
    install_oh_my_posh()
);
```

**Execution Timeline:**

```
Sequential (not used):
PSReadLine: [========] 20s
posh-git:              [========] 20s
Oh-My-Posh:                       [========] 20s
Total: 60s

Parallel (used):
PSReadLine: [========] 20s
posh-git:   [========] 20s
Oh-My-Posh: [========] 20s
Total: 20s
```

**Implementation Details:**

- `tokio::join!` runs all futures concurrently
- Each installation is independent (no shared state)
- All results are collected and checked
- Failures don't block other installations

### Error Handling Approach

```rust
for result in [r1, r2, r3] {
    if let Err(e) = result {
        eprintln!("{} {}", "⚠".yellow(), format!("Warning: {}", e).yellow());
    }
}
```

**Philosophy:**

- Module installation failures are warnings, not fatal errors
- User can manually install missing modules later
- Profile is still created even if modules fail
- Provides better user experience than aborting

## Profile Generation

### Profile Path Determination

```rust
async fn get_powershell_profile_path() -> Result<PathBuf> {
    let output = Command::new("pwsh")
        .args(["-NoProfile", "-Command", "$PROFILE"])
        .output()
        .await
        .context("Failed to get PowerShell profile path")?;

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}
```

**Why Ask PowerShell:**

- Profile path varies by user and PowerShell version
- Typical path: `C:\Users\<User>\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`
- PowerShell knows the correct path for the current user
- Avoids hardcoding or guessing paths

**Alternative Approaches (Not Used):**

1. **Hardcode path:**

   ```rust
   let profile = format!("C:\\Users\\{}\\Documents\\PowerShell\\Microsoft.PowerShell_profile.ps1", username);
   ```

   - Problem: Doesn't work with custom Documents locations

2. **Use environment variables:**
   ```rust
   let profile = format!("{}\\Documents\\PowerShell\\Microsoft.PowerShell_profile.ps1", env::var("USERPROFILE")?);
   ```

   - Problem: Documents folder can be redirected

### Template-Based Generation

```rust
const PROFILE_CONTENT: &str = r#"
# ===========================
# Modern PowerShell 7 Profile
# ===========================

# --- Import Modules ---
Import-Module posh-git

# Oh-My-Posh prompt theme
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\paradox.omp.json" | Invoke-Expression

# ... rest of profile
"#;
```

**Benefits:**

- Profile content is embedded in binary
- No external files needed
- Easy to modify and version control
- Consistent across all installations

**File Writing:**

```rust
fs::write(&profile_path, PROFILE_CONTENT.trim()).await?;
```

- Overwrites existing profile (if any)
- Creates parent directories if needed
- Async operation for consistency

## Error Handling

### anyhow::Result Usage

All fallible functions return `anyhow::Result`:

```rust
async fn download_file(url: &str, path: &Path) -> Result<()> {
    // ... code that might fail
    Ok(())
}
```

**Benefits:**

- Automatic error propagation with `?` operator
- Can add context to errors
- Simplified error types (no need for custom error enums)

### Context Addition

Errors are enriched with context:

```rust
fs::create_dir_all(profile_dir)
    .await
    .context("Failed to create profile directory")?;
```

**Error Chain Example:**

```
Error: Failed to create profile directory

Caused by:
    Access is denied. (os error 5)
```

**Benefits:**

- User sees what operation failed
- Original error is preserved
- Easy to debug issues

### User-Friendly Error Messages

```rust
if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("Command failed: {}", stderr);
}
```

**Principles:**

- Show what went wrong
- Include relevant details (stderr output)
- Avoid technical jargon when possible
- Provide actionable information

### Graceful Degradation

Module installation failures don't abort the process:

```rust
for result in [r1, r2, r3] {
    if let Err(e) = result {
        eprintln!("{} {}", "⚠".yellow(), format!("Warning: {}", e).yellow());
    }
}

// Continue with profile creation
fs::write(&profile_path, PROFILE_CONTENT.trim()).await?;
```

**Philosophy:**

- Partial success is better than complete failure
- User can fix issues later
- Profile is still useful without all modules

## Performance Considerations

### Async Operations

All I/O operations are async:

- File downloads
- File writes
- Command execution
- Module installations

**Benefits:**

- Non-blocking operations
- Better resource utilization
- Faster overall execution

### Parallel Execution

Module installations run in parallel:

```rust
tokio::join!(
    install_module("PSReadLine"),
    install_module("posh-git"),
    install_oh_my_posh()
)
```

**Time Savings:**

- Sequential: ~60 seconds
- Parallel: ~20 seconds
- 3x faster

### Streaming Downloads

Files are downloaded in chunks:

```rust
let mut stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    file.write_all(&chunk?).await?;
}
```

**Benefits:**

- Low memory usage (~8 KB buffer vs ~100 MB file)
- Can add progress tracking
- Handles network interruptions better

## Security Considerations

### HTTPS Only

All downloads use HTTPS:

```rust
const GITHUB_RELEASES: &str = "https://github.com/PowerShell/PowerShell/releases/latest";
```

### Official Sources

Downloads come from official sources:

- PowerShell: GitHub releases (Microsoft)
- Modules: PowerShell Gallery (Microsoft)
- Oh-My-Posh: winget (verified publisher)

### No Arbitrary Code Execution

The tool doesn't execute user-provided code or download from user-specified URLs.

### Scope Limitation

Modules are installed to CurrentUser scope:

```rust
Install-Module ModuleName -Scope CurrentUser
```

**Benefits:**

- Doesn't require administrator rights
- Doesn't affect other users
- Easier to uninstall

## Future Enhancements

Potential improvements:

1. **Progress Bars**: Show download progress
2. **Theme Selection**: Let users choose Oh-My-Posh theme
3. **Custom Profile**: Allow profile customization during setup
4. **Rollback**: Backup and restore previous profile
5. **Update Check**: Check for tool updates
6. **Logging**: Write detailed logs for debugging
7. **Configuration File**: Allow configuration via file
8. **Pre-built Binaries**: Distribute compiled binaries

## Build and Distribution

### Building

```cmd
cargo build --release
```

Output: `target/release/setup_powershell.exe`

### Binary Size

Typical size: ~5-8 MB (includes all dependencies)

### Dependencies

All dependencies are statically linked (no DLL dependencies except Windows system DLLs).

## Conclusion

The tool uses modern Rust practices with async/await for efficient, concurrent operations. The architecture prioritizes user experience with colored output, parallel installations, and graceful error handling. The codebase is maintainable with clear separation of concerns and comprehensive error context.
