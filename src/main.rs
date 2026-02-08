use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Confirm;
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

# --- Git Shortcuts (functions instead of aliases) ---
function gs { git status }
function gcom { git commit @args }   # usage: gcom -m "msg"
function gpush { git push @args }
function gl { git log --oneline --graph --decorate --all }
function gco { git checkout @args }
function gb { git branch @args }
function gd { git diff @args }

# --- Environment ---
$env:POSH_GIT_ENABLED = $true
"#;

#[tokio::main]
async fn main() -> Result<()> {
    let use_pwsh = if !is_pwsh_available().await {
        println!("{}", "❌ pwsh (PowerShell 7) not found.".red());

        let should_install = Confirm::new()
            .with_prompt("Would you like to download and install PowerShell 7?")
            .default(true)
            .interact()?;

        if !should_install {
            println!(
                "\n{} {}",
                "ℹ".blue(),
                "Skipping PowerShell 7 installation.".bright_white()
            );
            println!(
                "{} {}",
                "💡".yellow(),
                "You can install it manually from: https://github.com/PowerShell/PowerShell/releases"
                    .bright_black()
            );

            if is_powershell_available().await {
                println!(
                    "\n{} {}\n",
                    "🔄".cyan(),
                    "Continuing with Windows PowerShell (powershell.exe)...".cyan()
                );
                false // Use powershell.exe instead
            } else {
                println!(
                    "\n{} {}\n",
                    "❌".red(),
                    "No PowerShell version found. Please install PowerShell.".red()
                );
                return Ok(());
            }
        } else {
            println!("{}", "⬇ Installing PowerShell 7...".cyan());
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
            true
        }
    } else {
        true // pwsh is available
    };

    let profile_path = get_powershell_profile_path(use_pwsh).await?;
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
    let (r1, r2, r3, r4) = tokio::join!(
        install_module("PSReadLine", use_pwsh),
        install_module("posh-git", use_pwsh),
        install_oh_my_posh(),
        install_nerd_font()
    );

    for result in [r1, r2, r3, r4] {
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
    let shell_name = if use_pwsh {
        "PowerShell 7 (pwsh)"
    } else {
        "PowerShell"
    };
    println!(
        "\n{} {}\n",
        "🔄".cyan(),
        format!("Restart {} to see the changes.", shell_name).cyan()
    );

    // Try to update Windows Terminal font
    if let Err(e) = update_windows_terminal_font().await {
        println!(
            "\n{} {}",
            "💡".yellow(),
            format!("Could not auto-configure Windows Terminal font: {}", e).yellow()
        );
        println!(
            "{} {}",
            "ℹ".blue(),
            "Manually set font to 'MesloLGM Nerd Font' in Windows Terminal settings."
                .bright_white()
        );
    } else {
        println!(
            "\n{} {}",
            "✅".green(),
            "Windows Terminal font updated to MesloLGM Nerd Font!".green()
        );
    }

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

async fn is_powershell_available() -> bool {
    Command::new("powershell")
        .arg("-Version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok()
}

async fn run_pwsh_command(cmd: &str, use_pwsh: bool) -> Result<String> {
    println!("{} {}", "➡".blue(), cmd.bright_black());

    let shell = if use_pwsh { "pwsh" } else { "powershell" };
    let output = Command::new(shell)
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

async fn get_powershell_profile_path(use_pwsh: bool) -> Result<PathBuf> {
    let shell = if use_pwsh { "pwsh" } else { "powershell" };
    let output = Command::new(shell)
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

async fn install_module(module_name: &str, use_pwsh: bool) -> Result<()> {
    // Check if module is already installed
    let check_cmd = format!("Get-Module -ListAvailable -Name {}", module_name);
    if let Ok(output) = run_pwsh_command(&check_cmd, use_pwsh).await {
        if !output.is_empty() {
            println!(
                "{} {} {}",
                "✓".green(),
                module_name.bright_white(),
                "already installed".bright_black()
            );
            return Ok(());
        }
    }

    let cmd = format!("Install-Module {} -Force -Scope CurrentUser", module_name);
    run_pwsh_command(&cmd, use_pwsh).await?;
    Ok(())
}

async fn install_oh_my_posh() -> Result<()> {
    // Check if oh-my-posh is already installed
    if Command::new("oh-my-posh")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok()
    {
        println!(
            "{} {} {}",
            "✓".green(),
            "oh-my-posh".bright_white(),
            "already installed".bright_black()
        );
        return Ok(());
    }

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

async fn install_nerd_font() -> Result<()> {
    // Check if Meslo font is already installed
    let fonts_dir = std::env::var("LOCALAPPDATA")
        .map(|p| PathBuf::from(p).join("Microsoft\\Windows\\Fonts"))
        .ok();

    if let Some(dir) = fonts_dir {
        if dir.exists() {
            let entries = fs::read_dir(&dir).await;
            if let Ok(mut entries) = entries {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.contains("MesloLGM") || name.contains("Meslo") {
                            println!(
                                "{} {} {}",
                                "✓".green(),
                                "Meslo Nerd Font".bright_white(),
                                "already installed".bright_black()
                            );
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    println!(
        "{} {}",
        "➡".blue(),
        "oh-my-posh font install meslo".bright_black()
    );

    let status = Command::new("oh-my-posh")
        .args(["font", "install", "meslo"])
        .status()
        .await
        .context("Failed to execute oh-my-posh font install")?;

    if !status.success() {
        anyhow::bail!("oh-my-posh font install failed");
    }

    Ok(())
}

async fn update_windows_terminal_font() -> Result<()> {
    let local_appdata =
        std::env::var("LOCALAPPDATA").context("LOCALAPPDATA environment variable not found")?;

    let settings_paths = [
        format!(
            "{}\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json",
            local_appdata
        ),
        format!(
            "{}\\Packages\\Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe\\LocalState\\settings.json",
            local_appdata
        ),
        format!("{}\\Microsoft\\Windows Terminal\\settings.json", local_appdata),
    ];

    let mut found = false;
    for settings_path in &settings_paths {
        let path = PathBuf::from(settings_path);
        if !path.exists() {
            continue;
        }

        found = true;
        let content = fs::read_to_string(&path)
            .await
            .context("Failed to read settings.json")?;

        let mut json: serde_json::Value =
            serde_json::from_str(&content).context("Failed to parse settings.json")?;

        // Update font for all profiles
        if let Some(profiles) = json.get_mut("profiles") {
            if let Some(defaults) = profiles.get_mut("defaults") {
                if let Some(obj) = defaults.as_object_mut() {
                    obj.insert(
                        "font".to_string(),
                        serde_json::json!({
                            "face": "MesloLGM Nerd Font"
                        }),
                    );
                }
            } else {
                // Create defaults if it doesn't exist
                if let Some(obj) = profiles.as_object_mut() {
                    obj.insert(
                        "defaults".to_string(),
                        serde_json::json!({
                            "font": {
                                "face": "MesloLGM Nerd Font"
                            }
                        }),
                    );
                }
            }
        }

        let updated_content =
            serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?;
        fs::write(&path, updated_content)
            .await
            .context("Failed to write settings.json")?;

        println!(
            "{} {}",
            "✅".green(),
            format!("Updated: {}", path.display()).green()
        );
    }

    if !found {
        anyhow::bail!("Windows Terminal settings.json not found");
    }

    Ok(())
}
