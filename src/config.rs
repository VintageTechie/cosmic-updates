use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub check_interval_minutes: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check_interval_minutes: 30,
        }
    }
}

impl Config {
    /// Get the path to the config file
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("cosmic-updates");
            path.push("config.toml");
            path
        })
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
