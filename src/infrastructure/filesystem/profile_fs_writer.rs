use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;

use crate::domain::interfaces::{ProfileWriter, ShellRunner};

/// File system-based profile writer
pub struct ProfileFsWriter {
    shell: Arc<dyn ShellRunner>,
}

impl ProfileFsWriter {
    pub fn new(shell: Arc<dyn ShellRunner>) -> Self {
        Self { shell }
    }
}

#[async_trait]
impl ProfileWriter for ProfileFsWriter {
    async fn write(&self, path: &Path, content: &str) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create profile directory")?;
        }

        fs::write(path, content.trim())
            .await
            .context("Failed to write profile")?;

        Ok(())
    }

    async fn get_profile_path(&self) -> Result<PathBuf> {
        let output = self.shell.run("$PROFILE").await?;
        let path_str = output.trim();
        Ok(PathBuf::from(path_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::shell::PowerShellRunner;

    #[tokio::test]
    async fn test_profile_writer_creation() {
        let shell = Arc::new(PowerShellRunner::pwsh());
        let writer = ProfileFsWriter::new(shell);
        assert!(std::mem::size_of_val(&writer) > 0);
    }
}
