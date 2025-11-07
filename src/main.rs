//! COSMIC COSMIC Updates Applet
//!
//! This applet monitors APT package updates and displays them in the COSMIC panel.
//! It provides a visual indicator when updates are available and allows one-click upgrades.

use cosmic::app::{Core, Task};
use cosmic::iced::platform_specific::shell::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{Alignment, Length, Limits, Subscription};
use cosmic::iced_core::window;
use cosmic::iced_runtime::core::window::Id as WindowId;
use cosmic::{widget, Application, Element};
use std::process::Command as StdCommand;
use std::time::Duration;

// For loading SVG icons with colors
const ICON_NORMAL: &[u8] = include_bytes!("../icons/hicolor/scalable/apps/tux-normal.svg");
const ICON_ALERT: &[u8] = include_bytes!("../icons/hicolor/scalable/apps/tux-alert.svg");

/// Unique application identifier for the COSMIC desktop
const APP_ID: &str = "dev.vintagetechie.CosmicUpdates";

/// Application version
const VERSION: &str = "0.2.0";

/// Entry point for the applet
fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<UpdateChecker>(())
}

/// Represents a package that has an available update
#[derive(Debug, Clone)]
struct Package {
    /// Package name (e.g., "firefox")
    name: String,
    /// Currently installed version
    current_version: String,
    /// New version available for upgrade
    new_version: String,
}

/// Main application state for the APT checker applet
struct UpdateChecker {
    /// COSMIC core - provides applet infrastructure
    core: Core,
    /// Optional popup window ID (Some when popup is open)
    popup: Option<WindowId>,
    /// List of packages with available updates
    packages: Vec<Package>,
    /// True when checking for updates
    checking: bool,
    /// True when upgrade is in progress
    upgrading: bool,
    /// Error message if any operation failed
    error: Option<String>,
}

impl Default for UpdateChecker {
    fn default() -> Self {
        Self {
            core: Core::default(),
            popup: None,
            packages: Vec::new(),
            checking: false,
            upgrading: false,
            error: None,
        }
    }
}

/// Messages that the applet can send to itself
#[derive(Debug, Clone)]
enum Message {
    /// Toggle the popup window open/closed
    TogglePopup,
    /// Notification that a popup window was closed
    PopupClosed(WindowId),
    /// Request to check for available updates
    CheckForUpdates,
    /// Result of checking for updates (success with package list, or error)
    UpdatesFound(Result<Vec<Package>, String>),
    /// Request to start the upgrade process
    Upgrade,
    /// Result of starting the upgrade (success or error launching terminal)
    UpgradeStarted(Result<(), String>),
    /// Poll to check if upgrade is still running
    CheckUpgradeStatus,
    /// Result of checking upgrade status (true if still running)
    UpgradeStatusChecked(bool),
    /// Request to refresh the APT cache
    RefreshCache,
    /// Result of refreshing the APT cache
    CacheRefreshed(Result<(), String>),
    /// Periodic tick for scheduled update checks
    Tick,
}

