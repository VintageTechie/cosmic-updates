use std::path::PathBuf;
use std::process::Command;

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

/// Detect available terminal emulator with fallback chain
///
/// Searches for terminal emulators in order of preference and returns the first one found.
/// The search order prioritizes COSMIC-native terminal, then common Linux terminals.
///
/// # Search Order
/// 1. cosmic-term (COSMIC Desktop native terminal)
/// 2. konsole (KDE terminal)
/// 3. gnome-terminal (GNOME terminal)
/// 4. xfce4-terminal (Xfce terminal)
/// 5. alacritty (Modern GPU-accelerated terminal)
/// 6. kitty (GPU-accelerated terminal)
/// 7. xterm (Fallback, nearly always available)
///
/// # Returns
/// * `String` - Name of the first available terminal, or "cosmic-term" as final fallback
pub fn detect_terminal() -> String {
    let terminals = [
        "cosmic-term",
        "konsole",
        "gnome-terminal",
        "xfce4-terminal",
        "alacritty",
        "kitty",
        "xterm",
    ];

    for terminal in &terminals {
        if Command::new("which").arg(terminal).output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return terminal.to_string();
        }
    }

    // Ultimate fallback (should rarely reach here)
    "cosmic-term".to_string()
}

/// Get the terminal emulator to use based on config preference
///
/// Resolves the terminal to use by checking the user's configuration. If set to "auto"
/// or empty, performs automatic detection. Otherwise, uses the configured terminal name.
///
/// # Arguments
/// * `preference` - User's terminal preference from config (e.g., "auto", "kitty", "konsole")
///
/// # Returns
/// * `String` - Name of the terminal emulator to use
///
/// # Examples
/// ```
/// let terminal = get_terminal("auto");  // Auto-detects available terminal
/// let terminal = get_terminal("kitty"); // Uses kitty if user specified it
/// ```
pub fn get_terminal(preference: &str) -> String {
    if preference.is_empty() || preference == "auto" {
        detect_terminal()
    } else {
        preference.to_string()
    }
}
