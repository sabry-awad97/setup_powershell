pub mod entities;
pub mod interfaces;

pub use entities::{ProfileConfig, ProfilePreset};
pub use interfaces::{Downloader, Installer, ProfileWriter, ShellRunner, TerminalConfigurator};
