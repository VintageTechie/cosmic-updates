//! COSMIC COSMIC Updates Applet
//!
//! This applet monitors package updates and displays them in the COSMIC panel.
//! It provides a visual indicator when updates are available and allows one-click upgrades.
mod config;
mod package_manager;

use config::Config;
use cosmic::app::{Core, Task};
use cosmic::iced::platform_specific::shell::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{Alignment, Length, Limits, Subscription};
use cosmic::iced_core::window;
use cosmic::iced_runtime::core::window::Id as WindowId;
use cosmic::{widget, Application, Element};
use package_manager::{Package, PackageManager};
use std::time::Duration;

// For loading SVG icons with colors
const ICON_NORMAL: &[u8] = include_bytes!("../icons/hicolor/scalable/apps/tux-normal.svg");
const ICON_ALERT: &[u8] = include_bytes!("../icons/hicolor/scalable/apps/tux-alert.svg");

/// Unique application identifier for the COSMIC desktop
const APP_ID: &str = "com.vintagetechie.CosmicUpdates";

/// Application version
const VERSION: &str = "0.4.0";

/// Entry point for the applet
fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<UpdateChecker>(())
}

/// Main application state for the update checker applet
struct UpdateChecker {
    core: Core,
    popup: Option<WindowId>,
    packages: Vec<Package>,
    checking: bool,
    upgrading: bool,
    error: Option<String>,
    package_manager: PackageManager,
    config: Config,
    pending_config: Config,
    interval_options: Vec<String>,
    showing_settings: bool,
}

impl Default for UpdateChecker {
    fn default() -> Self {
        let package_manager =
            package_manager::detect_package_manager().expect("No supported package manager found");

        let config = Config::load();

        Self {
            core: Core::default(),
            popup: None,
            packages: Vec::new(),
            checking: false,
            upgrading: false,
            error: None,
            package_manager,
            config: config.clone(),
            pending_config: config,
            interval_options: vec![
                "5 minutes".to_string(),
                "10 minutes".to_string(),
                "15 minutes".to_string(),
                "20 minutes".to_string(),
                "30 minutes".to_string(),
                "45 minutes".to_string(),
                "60 minutes".to_string(),
                "90 minutes".to_string(),
                "120 minutes".to_string(),
            ],
            showing_settings: false,
        }
    }
}

