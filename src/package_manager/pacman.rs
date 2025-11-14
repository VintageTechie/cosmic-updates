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

    pub async fn run_upgrade(&self) -> Result<(), String> {
        task::spawn_blocking(|| {
            StdCommand::new("cosmic-term")
                .args(["-e", "pkexec", "pacman", "-Syu", "--noconfirm"])
                .spawn()
                .map_err(|e| format!("Failed to launch terminal: {}", e))?;

            Ok(())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    pub async fn is_running(&self) -> bool {
        task::spawn_blocking(|| {
            std::path::Path::new("/var/lib/pacman/db.lck").exists()
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

fn parse_pacman_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                Some(Package {
                    name: parts[0].to_string(),
                    current_version: parts[1].to_string(),
                    new_version: parts[3].to_string(),
                    is_aur: false, // Official repo packages
                })
            } else {
                None
            }
        })
        .collect()
}
