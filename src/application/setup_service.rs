use anyhow::Result;
use colored::Colorize;
use std::sync::Arc;
use typed_builder::TypedBuilder;

use crate::domain::{Installer, ProfileConfig, ProfileWriter, ShellRunner, TerminalConfigurator};

/// Core application service for orchestrating PowerShell setup
#[derive(TypedBuilder)]
pub struct SetupService {
    shell: Arc<dyn ShellRunner>,
    #[allow(dead_code)]
    pwsh_installer: Arc<dyn Installer>,
    font_installer: Arc<dyn Installer>,
    terminal_config: Arc<dyn TerminalConfigurator>,
    profile_writer: Arc<dyn ProfileWriter>,
    #[builder(default)]
    module_installers: Vec<Arc<dyn Installer>>,
}

impl SetupService {
    /// Run the complete setup process
    pub async fn run_setup(&self, config: &ProfileConfig) -> Result<()> {
        // Note: PowerShell installation is handled in main.rs before service creation
        self.install_core_components().await?;
        self.install_modules(&config.plugins).await?;
        self.write_profile(config).await?;

        self.print_success(config).await?;

        Ok(())
    }

    /// Install core components (fonts, terminal config)
    async fn install_core_components(&self) -> Result<()> {
        println!(
            "\n{} {}",
            "📦".cyan(),
            "Installing core components...".cyan()
        );

        // Install font
        if !self.font_installer.is_installed().await {
            self.font_installer.install().await?;
        } else {
            println!(
                "{} {} {}",
                "✓".green(),
                self.font_installer.component_name().bright_white(),
                "already installed".bright_black()
            );
        }

        // Configure terminal
        if self.terminal_config.is_supported() {
            match self
                .terminal_config
                .configure_font("MesloLGM Nerd Font")
                .await
            {
                Ok(_) => println!(
                    "{} {}",
                    "✅".green(),
                    "Windows Terminal font updated!".green()
                ),
                Err(e) => println!(
                    "{} {}",
                    "⚠".yellow(),
                    format!("Could not configure terminal font: {}", e).yellow()
                ),
            }
        }

        Ok(())
    }

    /// Install PowerShell modules
    async fn install_modules(&self, plugins: &[String]) -> Result<()> {
        println!(
            "\n{} {}",
            "🔌".cyan(),
            "Installing selected plugins...".cyan()
        );

        for plugin in plugins {
            // Find installer for this plugin
            if let Some(installer) = self
                .module_installers
                .iter()
                .find(|i| i.component_name() == plugin)
            {
                if !installer.is_installed().await {
                    if let Err(e) = installer.install().await {
                        eprintln!(
                            "{} {}",
                            "⚠".yellow(),
                            format!("Warning: Failed to install {}: {}", plugin, e).yellow()
                        );
                    }
                } else {
                    println!(
                        "{} {} {}",
                        "✓".green(),
                        plugin.bright_white(),
                        "already installed".bright_black()
                    );
                }
            }
        }

        Ok(())
    }

    /// Write the PowerShell profile
    async fn write_profile(&self, config: &ProfileConfig) -> Result<()> {
        let profile_path = self.profile_writer.get_profile_path().await?;
        let content = self.generate_profile_content(config);

        self.profile_writer.write(&profile_path, &content).await?;

        Ok(())
    }

    /// Generate profile content
    fn generate_profile_content(&self, config: &ProfileConfig) -> String {
        let mut content = String::from(
            r#"# ===========================
# Modern PowerShell 7 Profile
# ===========================

"#,
        );

        // Import modules
        content.push_str("# --- Import Modules ---\n");
        for plugin in &config.plugins {
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
            config.theme
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
        if config.include_aliases {
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

    /// Print success message
    async fn print_success(&self, config: &ProfileConfig) -> Result<()> {
        let profile_path = self.profile_writer.get_profile_path().await?;

        println!("\n{}", "━".repeat(60).bright_black());
        println!(
            "{} {}",
            "✅".green(),
            "Setup completed successfully!".green().bold()
        );
        println!("{}", "━".repeat(60).bright_black());
        println!(
            "\n{} {}",
            "🎨".blue(),
            format!("Theme: {}", config.theme).bright_white()
        );
        println!(
            "{} {}",
            "🔌".blue(),
            format!("Plugins: {}", config.plugins.join(", ")).bright_white()
        );
        println!(
            "\n{} {}",
            "📄".blue(),
            format!("Profile written to: {}", profile_path.display()).bright_white()
        );
        println!(
            "\n{} {}\n",
            "🔄".cyan(),
            format!("Restart {} to see the changes.", self.shell.shell_name()).cyan()
        );

        Ok(())
    }
}