/// Messages that the applet can send to itself
#[derive(Debug, Clone)]
enum Message {
    /// Toggle the popup window open/closed
    TogglePopup,
    /// Open settings view
    OpenSettings,
    /// Close settings view (back to main)
    CloseSettings,
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
    /// Request to refresh the cache
    RefreshCache,
    /// Result of refreshing the cache
    CacheRefreshed(Result<(), String>),
    /// Periodic tick for scheduled update checks
    Tick,
    /// Update check interval in settings
    SetCheckInterval(u64),
    /// Save settings
    SaveSettings,
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
        (
            app,
            Task::done(cosmic::Action::App(Message::CheckForUpdates)),
        )
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
            Message::OpenSettings => {
                // Reset pending config and show settings view
                self.pending_config = self.config.clone();
                self.showing_settings = true;
                Task::none()
            }
            Message::CloseSettings => {
                // Go back to main view
                self.showing_settings = false;
                Task::none()
            }
            Message::PopupClosed(id) => {
                // Clear popup reference if it matches the closed window
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                    self.showing_settings = false; // Reset settings view when popup closes
                }
                Task::none()
            }
            Message::CheckForUpdates => {
                // Start checking for updates
                self.checking = true;
                self.error = None;
                let pm = self.package_manager.clone();
                Task::perform(async move { pm.check_updates().await }, |result| {
                    cosmic::Action::App(Message::UpdatesFound(result))
                })
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
                let pm = self.package_manager.clone();
                Task::perform(async move { pm.run_upgrade().await }, |result| {
                    cosmic::Action::App(Message::UpgradeStarted(result))
                })
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
                let pm = self.package_manager.clone();
                Task::perform(async move { pm.is_running().await }, |is_running| {
                    cosmic::Action::App(Message::UpgradeStatusChecked(is_running))
                })
            }
            Message::UpgradeStatusChecked(is_running) => {
                if !is_running && self.upgrading {
                    // Upgrade finished, refresh cache before checking for updates
                    self.upgrading = false;
                    self.update(Message::RefreshCache)
                } else {
                    Task::none()
                }
            }
            Message::RefreshCache => {
                // Refresh the package cache
                let pm = self.package_manager.clone();
                Task::perform(async move { pm.refresh_cache().await }, |result| {
                    cosmic::Action::App(Message::CacheRefreshed(result))
                })
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
            Message::SetCheckInterval(minutes) => {
                self.pending_config.check_interval_minutes = minutes;
                Task::none()
            }
            Message::SaveSettings => {
                // Save config to file
                if let Err(e) = self.pending_config.save() {
                    self.error = Some(format!("Failed to save settings: {}", e));
                    Task::none()
                } else {
                    // Apply the new config
                    self.config = self.pending_config.clone();
                    // Go back to main view
                    self.showing_settings = false;
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
            // Check for updates based on config interval
            cosmic::iced::time::every(Duration::from_secs(self.config.check_interval_minutes * 60))
                .map(|_| Message::Tick),
        ];

        // If upgrading, poll every 2 seconds to check if still running
        if self.upgrading {
            subscriptions.push(
                cosmic::iced::time::every(Duration::from_secs(2))
                    .map(|_| Message::CheckUpgradeStatus),
            );
        }

        Subscription::batch(subscriptions)
    }

    /// Render the panel icon view
    fn view(&self) -> Element<Self::Message> {
        let count = self.packages.len();
        let icon_data = if count > 0 { ICON_ALERT } else { ICON_NORMAL };
        let svg_handle = widget::svg::Handle::from_memory(icon_data);

        widget::container(
            widget::mouse_area(
                widget::svg(svg_handle)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0)),
            )
            .on_press(Message::TogglePopup),
        )
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .into()
    }

    /// Render the popup window view
    fn view_window(&self, id: window::Id) -> Element<Self::Message> {
        // Main popup window
        if !matches!(self.popup, Some(p) if p == id) {
            return widget::text("").into();
        }

        // Show settings view or main view based on state
        if self.showing_settings {
            return self.settings_view();
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
            .push(widget::text("COSMIC Updates").size(18))
            .push(widget::horizontal_space())
            .push(
                widget::column()
                    .push(widget::text(format!("v{}", VERSION)).size(11))
                    .push(widget::text(self.package_manager.name()).size(10))
                    .spacing(2)
                    .align_x(Alignment::End),
            )
            .spacing(12)
            .align_y(Alignment::Center);

        // Status section with colored indicators
        let status_content = if self.upgrading {
            widget::column()
                .push(widget::text("⚙ Upgrading packages...").size(15))
                .push(widget::text("Check terminal for progress").size(12))
                .spacing(4)
        } else if self.checking {
            widget::column()
                .push(widget::text("🔄 Checking for updates...").size(15))
                .spacing(4)
        } else if let Some(error) = &self.error {
            widget::column()
                .push(widget::text(format!("❌ Error: {}", error)).size(13))
                .spacing(4)
        } else if count > 0 {
            let mut col = widget::column()
                .push({
                    // Count official and AUR packages separately
                    let aur_count = self.packages.iter().filter(|p| p.is_aur).count();
                    let official_count = count - aur_count;

                    let status_text = if aur_count > 0 && official_count > 0 {
                        format!(
                            "⚠ {} update{} available ({} official + {} AUR)",
                            count,
                            if count == 1 { "" } else { "s" },
                            official_count,
                            aur_count
                        )
                    } else if aur_count > 0 {
                        format!(
                            "⚠ {} AUR update{} available",
                            aur_count,
                            if aur_count == 1 { "" } else { "s" }
                        )
                    } else {
                        format!(
                            "⚠ {} update{} available",
                            count,
                            if count == 1 { "" } else { "s" }
                        )
                    };

                    widget::text(status_text).size(15)
                })
                .spacing(6);

            // Add package cards with colored text using rich_text and Spans
            for package in &self.packages {
                use cosmic::iced::widget::text::Span;

                // Build spans dynamically to include AUR badge
                let mut spans = vec![Span::new(format!("📦 {}: ", package.name))];

                // Add [AUR] badge if it's an AUR package
                if package.is_aur {
                    spans.push(
                        Span::new("[AUR] ").color(cosmic::iced::Color::from_rgb(0.4, 0.6, 1.0)), // Blue
                    );
                }

                // Add version information with colors
                spans.extend(vec![
                    Span::new(&package.current_version)
                        .color(cosmic::iced::Color::from_rgb(0.9, 0.3, 0.3)), // Red
                    Span::new(" → "),
                    Span::new(&package.new_version)
                        .color(cosmic::iced::Color::from_rgb(0.3, 0.8, 0.3)), // Green
                ]);

                let package_text = cosmic::iced::widget::rich_text(spans).size(12);

                let package_card = widget::container(package_text).padding(6);

                col = col.push(package_card);
            }
            col
        } else {
            widget::column()
                .push(widget::text("✓ System is up to date").size(15))
                .spacing(4)
        };

        // Buttons row
        let buttons = widget::row()
            .push(widget::button::standard("Settings").on_press(Message::OpenSettings))
            .push(widget::button::standard("Check Now").on_press(Message::CheckForUpdates))
            .push(widget::horizontal_space())
            .push(if count > 0 && !self.upgrading {
                widget::button::suggested("Upgrade").on_press(Message::Upgrade)
            } else {
                widget::button::suggested("Upgrade")
            })
            .spacing(12);

        // Wrap status content in scrollable with max height
        let scrollable_status = widget::scrollable(status_content).height(Length::Fixed(400.0));

        let content = self.core.applet.popup_container(
            widget::column()
                .push(header)
                .push(scrollable_status)
                .push(buttons)
                .spacing(12)
                .padding(16)
                .align_x(Alignment::Start),
        );

        content.into()
    }
}

