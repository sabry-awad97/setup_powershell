use anyhow::Result;
use async_trait::async_trait;

/// Abstraction for installing components
#[async_trait]
pub trait Installer: Send + Sync {
    /// Install the component
    async fn install(&self) -> Result<()>;

    /// Check if the component is already installed
    async fn is_installed(&self) -> bool;

    /// Get the component name
    fn component_name(&self) -> &str;
}
