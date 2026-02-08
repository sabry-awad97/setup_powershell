use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::process::Stdio;
use tokio::process::Command;

use crate::domain::interfaces::Installer;

/// fzf installer using winget (required for PSFzf module)
pub struct FzfInstaller;

impl FzfInstaller {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FzfInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Installer for FzfInstaller {
    async fn install(&self) -> Result<()> {
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

        println!("{} {}", "✅".green(), "fzf installed".green());

        Ok(())
    }

    async fn is_installed(&self) -> bool {
        Command::new("fzf")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .is_ok()
    }

    fn component_name(&self) -> &str {
        "fzf"
    }
}
