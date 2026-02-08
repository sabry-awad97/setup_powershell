use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Confirm, MultiSelect, Select};
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

const GITHUB_RELEASES: &str = "https://github.com/PowerShell/PowerShell/releases/latest";

// Available Oh-My-Posh themes
const THEMES: &[(&str, &str)] = &[
    ("paradox", "Clean and informative with git status"),
    ("agnoster", "Classic powerline theme"),
    ("atomic", "Minimal and fast"),
    ("blue-owl", "Blue themed with icons"),
    ("bubbles", "Colorful bubble segments"),
    ("capr4n", "Compact with git info"),
    ("clean-detailed", "Detailed system info"),
    ("craver", "Developer focused"),
    ("dracula", "Dark Dracula theme"),
    ("gruvbox", "Retro groove colors"),
    ("jandedobbeleer", "Oh-My-Posh author's theme"),
    ("material", "Material design inspired"),
    ("montys", "Monty Python themed"),
    ("night-owl", "Night Owl color scheme"),
    ("powerlevel10k_rainbow", "Colorful powerline"),
    ("pure", "Minimal pure theme"),
    ("robbyrussell", "Oh-My-Zsh classic"),
    ("sonicboom_dark", "Fast and dark"),
    ("star", "Star symbols theme"),
    ("tokyo", "Tokyo Night theme"),
];

// Available plugins
const PLUGINS: &[(&str, &str)] = &[
    ("PSReadLine", "Enhanced command line editing (core)"),
    ("posh-git", "Git status in prompt (core)"),
    ("Terminal-Icons", "File and folder icons in listings"),
    ("PSFzf", "Fuzzy finder integration (auto-installs fzf)"),
    ("z", "Quick directory jumping"),
];

// Profile presets
#[derive(Debug, Clone)]
struct ProfilePreset {
    name: &'static str,
    description: &'static str,
    theme: &'static str,
    plugins: &'static [&'static str],
    custom_aliases: bool,
}

const PROFILE_PRESETS: &[ProfilePreset] = &[
    ProfilePreset {
        name: "Minimal",
        description: "Basic setup with essential features only",
        theme: "pure",
        plugins: &["PSReadLine", "posh-git"],
        custom_aliases: false,
    },
    ProfilePreset {
        name: "Developer",
        description: "Full-featured setup for developers",
        theme: "paradox",
        plugins: &["PSReadLine", "posh-git", "Terminal-Icons", "PSFzf", "z"],
        custom_aliases: true,
    },
    ProfilePreset {
        name: "Work",
        description: "Professional setup with productivity tools",
        theme: "jandedobbeleer",
        plugins: &["PSReadLine", "posh-git", "Terminal-Icons", "PSFzf"],
        custom_aliases: true,
    },
    ProfilePreset {
        name: "Custom",
        description: "Choose your own theme and plugins",
        theme: "",
        plugins: &[],
        custom_aliases: true,
    },
];

fn generate_profile_content(theme: &str, plugins: &[String], include_aliases: bool) -> String {
    let mut content = String::from(
        r#"# ===========================
# Modern PowerShell 7 Profile
# ===========================

"#,
    );

    // Import modules
    content.push_str("# --- Import Modules ---\n");
    for plugin in plugins {
        if plugin == "PSReadLine" || plugin == "posh-git" {
            content.push_str(&format!("Import-Module {}\n", plugin));
        } else {
            content.push_str(&format!(
                "if (Get-Module -ListAvailable -Name {}) {{ Import-Module {} }}\n",
                plugin, plugin
            ));
        }
    }
    content.push('\n');

    // Oh-My-Posh theme
    content.push_str(&format!(
        r#"# Oh-My-Posh prompt theme
if (Get-Command oh-my-posh -ErrorAction SilentlyContinue) {{
    $configPath = "$env:POSH_THEMES_PATH\{}.omp.json"
    if (Test-Path $configPath) {{
        oh-my-posh init pwsh --config $configPath | Invoke-Expression
    }} else {{
        oh-my-posh init pwsh | Invoke-Expression
    }}
}}

"#,
        theme
    ));

    // PSReadLine settings
    content.push_str(
        r#"# --- PSReadLine Settings ---
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

"#,
    );

    // Aliases
    if include_aliases {
        content.push_str(
            r#"# --- Aliases ---
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

"#,
        );
    }

    // Environment
    content.push_str(
        r#"# --- Environment ---
$env:POSH_GIT_ENABLED = $true
"#,
    );

    content
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n{}", "🚀 PowerShell Setup Tool".cyan().bold());
    println!("{}\n", "━".repeat(60).bright_black());

    let use_pwsh = check_and_install_pwsh().await?;

    // Profile selection
    println!(
        "\n{} {}",
        "📋".cyan(),
        "Choose a profile preset:".cyan().bold()
    );
    let preset_names: Vec<String> = PROFILE_PRESETS
        .iter()
        .map(|p| format!("{} - {}", p.name, p.description))
        .collect();

    let preset_idx = Select::new()
        .with_prompt("Select profile")
        .items(&preset_names)
        .default(1)
        .interact()?;

    let selected_preset = &PROFILE_PRESETS[preset_idx];

    let (theme, plugins) = if selected_preset.name == "Custom" {
        select_custom_configuration().await?
    } else {
        (
            selected_preset.theme.to_string(),
            selected_preset
                .plugins
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    };

    let profile_path = get_powershell_profile_path(use_pwsh).await?;
    let profile_dir = profile_path
        .parent()
        .context("Failed to get profile directory")?;

    fs::create_dir_all(profile_dir)
        .await
        .context("Failed to create profile directory")?;

    // Install core components
    println!(
        "\n{} {}",
        "📦".cyan(),
        "Installing core components...".cyan()
    );
    let (r1, r2, r3) = tokio::join!(
        install_oh_my_posh(),
        install_nerd_font(),
        update_windows_terminal_font()
    );

    for result in [r1, r2] {
        if let Err(e) = result {
            eprintln!("{} {}", "⚠".yellow(), format!("Warning: {}", e).yellow());
        }
    }

    if let Err(e) = r3 {
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
            "Windows Terminal font updated!".green()
        );
    }

    // Install selected plugins
    println!(
        "\n{} {}",
        "🔌".cyan(),
        "Installing selected plugins...".cyan()
    );

    for plugin in &plugins {
        // If PSFzf is selected, install fzf first
        if plugin == "PSFzf" {
            if let Err(e) = install_fzf().await {
                eprintln!(
                    "{} {}",
                    "⚠".yellow(),
                    format!("Warning: Failed to install fzf: {}. PSFzf may not work.", e).yellow()
                );
            }
        }

        if let Err(e) = install_module(plugin, use_pwsh).await {
            eprintln!(
                "{} {}",
                "⚠".yellow(),
                format!("Warning: Failed to install {}: {}", plugin, e).yellow()
            );
        }
    }

    // Generate and write profile
    let profile_content =
        generate_profile_content(&theme, &plugins, selected_preset.custom_aliases);

    fs::write(&profile_path, profile_content.trim())
        .await
        .context("Failed to write profile")?;

    print_summary(selected_preset, &theme, &plugins, &profile_path, use_pwsh);

    Ok(())
}

