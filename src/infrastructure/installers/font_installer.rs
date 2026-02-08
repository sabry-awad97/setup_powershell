use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

use crate::domain::interfaces::Installer;

/// Nerd Font installer using oh-my-posh
pub struct FontInstaller {
    font_name: String,
}

impl FontInstaller {
    pub fn new(font_name: impl Into<String>) -> Self {
        Self {
            font_name: font_name.into(),
        }
    }

    pub fn meslo() -> Self {
        Self::new("meslo")
    }
}

impl Default for FontInstaller {
    fn default() -> Self {
        Self::meslo()
    }
}

#[async_trait]
impl Installer for FontInstaller {
    async fn install(&self) -> Result<()> {
        println!(
            "{} {}",
            "➡".blue(),
            format!("oh-my-posh font install {}", self.font_name).bright_black()
        );

        let status = Command::new("oh-my-posh")
            .args(["font", "install", &self.font_name])
            .status()
            .await
            .context("Failed to execute oh-my-posh font install")?;

        if !status.success() {
            anyhow::bail!("oh-my-posh font install failed");
        }

        println!(
            "{} {}",
            "✅".green(),
            format!("{} font installed", self.font_name).green()
        );

        Ok(())
    }

    async fn is_installed(&self) -> bool {
        // Check if Meslo font exists in local fonts directory
        let fonts_dir = std::env::var("LOCALAPPDATA")
            .map(|p| PathBuf::from(p).join("Microsoft\\Windows\\Fonts"))
            .ok();

        if let Some(dir) = fonts_dir {
            if dir.exists() {
                if let Ok(mut entries) = fs::read_dir(&dir).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        if let Ok(name) = entry.file_name().into_string() {
                            if name.contains("MesloLGM") || name.contains("Meslo") {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn component_name(&self) -> &str {
        "Meslo Nerd Font"
    }
}
