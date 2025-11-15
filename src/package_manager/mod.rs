pub mod apt;
pub mod pacman;
pub mod paru;
pub mod yay;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub is_aur: bool, // Track if this is from AUR
}

#[derive(Clone)]
pub enum PackageManager {
    Apt(apt::AptPackageManager),
    Pacman(pacman::PacmanPackageManager),
    CombinedParu(pacman::PacmanPackageManager, paru::ParuPackageManager), // Pacman + paru
    CombinedYay(pacman::PacmanPackageManager, yay::YayPackageManager),    // Pacman + yay
}

impl PackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
        match self {
            PackageManager::Apt(pm) => pm.check_updates().await,
            PackageManager::Pacman(pm) => pm.check_updates().await,
            PackageManager::CombinedParu(pacman, paru) => {
                // Get both official repo and AUR updates
                let mut all_packages = Vec::new();

                // Official repos
                let mut official = pacman.check_updates().await?;
                for pkg in &mut official {
                    pkg.is_aur = false;
                }
                all_packages.extend(official);

                // AUR packages
                let aur = paru.check_updates().await?;
                all_packages.extend(aur);

                Ok(all_packages)
            }
            PackageManager::CombinedYay(pacman, yay) => {
                // Get both official repo and AUR updates
                let mut all_packages = Vec::new();

                // Official repos
                let mut official = pacman.check_updates().await?;
                for pkg in &mut official {
                    pkg.is_aur = false;
                }
                all_packages.extend(official);

                // AUR packages
                let aur = yay.check_updates().await?;
                all_packages.extend(aur);

                Ok(all_packages)
            }
        }
    }

    /// Launch the system upgrade process in a terminal emulator
    ///
    /// Opens a terminal window with the appropriate package manager upgrade command.
    /// Uses privilege escalation (pkexec) where necessary for system package updates.
    ///
    /// # Arguments
    /// * `terminal` - Name of the terminal emulator to use (e.g., "cosmic-term", "konsole")
    ///
    /// # Returns
    /// * `Ok(())` - Terminal launched successfully
    /// * `Err(String)` - Failed to launch terminal with error message
    pub async fn run_upgrade(&self, terminal: &str) -> Result<(), String> {
        match self {
            PackageManager::Apt(pm) => pm.run_upgrade(terminal).await,
            PackageManager::Pacman(pm) => pm.run_upgrade(terminal).await,
            PackageManager::CombinedParu(_pacman, paru) => {
                // Use paru for upgrade since it handles both official + AUR
                paru.run_upgrade(terminal).await
            }
            PackageManager::CombinedYay(_pacman, yay) => {
                // Use yay for upgrade since it handles both official + AUR
                yay.run_upgrade(terminal).await
            }
        }
    }

    pub async fn is_running(&self) -> bool {
        match self {
            PackageManager::Apt(pm) => pm.is_running().await,
            PackageManager::Pacman(pm) => pm.is_running().await,
            PackageManager::CombinedParu(pacman, _paru) => pacman.is_running().await,
            PackageManager::CombinedYay(pacman, _yay) => pacman.is_running().await,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PackageManager::Apt(pm) => pm.name(),
            PackageManager::Pacman(pm) => pm.name(),
            PackageManager::CombinedParu(_pacman, paru) => paru.name(),
            PackageManager::CombinedYay(_pacman, yay) => yay.name(),
        }
    }

    pub async fn refresh_cache(&self) -> Result<(), String> {
        match self {
            PackageManager::Apt(pm) => pm.refresh_cache().await,
            PackageManager::Pacman(pm) => pm.refresh_cache().await,
            PackageManager::CombinedParu(pacman, paru) => {
                // Refresh both
                pacman.refresh_cache().await?;
                paru.refresh_cache().await?;
                Ok(())
            }
            PackageManager::CombinedYay(pacman, yay) => {
                // Refresh both
                pacman.refresh_cache().await?;
                yay.refresh_cache().await?;
                Ok(())
            }
        }
    }
}

pub fn detect_package_manager() -> Option<PackageManager> {
    if std::process::Command::new("apt")
        .arg("--version")
        .output()
        .is_ok()
    {
        return Some(PackageManager::Apt(apt::AptPackageManager));
    }

    if std::process::Command::new("pacman")
        .arg("--version")
        .output()
        .is_ok()
    {
        // Check for AUR helpers in order of preference: paru > yay
        if std::process::Command::new("paru")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Some(PackageManager::CombinedParu(
                pacman::PacmanPackageManager,
                paru::ParuPackageManager,
            ));
        } else if std::process::Command::new("yay")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Some(PackageManager::CombinedYay(
                pacman::PacmanPackageManager,
                yay::YayPackageManager,
            ));
        } else {
            return Some(PackageManager::Pacman(pacman::PacmanPackageManager));
        }
    }

    None
}
