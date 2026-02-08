use anyhow::Result;
use async_trait::async_trait;

/// Abstraction for configuring terminal settings
#[async_trait]
pub trait TerminalConfigurator: Send + Sync {
    /// Configure the terminal font
    async fn configure_font(&self, font_name: &str) -> Result<()>;

    /// Check if terminal configuration is supported
    fn is_supported(&self) -> bool;
}
