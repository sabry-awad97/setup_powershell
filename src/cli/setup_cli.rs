use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, MultiSelect, Select};

use crate::domain::{ProfileConfig, ProfilePreset};

/// Available Oh-My-Posh themes
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

/// Available plugins
const PLUGINS: &[(&str, &str)] = &[
    ("PSReadLine", "Enhanced command line editing (core)"),
    ("posh-git", "Git status in prompt (core)"),
    ("Terminal-Icons", "File and folder icons in listings"),
    ("PSFzf", "Fuzzy finder integration (auto-installs fzf)"),
    ("z", "Quick directory jumping"),
];

/// CLI interface for PowerShell setup
pub struct SetupCli;

impl SetupCli {
    pub fn new() -> Self {
        Self
    }

    /// Show welcome banner
    pub fn show_banner(&self) {
        println!("\n{}", "🚀 PowerShell Setup Tool".cyan().bold());
        println!("{}\n", "━".repeat(60).bright_black());
    }

    /// Prompt user to install PowerShell 7
    pub fn prompt_install_pwsh(&self) -> Result<bool> {
        println!("{}", "❌ pwsh (PowerShell 7) not found.".red());

        Confirm::new()
            .with_prompt("Would you like to download and install PowerShell 7?")
            .default(true)
            .interact()
            .map_err(Into::into)
    }

    /// Select a profile preset
    pub fn select_preset(&self) -> Result<&'static ProfilePreset> {
        println!(
            "\n{} {}",
            "📋".cyan(),
            "Choose a profile preset:".cyan().bold()
        );

        let preset_names: Vec<String> = ProfilePreset::all()
            .iter()
            .map(|p| format!("{} - {}", p.name, p.description))
            .collect();

        let preset_idx = Select::new()
            .with_prompt("Select profile")
            .items(&preset_names)
            .default(1)
            .interact()?;

        Ok(&ProfilePreset::all()[preset_idx])
    }

    /// Select custom theme and plugins
    pub fn select_custom_configuration(&self) -> Result<(String, Vec<String>)> {
        let theme = self.select_theme()?;
        let plugins = self.select_plugins()?;
        Ok((theme, plugins))
    }

    /// Select a theme
    fn select_theme(&self) -> Result<String> {
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

        Ok(THEMES[theme_idx].0.to_string())
    }

    /// Select plugins
    fn select_plugins(&self) -> Result<Vec<String>> {
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

        Ok(selected_plugins)
    }

    /// Build profile configuration from preset
    pub fn build_config_from_preset(&self, preset: &ProfilePreset) -> Result<ProfileConfig> {
        let (theme, plugins) = if preset.name == "Custom" {
            self.select_custom_configuration()?
        } else {
            (
                preset.theme.to_string(),
                preset.plugins.iter().map(|s| s.to_string()).collect(),
            )
        };

        Ok(ProfileConfig::builder()
            .theme(theme)
            .plugins(plugins)
            .include_aliases(preset.include_aliases)
            .build())
    }
}

impl Default for SetupCli {
    fn default() -> Self {
        Self::new()
    }
}
