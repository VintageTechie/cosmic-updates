//! COSMIC COSMIC Updates Applet
//!
//! This applet monitors APT package updates and displays them in the COSMIC panel.
//! It provides a visual indicator when updates are available and allows one-click upgrades.
mod package_manager;
use cosmic::app::{Core, Task};
use cosmic::iced::platform_specific::shell::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{Alignment, Length, Limits, Subscription};
use cosmic::iced_core::window;
use cosmic::iced_runtime::core::window::Id as WindowId;
use cosmic::{widget, Application, Element};
use package_manager::{Package, PackageManager};
use std::process::Command as StdCommand;
use std::time::Duration;

// For loading SVG icons with colors
const ICON_NORMAL: &[u8] = include_bytes!("../icons/hicolor/scalable/apps/tux-normal.svg");
const ICON_ALERT: &[u8] = include_bytes!("../icons/hicolor/scalable/apps/tux-alert.svg");

/// Unique application identifier for the COSMIC desktop
const APP_ID: &str = "com.vintagetechie.CosmicUpdates";

/// Application version
const VERSION: &str = "0.2.0";

/// Entry point for the applet
fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<UpdateChecker>(())
}

/// Main application state for the APT checker applet
struct UpdateChecker {
    core: Core,
    popup: Option<WindowId>,
    packages: Vec<Package>,
    checking: bool,
    upgrading: bool,
    error: Option<String>,
    package_manager: PackageManager,
}

impl Default for UpdateChecker {
    fn default() -> Self {
        let package_manager =
            package_manager::detect_package_manager().expect("No supported package manager found");

        Self {
            core: Core::default(),
            popup: None,
            packages: Vec::new(),
            checking: false,
            upgrading: false,
            error: None,
            package_manager,
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
                    // Upgrade finished, refresh apt cache before checking for updates
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
            cosmic::iced::time::every(Duration::from_secs(30 * 60)).map(|_| Message::Tick),
        ];

        // If upgrading, poll every 2 seconds to check if apt is still running
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
            .push(widget::text("COSMIC Updates").size(18).width(Length::Fill))
            .push(widget::text(format!("v{}", VERSION)).size(12))
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
                .push(
                    widget::text(format!(
                        "⚠ {} update{} available",
                        count,
                        if count == 1 { "" } else { "s" }
                    ))
                    .size(15),
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
