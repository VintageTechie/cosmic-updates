use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;

/// Persistent state for tracking notification behavior
///
/// Stores information between app restarts to enable intelligent notification logic,
/// such as only notifying when the update count increases.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    /// Number of available updates from the last check
    pub last_update_count: usize,
}

impl State {
    /// Get the path to the state file (checks new location, falls back to old)
    fn state_path() -> Option<std::path::PathBuf> {
        utils::get_app_file_path("state.toml")
    }

    /// Load state from file, or create default if it doesn't exist
    pub fn load() -> Self {
        if let Some(path) = Self::state_path() {
            if path.exists() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(state) = toml::from_str(&contents) {
                        return state;
                    }
                }
            }
        }
        // Return default if file doesn't exist or can't be read
        Self::default()
    }

    /// Save state to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::state_path().ok_or("Could not determine config directory")?;

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create state directory: {}", e))?;
        }

        // Serialize and write
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;

        fs::write(&path, toml_string).map_err(|e| format!("Failed to write state file: {}", e))?;

        Ok(())
    }
}
