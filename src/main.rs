use anyhow::{Context, Result};
use colored::Colorize;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

const GITHUB_RELEASES: &str = "https://github.com/PowerShell/PowerShell/releases/latest";

const PROFILE_CONTENT: &str = r#"
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
"#;

#[tokio::main]
async fn main() -> Result<()> {
    if !is_pwsh_available().await {
        println!(
            "{}",
            "❌ pwsh (PowerShell 7) not found. Installing...".red()
        );
        download_and_install_powershell().await?;

        if !is_pwsh_available().await {
            println!("\n{}", "━".repeat(60).bright_black());
            println!(
                "{} {}",
                "✅".green(),
                "PowerShell 7 installation completed!".green().bold()
            );
            println!("{}", "━".repeat(60).bright_black());
            println!(
                "\n{} {}",
                "ℹ".blue(),
                "PowerShell 7 has been installed but is not yet available in the current session."
                    .bright_white()
            );
            println!("\n{}", "To complete the setup:".yellow().bold());
            println!("  {} Open a new terminal window", "1.".cyan());
            println!(
                "  {} Run this program again: {}",
                "2.".cyan(),
                "cargo run --release".bright_black()
            );
            println!(
                "\n{} {}\n",
                "💡".yellow(),
                "Alternatively, restart your PC for system-wide PATH updates.".bright_black()
            );
            return Ok(());
        }
    }

    let profile_path = get_powershell_profile_path().await?;
    let profile_dir = profile_path
        .parent()
        .context("Failed to get profile directory")?;

    fs::create_dir_all(profile_dir)
        .await
        .context("Failed to create profile directory")?;

    // Install required modules in parallel
    println!(
        "\n{} {}",
        "📦".cyan(),
        "Installing PowerShell modules...".cyan()
    );
    let (r1, r2, r3) = tokio::join!(
        install_module("PSReadLine"),
        install_module("posh-git"),
        install_oh_my_posh()
    );

    for result in [r1, r2, r3] {
        if let Err(e) = result {
            eprintln!("{} {}", "⚠".yellow(), format!("Warning: {}", e).yellow());
        }
    }

    // Write profile
    fs::write(&profile_path, PROFILE_CONTENT.trim())
        .await
        .context("Failed to write profile")?;

    println!("\n{}", "━".repeat(60).bright_black());
    println!(
        "{} {}",
        "✅".green(),
        "Setup completed successfully!".green().bold()
    );
    println!("{}", "━".repeat(60).bright_black());
    println!(
        "\n{} {}",
        "📝".blue(),
        format!("Profile written to: {}", profile_path.display()).bright_white()
    );
    println!(
        "\n{} {}\n",
        "🔄".cyan(),
        "Restart PowerShell 7 (pwsh) to see the changes.".cyan()
    );

    Ok(())
}

async fn is_pwsh_available() -> bool {
    Command::new("pwsh")
        .arg("-Version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok()
}

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

async fn get_powershell_profile_path() -> Result<PathBuf> {
    let output = Command::new("pwsh")
        .args(["-NoProfile", "-Command", "$PROFILE"])
        .output()
        .await
        .context("Failed to get PowerShell profile path")?;

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}

async fn download_and_install_powershell() -> Result<()> {
    println!(
        "{} {}",
        "🔍".cyan(),
        "Checking latest PowerShell release...".cyan()
    );

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let response = client.get(GITHUB_RELEASES).send().await?;
    let redirect_url = response
        .headers()
        .get("location")
        .context("No redirect location found")?
        .to_str()?;

    let version = redirect_url
        .rsplit('/')
        .next()
        .context("Failed to parse version")?;

    println!(
        "{} {}",
        "✅".green(),
        format!("Latest PowerShell: {}", version).green()
    );

    let version_number = version.strip_prefix('v').unwrap_or(version);
    let msi_name = format!("PowerShell-{}-win-x64.msi", version_number);
    let download_url = format!(
        "https://github.com/PowerShell/PowerShell/releases/download/{}/{}",
        version, msi_name
    );

    let temp_dir = std::env::temp_dir();
    let msi_path = temp_dir.join(&msi_name);

    println!(
        "{} {}",
        "⬇".blue(),
        format!("Downloading {} ...", msi_name).blue()
    );
    download_file(&download_url, &msi_path).await?;
    println!(
        "{} {}",
        "✅".green(),
        format!("Downloaded to {}", msi_path.display()).green()
    );

    println!(
        "{} {}",
        "⚙".yellow(),
        "Installing PowerShell 7 (this may need admin rights)...".yellow()
    );
    let status = Command::new("msiexec")
        .args(["/i", msi_path.to_str().unwrap(), "/quiet", "/norestart"])
        .status()
        .await?;

    if status.success() {
        println!(
            "{} {}",
            "✅".green(),
            "PowerShell 7 installed. Please restart the script if pwsh not found.".green()
        );
    } else {
        anyhow::bail!("Installation failed with status: {}", status);
    }

    Ok(())
}

async fn download_file(url: &str, path: &Path) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download: HTTP {}", response.status());
    }

    let mut file = fs::File::create(path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    Ok(())
}

async fn install_module(module_name: &str) -> Result<()> {
    let cmd = format!("Install-Module {} -Force -Scope CurrentUser", module_name);
    run_pwsh_command(&cmd).await?;
    Ok(())
}

async fn install_oh_my_posh() -> Result<()> {
    println!(
        "{} {}",
        "➡".blue(),
        "winget install JanDeDobbeleer.OhMyPosh -s winget".bright_black()
    );

    let status = Command::new("winget")
        .args(["install", "JanDeDobbeleer.OhMyPosh", "-s", "winget"])
        .status()
        .await
        .context("Failed to execute winget")?;

    if !status.success() {
        anyhow::bail!("winget install failed");
    }

    Ok(())
}

