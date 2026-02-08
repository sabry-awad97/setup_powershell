use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use std::path::PathBuf;
use tokio::fs;

use crate::domain::interfaces::TerminalConfigurator;

/// Windows Terminal configurator
pub struct WindowsTerminalConfig;

impl WindowsTerminalConfig {
    pub fn new() -> Self {
        Self
    }

    fn get_settings_paths() -> Vec<PathBuf> {
        let local_appdata = match std::env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(_) => return vec![],
        };

        vec![
            PathBuf::from(format!(
                "{}\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json",
                local_appdata
            )),
            PathBuf::from(format!(
                "{}\\Packages\\Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe\\LocalState\\settings.json",
                local_appdata
            )),
            PathBuf::from(format!(
                "{}\\Microsoft\\Windows Terminal\\settings.json",
                local_appdata
            )),
        ]
    }
}

impl Default for WindowsTerminalConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TerminalConfigurator for WindowsTerminalConfig {
    async fn configure_font(&self, font_name: &str) -> Result<()> {
        let settings_paths = Self::get_settings_paths();
        let mut found = false;

        for settings_path in &settings_paths {
            if !settings_path.exists() {
                continue;
            }

            found = true;
            let content = fs::read_to_string(settings_path)
                .await
                .context("Failed to read settings.json")?;

            let mut json: serde_json::Value =
                serde_json::from_str(&content).context("Failed to parse settings.json")?;

            // Update font for all profiles
            if let Some(profiles) = json.get_mut("profiles") {
                if let Some(defaults) = profiles.get_mut("defaults") {
                    if let Some(obj) = defaults.as_object_mut() {
                        obj.insert(
                            "font".to_string(),
                            serde_json::json!({
                                "face": font_name
                            }),
                        );
                    }
                } else {
                    // Create defaults if it doesn't exist
                    if let Some(obj) = profiles.as_object_mut() {
                        obj.insert(
                            "defaults".to_string(),
                            serde_json::json!({
                                "font": {
                                    "face": font_name
                                }
                            }),
                        );
                    }
                }
            }

            let updated_content =
                serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?;
            fs::write(settings_path, updated_content)
                .await
                .context("Failed to write settings.json")?;

            println!(
                "{} {}",
                "✅".green(),
                format!("Updated: {}", settings_path.display()).green()
            );
        }

        if !found {
            anyhow::bail!("Windows Terminal settings.json not found");
        }

        Ok(())
    }

    fn is_supported(&self) -> bool {
        Self::get_settings_paths().iter().any(|path| path.exists())
    }
}