impl Application for UpdateChecker {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initialize the applet and perform initial update check
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = UpdateChecker {
            core,
            ..Default::default()
        };
        // Check for updates immediately on startup
        (app, Task::done(cosmic::Action::App(Message::CheckForUpdates)))
    }

    /// Handle incoming messages and update application state
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    // Popup is open, close it
                    destroy_popup(p)
                } else {
                    // Popup is closed, open it
                    let new_id = WindowId::unique();
                    self.popup.replace(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );

                    // Set size constraints for the popup
                    popup_settings.positioner.size_limits = Limits::NONE
                        .min_width(300.0)
                        .min_height(100.0)
                        .max_width(400.0)
                        .max_height(600.0);

                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                // Clear popup reference if it matches the closed window
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
                Task::none()
            }
            Message::CheckForUpdates => {
                // Start checking for updates
                self.checking = true;
                self.error = None;
                Task::perform(
                    check_apt_updates(),
                    |result| cosmic::Action::App(Message::UpdatesFound(result))
                )
            }
            Message::UpdatesFound(result) => {
                // Process the result of checking for updates
                self.checking = false;
                match result {
                    Ok(packages) => {
                        self.packages = packages;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                Task::none()
            }
            Message::Upgrade => {
                // Start the upgrade process
                self.error = None;
                Task::perform(
                    run_apt_upgrade(),
                    |result| cosmic::Action::App(Message::UpgradeStarted(result))
                )
            }
            Message::UpgradeStarted(result) => {
                // Process the result of starting the upgrade
                match result {
                    Ok(()) => {
                        self.upgrading = true;
                        Task::none()
                    }
                    Err(e) => {
                        self.error = Some(e);
                        Task::none()
                    }
                }
            }
            Message::CheckUpgradeStatus => {
                // Poll to see if upgrade is still running
                Task::perform(
                    check_apt_running(),
                    |is_running| cosmic::Action::App(Message::UpgradeStatusChecked(is_running))
                )
            }
            Message::UpgradeStatusChecked(is_running) => {
                if !is_running && self.upgrading {
                    // Upgrade finished, refresh apt cache before checking for updates
                    self.upgrading = false;
                    self.update(Message::RefreshCache)
                } else {
                    Task::none()
                }
            }
            Message::RefreshCache => {
                // Refresh the APT package cache
                Task::perform(
                    refresh_apt_cache(),
                    |result| cosmic::Action::App(Message::CacheRefreshed(result))
                )
            }
            Message::CacheRefreshed(result) => {
                match result {
                    Ok(()) => {
                        // Cache refreshed, now check for updates
                        self.update(Message::CheckForUpdates)
                    }
                    Err(e) => {
                        self.error = Some(format!("Cache refresh error: {}", e));
                        Task::none()
                    }
                }
            }
            Message::Tick => {
                // Periodic check trigger (only if not currently upgrading)
                if !self.upgrading {
                    self.update(Message::CheckForUpdates)
                } else {
                    Task::none()
                }
            }
        }
    }

    /// Handle window close requests
    fn on_close_requested(&self, id: WindowId) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Set up subscriptions for periodic checks and upgrade polling
    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![
            // Check for updates every 30 minutes
            cosmic::iced::time::every(Duration::from_secs(30 * 60))
                .map(|_| Message::Tick)
        ];

        // If upgrading, poll every 2 seconds to check if apt is still running
        if self.upgrading {
            subscriptions.push(
                cosmic::iced::time::every(Duration::from_secs(2))
                    .map(|_| Message::CheckUpgradeStatus)
            );
        }

        Subscription::batch(subscriptions)
    }

    /// Render the panel icon view
    fn view(&self) -> Element<Self::Message> {
        let count = self.packages.len();
        let icon_data = if count > 0 {
            ICON_ALERT
        } else {
            ICON_NORMAL
        };
        let svg_handle = widget::svg::Handle::from_memory(icon_data);

        widget::container(
            widget::mouse_area(
                widget::svg(svg_handle)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
            )
            .on_press(Message::TogglePopup)
        )
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .into()
    }
    /// Render the popup window view
    fn view_window(&self, id: window::Id) -> Element<Self::Message> {
        // Only show content if this is our popup window
        if !matches!(self.popup, Some(p) if p == id) {
            return widget::text("").into();
        }

        let count = self.packages.len();
        
        // Choose icon for header
        let header_icon_data = if count > 0 { ICON_ALERT } else { ICON_NORMAL };
        let header_icon = widget::svg(widget::svg::Handle::from_memory(header_icon_data))
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(32.0));

        // Create header with icon and title
        let header = widget::row()
            .push(header_icon)
            .push(widget::horizontal_space())
            .push(
                widget::text("COSMIC Updates")
                    .size(18)
                    .width(Length::Fill)
            )
            .push(
                widget::text(format!("v{}", VERSION))
                    .size(12)
            )
            .spacing(12)
            .align_y(Alignment::Center);

        // Status section with colored indicators
        let status_content = if self.upgrading {
            widget::column()
                .push(
                    widget::text("⚙ Upgrading packages...")
                        .size(15)
                )
                .push(
                    widget::text("Check terminal for progress")
                        .size(12)
                )
                .spacing(4)
        } else if self.checking {
            widget::column()
                .push(
                    widget::text("🔄 Checking for updates...")
                        .size(15)
                )
                .spacing(4)
        } else if let Some(error) = &self.error {
            widget::column()
                .push(
                    widget::text(format!("❌ Error: {}", error))
                        .size(13)
                )
                .spacing(4)
        } else if count > 0 {
            let mut col = widget::column()
                .push(
                    widget::text(format!("⚠ {} update{} available", 
                        count, 
                        if count == 1 { "" } else { "s" }
                    ))
                    .size(15)
                )
                .spacing(6);

            // Add package cards with colored text using rich_text and Spans
            for package in &self.packages {
                use cosmic::iced::widget::text::Span;
                
                let package_text = cosmic::iced::widget::rich_text(vec![
                    Span::new(format!("📦 {}: ", package.name)),
                    Span::new(&package.current_version)
                        .color(cosmic::iced::Color::from_rgb(0.9, 0.3, 0.3)),
                    Span::new(" → "),
                    Span::new(&package.new_version)
                        .color(cosmic::iced::Color::from_rgb(0.3, 0.8, 0.3)),
                ])
                .size(12);
                
                let package_card = widget::container(package_text)
                    .padding(6);
                
                col = col.push(package_card);
            }
            col
        } else {
            widget::column()
                .push(
                    widget::text("✓ System is up to date")
                        .size(15)
                )
                .spacing(4)
        };

        // Buttons row
        let buttons = widget::row()
            .push(
                widget::button::standard("Check Now")
                    .on_press(Message::CheckForUpdates)
            )
            .push(widget::horizontal_space())
            .push(if count > 0 && !self.upgrading {
                widget::button::suggested("Upgrade")
                    .on_press(Message::Upgrade)
            } else {
                widget::button::suggested("Upgrade")
            })
            .spacing(12);

        // Wrap status content in scrollable with max height
        let scrollable_status = widget::scrollable(status_content)
            .height(Length::Fixed(400.0));

        let content = self.core.applet.popup_container(
            widget::column()
                .push(header)
                .push(scrollable_status)
                .push(buttons)
                .spacing(12)
                .padding(16)
                .align_x(Alignment::Start)
        );

        content.into()
    }
}

