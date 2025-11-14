use std::path::PathBuf;

/// Get the path to an application file, checking the new location first and falling back to the old location
///
/// This function handles the migration from the old app name (cosmic-updates) to the new name (cosmic-ext-applet-updates).
/// It first checks the new location, and if the file doesn't exist there, it checks the old location.
///
/// # Arguments
/// * `filename` - The name of the file to locate (e.g., "config.toml", "state.toml")
///
/// # Returns
/// * `Some(PathBuf)` - Path to the file (either new or old location)
/// * `None` - If the config directory cannot be determined
pub fn get_app_file_path(filename: &str) -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        // Try new location first
        path.push("cosmic-ext-applet-updates");
        path.push(filename);

        // If new location doesn't exist, check old location
        if !path.exists() {
            if let Some(mut old_path) = dirs::config_dir() {
                old_path.push("cosmic-updates");
                old_path.push(filename);
                if old_path.exists() {
                    return old_path;
                }
            }
        }

        path
    })
}
