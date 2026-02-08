use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

/// Abstraction for downloading files
#[async_trait]
pub trait Downloader: Send + Sync {
    /// Download a file from URL to the specified path
    async fn download(&self, url: &str, path: &Path) -> Result<()>;
}
