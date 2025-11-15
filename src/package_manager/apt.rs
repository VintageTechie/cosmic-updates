use super::Package;
use std::process::Command as StdCommand;
use tokio::task;

#[derive(Clone)]
pub struct AptPackageManager;

impl AptPackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
        task::spawn_blocking(|| {
            if std::env::var("DEBUG_APT_CHECKER").is_ok() {
                return Ok(get_debug_packages());
            }

            let output = StdCommand::new("apt")
                .args(["list", "--upgradable"])
                .output()
                .map_err(|e| format!("Failed to run apt: {}", e))?;

            if !output.status.success() {
                return Err("apt command failed".to_string());
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            let packages = parse_apt_output(&stdout);

            Ok(packages)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    /// Launch APT upgrade in a terminal emulator
    ///
    /// Spawns the specified terminal with an interactive APT upgrade session.
    /// Uses pkexec for privilege escalation to perform system package upgrades.
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
                .args(["-e", "pkexec", "apt", "upgrade", "-y"])
                .spawn()
                .map_err(|e| format!("Failed to launch terminal '{}': {}", terminal, e))?;

            Ok(())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    pub async fn is_running(&self) -> bool {
        task::spawn_blocking(|| {
            // Check both lock files and running processes
            let locks_exist = std::path::Path::new("/var/lib/dpkg/lock-frontend").exists()
                || std::path::Path::new("/var/lib/apt/lists/lock").exists()
                || std::path::Path::new("/var/cache/apt/archives/lock").exists();

            // Check if apt, apt-get, or dpkg processes are running
            let processes_running = StdCommand::new("pgrep")
                .arg("-x")
                .arg("apt|apt-get|dpkg")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);

            locks_exist || processes_running
        })
        .await
        .unwrap_or(false)
    }

    pub fn name(&self) -> &'static str {
        "APT"
    }

    pub async fn refresh_cache(&self) -> Result<(), String> {
        task::spawn_blocking(|| {
            let output = StdCommand::new("pkexec")
                .args(["apt", "update"])
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

/// Parse APT output into a list of packages
///
/// Expected format: "package/repo version [architecture] [upgradable from: old_version]"
/// Example: "firefox/jammy-updates 121.0+build1-0ubuntu0.22.04.1 amd64 [upgradable from: 120.0+build2-0ubuntu0.22.04.1]"
fn parse_apt_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .skip(1) // Skip header line
        .filter_map(|line| {
            // Only process lines that contain the upgradable marker
            if !line.contains("[upgradable from:") {
                return None;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            // Validate we have enough parts for a complete package entry
            if parts.len() < 6 {
                eprintln!("Warning: Skipping malformed APT line (insufficient fields): {}", line);
                return None;
            }

            // Extract package name (before the '/')
            let name = parts.first()?.split('/').next()?.to_string();
            if name.is_empty() {
                eprintln!("Warning: Skipping APT line with empty package name: {}", line);
                return None;
            }

            // Extract new version
            let new_version = parts.get(1)?.to_string();

            // Extract current version (remove trailing ']')
            let current_version = parts.get(5)?.trim_end_matches(']').to_string();
            if current_version.is_empty() {
                eprintln!("Warning: Skipping APT line with empty version: {}", line);
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

fn get_debug_packages() -> Vec<Package> {
    vec![
        Package {
            name: "firefox".to_string(),
            current_version: "120.0".to_string(),
            new_version: "121.0".to_string(),
            is_aur: false,
        },
        Package {
            name: "libcosmic".to_string(),
            current_version: "0.1.0".to_string(),
            new_version: "0.2.0".to_string(),
            is_aur: false,
        },
        Package {
            name: "rust-analyzer".to_string(),
            current_version: "2024-01-01".to_string(),
            new_version: "2024-02-01".to_string(),
            is_aur: false,
        },
        Package {
            name: "linux-image-generic".to_string(),
            current_version: "6.5.0.14".to_string(),
            new_version: "6.5.0.15".to_string(),
            is_aur: false,
        },
        Package {
            name: "systemd".to_string(),
            current_version: "255.2-1".to_string(),
            new_version: "255.4-1".to_string(),
            is_aur: false,
        },
        Package {
            name: "libc6".to_string(),
            current_version: "2.39-0ubuntu8".to_string(),
            new_version: "2.39-0ubuntu8.1".to_string(),
            is_aur: false,
        },
        Package {
            name: "python3".to_string(),
            current_version: "3.12.3-0".to_string(),
            new_version: "3.12.4-0".to_string(),
            is_aur: false,
        },
        Package {
            name: "curl".to_string(),
            current_version: "8.5.0-2".to_string(),
            new_version: "8.6.0-1".to_string(),
            is_aur: false,
        },
        Package {
            name: "git".to_string(),
            current_version: "2.43.0".to_string(),
            new_version: "2.44.0".to_string(),
            is_aur: false,
        },
    ]
}
