use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;

/// Configuration for the update checker applet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// How often to check for updates (in minutes)
    pub check_interval_minutes: u64,
    /// Whether to show desktop notifications when updates are available
    #[serde(default = "default_true")]
    pub enable_notifications: bool,
    /// Number of updates required to mark notification as critical/urgent
    #[serde(default = "default_urgency_threshold")]
    pub urgency_threshold: u32,
    /// Preferred terminal emulator (auto-detected if not set or "auto")
    #[serde(default = "default_terminal")]
    pub terminal: String,
}

fn default_true() -> bool {
    true
}

fn default_urgency_threshold() -> u32 {
    10 // Default: consider critical when 10+ updates available
}

fn default_terminal() -> String {
    "auto".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check_interval_minutes: 30,
            enable_notifications: true,
            urgency_threshold: 10,
            terminal: "auto".to_string(),
        }
    }
}

impl Config {
    /// Get the path to the config file (checks new location, falls back to old)
    fn config_path() -> Option<std::path::PathBuf> {
        utils::get_app_file_path("config.toml")
    }

    /// Load config from file, or create default if it doesn't exist
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str(&contents) {
                        return config;
                    }
                }
            }
        }
        // Return default if file doesn't exist or can't be read
        Self::default()
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Could not determine config directory")?;

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        // Serialize and write
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, toml_string).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}
