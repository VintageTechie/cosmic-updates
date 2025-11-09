use super::Package;
use std::process::Command as StdCommand;

#[derive(Clone)]
pub struct YayPackageManager;

impl YayPackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
        let output = StdCommand::new("yay")
            .args(["-Qua"])
            .output()
            .map_err(|e| format!("Failed to run yay: {}", e))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages = parse_yay_output(&stdout);

        Ok(packages)
    }

    pub async fn run_upgrade(&self) -> Result<(), String> {
        // yay -Syu handles both official repos and AUR
        StdCommand::new("cosmic-term")
            .args(["-e", "yay", "-Syu"])
            .spawn()
            .map_err(|e| format!("Failed to launch terminal: {}", e))?;

        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        // Check if pacman lock exists (yay uses pacman)
        std::path::Path::new("/var/lib/pacman/db.lck").exists()
    }

    pub fn name(&self) -> &'static str {
        "Pacman + AUR (yay)"
    }

    pub async fn refresh_cache(&self) -> Result<(), String> {
        // yay -Sy refreshes both official and AUR databases
        let output = StdCommand::new("yay")
            .args(["-Sy"])
            .output()
            .map_err(|e| format!("Failed to refresh cache: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Cache refresh failed: {}", stderr));
        }

        Ok(())
    }
}

fn parse_yay_output(output: &str) -> Vec<Package> {
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
                    is_aur: true, // All packages from yay -Qua are AUR
                })
            } else {
                None
            }
        })
        .collect()
}
