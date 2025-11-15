use super::Package;
use std::process::Command as StdCommand;
use tokio::task;

#[derive(Clone)]
pub struct PacmanPackageManager;

impl PacmanPackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
        task::spawn_blocking(|| {
            let output = StdCommand::new("checkupdates")
                .output()
                .map_err(|e| format!("Failed to run checkupdates: {}", e))?;

            if !output.status.success() {
                return Ok(Vec::new());
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            let packages = parse_pacman_output(&stdout);

            Ok(packages)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    /// Launch Pacman upgrade in a terminal emulator
    ///
    /// Spawns the specified terminal with a Pacman system upgrade command.
    /// Uses pkexec for privilege escalation and --noconfirm for non-interactive upgrades.
    ///
    /// # Arguments
    /// * `terminal` - Terminal emulator to use (e.g., "cosmic-term", "konsole")
    ///
    /// # Returns
    /// * `Ok(())` - Terminal process spawned successfully
    /// * `Err(String)` - Failed to spawn terminal
    pub async fn run_upgrade(&self, terminal: &str) -> Result<(), String> {
        let terminal = terminal.to_string();
        task::spawn_blocking(move || {
            StdCommand::new(&terminal)
                .args(["-e", "pkexec", "pacman", "-Syu", "--noconfirm"])
                .spawn()
                .map_err(|e| format!("Failed to launch terminal '{}': {}", terminal, e))?;

            Ok(())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    pub async fn is_running(&self) -> bool {
        task::spawn_blocking(|| {
            // Check both lock file and running processes
            let lock_exists = std::path::Path::new("/var/lib/pacman/db.lck").exists();

            // Check if pacman process is running
            let process_running = StdCommand::new("pgrep")
                .arg("-x")
                .arg("pacman")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);

            lock_exists || process_running
        })
        .await
        .unwrap_or(false)
    }

    pub fn name(&self) -> &'static str {
        "Pacman"
    }

    pub async fn refresh_cache(&self) -> Result<(), String> {
        // Pacman's database is automatically updated by checkupdates
        // and pacman -Syu, so we don't need a separate refresh
        // Use spawn_blocking for consistency even though this is a no-op
        task::spawn_blocking(|| Ok(()))
            .await
            .map_err(|e| format!("Task join error: {}", e))?
    }
}

/// Parse Pacman checkupdates output into a list of packages
///
/// Expected format: "package current_version -> new_version"
/// Example: "firefox 121.0-1 -> 122.0-1"
fn parse_pacman_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            // Validate we have all required fields
            if parts.len() < 4 {
                if !line.trim().is_empty() {
                    eprintln!("Warning: Skipping malformed Pacman line (insufficient fields): {}", line);
                }
                return None;
            }

            // Validate the arrow separator is in the expected position
            if parts.get(2)? != &"->" {
                eprintln!("Warning: Skipping Pacman line with unexpected format (missing '->'): {}", line);
                return None;
            }

            let name = parts.get(0)?.to_string();
            let current_version = parts.get(1)?.to_string();
            let new_version = parts.get(3)?.to_string();

            // Validate non-empty fields
            if name.is_empty() || current_version.is_empty() || new_version.is_empty() {
                eprintln!("Warning: Skipping Pacman line with empty fields: {}", line);
                return None;
            }

            Some(Package {
                name,
                current_version,
                new_version,
                is_aur: false,
            })
        })
        .collect()
}
