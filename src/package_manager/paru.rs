use super::Package;
use std::process::Command as StdCommand;
use tokio::task;

#[derive(Clone)]
pub struct ParuPackageManager;

impl ParuPackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
        task::spawn_blocking(|| {
            let output = StdCommand::new("paru")
                .args(["-Qua"])
                .output()
                .map_err(|e| format!("Failed to run paru: {}", e))?;

            if !output.status.success() {
                return Ok(Vec::new());
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            let packages = parse_paru_output(&stdout);

            Ok(packages)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    /// Launch Paru upgrade in a terminal emulator
    ///
    /// Spawns the specified terminal with a Paru upgrade command that handles
    /// both official repository packages and AUR packages in a single operation.
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
                .args(["-e", "paru", "-Syu"])
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

            // Check if paru or pacman processes are running
            let process_running = StdCommand::new("pgrep")
                .arg("-x")
                .arg("paru|pacman")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);

            lock_exists || process_running
        })
        .await
        .unwrap_or(false)
    }

    pub fn name(&self) -> &'static str {
        "Pacman + AUR (paru)"
    }

    pub async fn refresh_cache(&self) -> Result<(), String> {
        // paru -Sy refreshes both official and AUR databases
        task::spawn_blocking(|| {
            let output = StdCommand::new("paru")
                .args(["-Sy"])
                .output()
                .map_err(|e| format!("Failed to refresh cache: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Cache refresh failed: {}", stderr));
            }

            Ok(())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
}

fn parse_paru_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Expected format: "package-name old-version -> new-version"
            if parts.len() >= 4 && parts[2] == "->" {
                Some(Package {
                    name: parts[0].to_string(),
                    current_version: parts[1].to_string(),
                    new_version: parts[3].to_string(),
                    is_aur: true, // All packages from paru -Qua are AUR
                })
            } else {
                None
            }
        })
        .collect()
}
