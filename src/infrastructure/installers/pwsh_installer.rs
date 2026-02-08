use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::process::Stdio;
use tokio::process::Command;

use crate::domain::interfaces::{Downloader, Installer};

const GITHUB_RELEASES: &str = "https://github.com/PowerShell/PowerShell/releases/latest";

/// PowerShell 7 installer
pub struct PwshInstaller {
    downloader: std::sync::Arc<dyn Downloader>,
}

impl PwshInstaller {
    pub fn new(downloader: std::sync::Arc<dyn Downloader>) -> Self {
        Self { downloader }
    }

    async fn get_latest_version(&self) -> Result<String> {
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

        Ok(version.to_string())
    }
}

#[async_trait]
impl Installer for PwshInstaller {
    async fn install(&self) -> Result<()> {
        println!(
            "{} {}",
            "🔍".cyan(),
            "Checking latest PowerShell release...".cyan()
        );

        let version = self.get_latest_version().await?;

        println!(
            "{} {}",
            "✅".green(),
            format!("Latest PowerShell: {}", version).green()
        );

        let version_number = version.strip_prefix('v').unwrap_or(&version);
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

        self.downloader.download(&download_url, &msi_path).await?;

        println!(
            "{} {}",
            "✅".green(),
            format!("Downloaded to {}", msi_path.display()).green()
        );

        println!(
            "{} {}",
            "⚙".yellow(),
            "Installing PowerShell 7 (may need admin rights)...".yellow()
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

    async fn is_installed(&self) -> bool {
        Command::new("pwsh")
            .arg("-Version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .is_ok()
    }

    fn component_name(&self) -> &str {
        "PowerShell 7"
    }
}
