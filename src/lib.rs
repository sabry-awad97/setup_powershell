pub mod application;
pub mod cli;
pub mod domain;
pub mod infrastructure;

pub use application::SetupService;
pub use cli::SetupCli;
pub use domain::{ProfileConfig, ProfilePreset};
