use super::Package;
use std::process::Command as StdCommand;

#[derive(Clone)]
pub struct AptPackageManager;

impl AptPackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
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
    }

    pub async fn run_upgrade(&self) -> Result<(), String> {
        StdCommand::new("cosmic-term")
            .args(["-e", "pkexec", "apt", "upgrade", "-y"])
            .spawn()
            .map_err(|e| format!("Failed to launch terminal: {}", e))?;

        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        std::path::Path::new("/var/lib/dpkg/lock-frontend").exists()
            || std::path::Path::new("/var/lib/apt/lists/lock").exists()
            || std::path::Path::new("/var/cache/apt/archives/lock").exists()
    }

    pub fn name(&self) -> &'static str {
        "APT"
    }
}

fn parse_apt_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 6 && line.contains("[upgradable from:") {
                let name = parts[0].split('/').next()?.to_string();
                let new_version = parts[1].to_string();
                let current_version = parts[5].trim_end_matches(']').to_string();

                Some(Package {
                    name,
                    current_version,
                    new_version,
                })
            } else {
                None
            }
        })
        .collect()
}

fn get_debug_packages() -> Vec<Package> {
    vec![
        Package {
            name: "firefox".to_string(),
            current_version: "120.0".to_string(),
            new_version: "121.0".to_string(),
        },
        Package {
            name: "libcosmic".to_string(),
            current_version: "0.1.0".to_string(),
            new_version: "0.2.0".to_string(),
        },
        Package {
            name: "rust-analyzer".to_string(),
            current_version: "2024-01-01".to_string(),
            new_version: "2024-02-01".to_string(),
        },
        Package {
            name: "linux-image-generic".to_string(),
            current_version: "6.5.0.14".to_string(),
            new_version: "6.5.0.15".to_string(),
        },
        Package {
            name: "systemd".to_string(),
            current_version: "255.2-1".to_string(),
            new_version: "255.4-1".to_string(),
        },
        Package {
            name: "libc6".to_string(),
            current_version: "2.39-0ubuntu8".to_string(),
            new_version: "2.39-0ubuntu8.1".to_string(),
        },
        Package {
            name: "python3".to_string(),
            current_version: "3.12.3-0".to_string(),
            new_version: "3.12.4-0".to_string(),
        },
        Package {
            name: "curl".to_string(),
            current_version: "8.5.0-2".to_string(),
            new_version: "8.6.0-1".to_string(),
        },
        Package {
            name: "git".to_string(),
            current_version: "2.43.0".to_string(),
            new_version: "2.44.0".to_string(),
        },
    ]
}
