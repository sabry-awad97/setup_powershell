use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use std::sync::Arc;

use crate::domain::interfaces::{Installer, ShellRunner};

/// PowerShell module installer
pub struct ModuleInstaller {
    module_name: String,
    shell: Arc<dyn ShellRunner>,
}

impl ModuleInstaller {
    pub fn new(module_name: impl Into<String>, shell: Arc<dyn ShellRunner>) -> Self {
        Self {
            module_name: module_name.into(),
            shell,
        }
    }
}

#[async_trait]
impl Installer for ModuleInstaller {
    async fn install(&self) -> Result<()> {
        let cmd = format!(
            "Install-Module {} -Force -Scope CurrentUser -AllowClobber",
            self.module_name
        );
        self.shell.run(&cmd).await?;
        println!(
            "{} {} {}",
            "✅".green(),
            self.module_name.bright_white(),
            "installed".green()
        );
        Ok(())
    }

    async fn is_installed(&self) -> bool {
        let cmd = format!("Get-Module -ListAvailable -Name {}", self.module_name);
        if let Ok(output) = self.shell.run(&cmd).await {
            !output.trim().is_empty()
        } else {
            false
        }
    }

    fn component_name(&self) -> &str {
        &self.module_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::shell::PowerShellRunner;

    #[tokio::test]
    async fn test_component_name() {
        let shell = Arc::new(PowerShellRunner::pwsh());
        let installer = ModuleInstaller::new("TestModule", shell);
        assert_eq!(installer.component_name(), "TestModule");
    }
}
