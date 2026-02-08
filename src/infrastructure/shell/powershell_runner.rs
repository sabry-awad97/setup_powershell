use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::process::Stdio;
use tokio::process::Command;

use crate::domain::interfaces::ShellRunner;

/// PowerShell command runner implementation
pub struct PowerShellRunner {
    shell: String,
}

impl PowerShellRunner {
    pub fn new(use_pwsh: bool) -> Self {
        Self {
            shell: if use_pwsh {
                "pwsh".to_string()
            } else {
                "powershell".to_string()
            },
        }
    }

    pub fn pwsh() -> Self {
        Self::new(true)
    }

    pub fn powershell() -> Self {
        Self::new(false)
    }
}

#[async_trait]
impl ShellRunner for PowerShellRunner {
    async fn run(&self, cmd: &str) -> Result<String> {
        println!("{} {}", "➡".blue(), cmd.bright_black());

        let output = Command::new(&self.shell)
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

    async fn exists(&self, command: &str) -> bool {
        Command::new(command)
            .arg("-Version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .is_ok()
    }

    fn shell_name(&self) -> &str {
        &self.shell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shell_name() {
        let runner = PowerShellRunner::pwsh();
        assert_eq!(runner.shell_name(), "pwsh");

        let runner = PowerShellRunner::powershell();
        assert_eq!(runner.shell_name(), "powershell");
    }
}
