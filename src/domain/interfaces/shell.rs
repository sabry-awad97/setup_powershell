use anyhow::Result;
use async_trait::async_trait;

/// Abstraction for executing shell commands
#[async_trait]
pub trait ShellRunner: Send + Sync {
    /// Execute a shell command and return its output
    async fn run(&self, cmd: &str) -> Result<String>;

    /// Check if a command exists in the system
    async fn exists(&self, command: &str) -> bool;

    /// Get the shell name (e.g., "pwsh", "powershell")
    fn shell_name(&self) -> &str;
}