impl UpdateChecker {
    /// Render the settings view
    fn settings_view(&self) -> Element<Message> {
        let header = widget::text("Settings").size(20);

        let current_interval = self.pending_config.check_interval_minutes;

        // Find the index of the current interval
        let current_index = match current_interval {
            5 => 0,
            10 => 1,
            15 => 2,
            20 => 3,
            30 => 4,
            45 => 5,
            60 => 6,
            90 => 7,
            120 => 8,
            _ => 4, // Default to 30 minutes
        };

        let interval_label = widget::text("Check for updates every:").size(14);

        let interval_dropdown =
            widget::dropdown(&self.interval_options, Some(current_index), |index| {
                // Convert index to minutes value
                let minutes = match index {
                    0 => 5,
                    1 => 10,
                    2 => 15,
                    3 => 20,
                    4 => 30,
                    5 => 45,
                    6 => 60,
                    7 => 90,
                    8 => 120,
                    _ => 30,
                };
                Message::SetCheckInterval(minutes)
            });

        let interval_row = widget::row()
            .push(interval_label)
            .push(widget::horizontal_space())
            .push(interval_dropdown)
            .spacing(12)
            .padding([8, 0]) // Add vertical padding for better spacing
            .align_y(Alignment::Center);

        // Buttons
        let buttons = widget::row()
            .push(widget::button::standard("Back").on_press(Message::CloseSettings))
            .push(widget::horizontal_space())
            .push(widget::button::suggested("Save").on_press(Message::SaveSettings))
            .spacing(12);

        let content = self.core.applet.popup_container(
            widget::column()
                .push(header)
                .push(widget::vertical_space().height(Length::Fixed(20.0)))
                .push(interval_row)
                .push(widget::vertical_space().height(Length::Fixed(20.0)))
                .push(buttons)
                .spacing(12)
                .padding(16),
        );

        content.into()
    }
}
