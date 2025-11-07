pub mod apt;
pub mod pacman;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
}

#[derive(Clone)]
pub enum PackageManager {
    Apt(apt::AptPackageManager),
    Pacman(pacman::PacmanPackageManager),
}

impl PackageManager {
    pub async fn check_updates(&self) -> Result<Vec<Package>, String> {
        match self {
            PackageManager::Apt(pm) => pm.check_updates().await,
            PackageManager::Pacman(pm) => pm.check_updates().await,
        }
    }

    pub async fn run_upgrade(&self) -> Result<(), String> {
        match self {
            PackageManager::Apt(pm) => pm.run_upgrade().await,
            PackageManager::Pacman(pm) => pm.run_upgrade().await,
        }
    }

    pub async fn is_running(&self) -> bool {
        match self {
            PackageManager::Apt(pm) => pm.is_running().await,
            PackageManager::Pacman(pm) => pm.is_running().await,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PackageManager::Apt(pm) => pm.name(),
            PackageManager::Pacman(pm) => pm.name(),
        }
    }

    pub async fn refresh_cache(&self) -> Result<(), String> {
    match self {
        PackageManager::Apt(pm) => pm.refresh_cache().await,
        PackageManager::Pacman(pm) => pm.refresh_cache().await,
    }
}
}

pub fn detect_package_manager() -> Option<PackageManager> {
    if std::process::Command::new("apt").arg("--version").output().is_ok() {
        return Some(PackageManager::Apt(apt::AptPackageManager));
    }
    
    if std::process::Command::new("pacman").arg("--version").output().is_ok() {
        return Some(PackageManager::Pacman(pacman::PacmanPackageManager));
    }
    
    None
}