/// Check for available APT package updates
///
/// If DEBUG_APT_CHECKER environment variable is set, returns fake test data.
/// Otherwise, runs `apt list --upgradable` and parses the output.
///
/// # Returns
/// * `Ok(Vec<Package>)` - List of packages with available updates
/// * `Err(String)` - Error message if the check failed
async fn check_apt_updates() -> Result<Vec<Package>, String> {
    // Check for debug mode - returns fake data for testing
    if std::env::var("DEBUG_APT_CHECKER").is_ok() {
        return Ok(vec![
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
            Package {
                name: "vim".to_string(),
                current_version: "9.0.2048".to_string(),
                new_version: "9.1.0000".to_string(),
            },
            Package {
                name: "gcc".to_string(),
                current_version: "13.2.0-7".to_string(),
                new_version: "13.2.0-8".to_string(),
            },
            Package {
                name: "make".to_string(),
                current_version: "4.3-4.1".to_string(),
                new_version: "4.3-4.2".to_string(),
            },
            Package {
                name: "bash".to_string(),
                current_version: "5.2.21-2".to_string(),
                new_version: "5.2.21-3".to_string(),
            },
            Package {
                name: "openssh-client".to_string(),
                current_version: "9.6p1-3".to_string(),
                new_version: "9.7p1-1".to_string(),
            },
            Package {
                name: "wget".to_string(),
                current_version: "1.21.4-1".to_string(),
                new_version: "1.21.4-2".to_string(),
            },
            Package {
                name: "tar".to_string(),
                current_version: "1.35+dfsg-3".to_string(),
                new_version: "1.35+dfsg-4".to_string(),
            },
            Package {
                name: "gzip".to_string(),
                current_version: "1.12-1".to_string(),
                new_version: "1.13-1".to_string(),
            },
            Package {
                name: "perl".to_string(),
                current_version: "5.38.2-3".to_string(),
                new_version: "5.38.2-4".to_string(),
            },
            Package {
                name: "sqlite3".to_string(),
                current_version: "3.45.1-1".to_string(),
                new_version: "3.45.2-1".to_string(),
            },
            Package {
                name: "nodejs".to_string(),
                current_version: "20.11.0".to_string(),
                new_version: "20.12.0".to_string(),
            },
            Package {
                name: "nginx".to_string(),
                current_version: "1.24.0-2".to_string(),
                new_version: "1.25.0-1".to_string(),
            },
            Package {
                name: "postgresql-client".to_string(),
                current_version: "16.1-1".to_string(),
                new_version: "16.2-1".to_string(),
            },
            Package {
                name: "docker-ce".to_string(),
                current_version: "25.0.3".to_string(),
                new_version: "25.0.4".to_string(),
            },
            Package {
                name: "tmux".to_string(),
                current_version: "3.3a-6".to_string(),
                new_version: "3.4-1".to_string(),
            },
            Package {
                name: "htop".to_string(),
                current_version: "3.3.0-1".to_string(),
                new_version: "3.3.0-2".to_string(),
            },
            Package {
                name: "neofetch".to_string(),
                current_version: "7.1.0-4".to_string(),
                new_version: "7.1.0-5".to_string(),
            },
            Package {
                name: "zsh".to_string(),
                current_version: "5.9-5".to_string(),
                new_version: "5.9-6".to_string(),
            },
            Package {
                name: "rsync".to_string(),
                current_version: "3.2.7-1".to_string(),
                new_version: "3.3.0-1".to_string(),
            },
            Package {
                name: "grep".to_string(),
                current_version: "3.11-4".to_string(),
                new_version: "3.11-5".to_string(),
            },
            Package {
                name: "sed".to_string(),
                current_version: "4.9-2".to_string(),
                new_version: "4.9-3".to_string(),
            },
        ]);
    }

    // Run apt list --upgradable to get available updates
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

/// Parse the output of `apt list --upgradable`
///
/// Expected format per line:
/// `package/suite version arch [upgradable from: old_version]`
///
/// # Arguments
/// * `output` - Raw stdout from apt command
///
/// # Returns
/// Vector of Package structs with parsed information
fn parse_apt_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .skip(1) // Skip "Listing..." header line
        .filter_map(|line| {
            // Split line into parts by whitespace
            let parts: Vec<&str> = line.split_whitespace().collect();

            // Verify we have enough parts and the upgrade marker
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

/// Run APT upgrade in a terminal window
///
/// Launches cosmic-term with pkexec to elevate privileges and run apt upgrade.
/// The user can see the upgrade progress in the terminal window.
///
/// # Returns
/// * `Ok(())` - Terminal launched successfully
/// * `Err(String)` - Failed to launch terminal
async fn run_apt_upgrade() -> Result<(), String> {
    // Launch upgrade in terminal window so user can see progress
    StdCommand::new("cosmic-term")
        .args([
            "-e",
            "pkexec",
            "apt",
            "upgrade",
            "-y"
        ])
        .spawn()
        .map_err(|e| format!("Failed to launch terminal: {}", e))?;

    Ok(())
}

/// Check if APT is currently running by looking for lock files
///
/// APT creates lock files while running to prevent concurrent operations.
/// We check for the existence of these files to determine if an upgrade is in progress.
///
/// # Returns
/// `true` if any APT lock files exist (upgrade in progress), `false` otherwise
async fn check_apt_running() -> bool {
    std::path::Path::new("/var/lib/dpkg/lock-frontend").exists() ||
        std::path::Path::new("/var/lib/apt/lists/lock").exists() ||
        std::path::Path::new("/var/cache/apt/archives/lock").exists()
}

/// Refresh the APT package cache
///
/// Runs `apt update` with elevated privileges to fetch the latest package information
/// from repositories. This ensures we have the most current data about available updates.
///
/// # Returns
/// * `Ok(())` - Cache refreshed successfully
/// * `Err(String)` - Error message if refresh failed
async fn refresh_apt_cache() -> Result<(), String> {
    let output = StdCommand::new("pkexec")
        .args(["apt", "update"])
        .output()
        .map_err(|e| format!("Failed to refresh cache: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Cache refresh failed: {}", stderr));
    }

    Ok(())
}