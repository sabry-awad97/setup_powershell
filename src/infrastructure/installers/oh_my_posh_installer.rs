use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::process::Stdio;
use tokio::process::Command;

use crate::domain::interfaces::Installer;

/// Oh-My-Posh installer using winget
pub struct OhMyPoshInstaller;

impl OhMyPoshInstaller {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OhMyPoshInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Installer for OhMyPoshInstaller {
    async fn install(&self) -> Result<()> {
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
            anyhow::bail!("winget install oh-my-posh failed");
        }

        println!("{} {}", "✅".green(), "oh-my-posh installed".green());

        Ok(())
    }

    async fn is_installed(&self) -> bool {
        Command::new("oh-my-posh")
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .is_ok()
    }

    fn component_name(&self) -> &str {
        "oh-my-posh"
    }
}
