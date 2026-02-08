use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

/// Abstraction for writing profile files
#[async_trait]
pub trait ProfileWriter: Send + Sync {
    /// Write profile content to the specified path
    async fn write(&self, path: &Path, content: &str) -> Result<()>;

    /// Get the profile path for the current shell
    async fn get_profile_path(&self) -> Result<std::path::PathBuf>;
}