async fn check_and_install_pwsh() -> Result<bool> {
    if is_pwsh_available().await {
        return Ok(true);
    }

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

        if is_powershell_available().await {
            println!(
                "\n{} {}\n",
                "🔄".cyan(),
                "Continuing with Windows PowerShell...".cyan()
            );
            return Ok(false);
        } else {
            anyhow::bail!("No PowerShell version found");
        }
    }

    println!("{}", "⬇ Installing PowerShell 7...".cyan());
    download_and_install_powershell().await?;

    if !is_pwsh_available().await {
        println!("\n{}", "━".repeat(60).bright_black());
        println!(
            "{} {}",
            "✅".green(),
            "PowerShell 7 installed!".green().bold()
        );
        println!("{}", "━".repeat(60).bright_black());
        println!(
            "\n{} {}",
            "ℹ".blue(),
            "Please restart your terminal and run this program again.".bright_white()
        );
        std::process::exit(0);
    }

    Ok(true)
}

async fn select_custom_configuration() -> Result<(String, Vec<String>)> {
    // Theme selection
    println!("\n{} {}", "🎨".cyan(), "Choose a theme:".cyan().bold());
    let theme_items: Vec<String> = THEMES
        .iter()
        .map(|(name, desc)| format!("{} - {}", name, desc))
        .collect();

    let theme_idx = Select::new()
        .with_prompt("Select theme")
        .items(&theme_items)
        .default(0)
        .interact()?;

    let selected_theme = THEMES[theme_idx].0.to_string();

    // Plugin selection
    println!(
        "\n{} {}",
        "🔌".cyan(),
        "Choose plugins to install:".cyan().bold()
    );
    let plugin_items: Vec<String> = PLUGINS
        .iter()
        .map(|(name, desc)| format!("{} - {}", name, desc))
        .collect();

    let plugin_indices = MultiSelect::new()
        .with_prompt("Select plugins (Space to toggle, Enter to confirm)")
        .items(&plugin_items)
        .defaults(&[true, true, false, false, false])
        .interact()?;

    let selected_plugins: Vec<String> = plugin_indices
        .iter()
        .map(|&i| PLUGINS[i].0.to_string())
        .collect();

    Ok((selected_theme, selected_plugins))
}

fn print_summary(
    preset: &ProfilePreset,
    theme: &str,
    plugins: &[String],
    profile_path: &Path,
    use_pwsh: bool,
) {
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
        format!("Profile: {}", preset.name).bright_white()
    );
    println!(
        "{} {}",
        "🎨".blue(),
        format!("Theme: {}", theme).bright_white()
    );
    println!(
        "{} {}",
        "🔌".blue(),
        format!("Plugins: {}", plugins.join(", ")).bright_white()
    );
    println!(
        "\n{} {}",
        "📄".blue(),
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
        println!("{} {}", "✅".green(), "PowerShell 7 installed!".green());
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

async fn install_fzf() -> Result<()> {
    // Check if fzf is already installed
    if Command::new("fzf")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok()
    {
        println!(
            "{} {} {}",
            "✓".green(),
            "fzf".bright_white(),
            "already installed".bright_black()
        );
        return Ok(());
    }

    println!(
        "{} {}",
        "➡".blue(),
        "winget install fzf -s winget".bright_black()
    );

    let status = Command::new("winget")
        .args([
            "install",
            "fzf",
            "-s",
            "winget",
            "--accept-source-agreements",
        ])
        .status()
        .await
        .context("Failed to execute winget")?;

    if !status.success() {
        anyhow::bail!("winget install fzf failed");
    }

    println!("{} {}", "✅".green(), "fzf installed successfully!".green());

    Ok(())
}
