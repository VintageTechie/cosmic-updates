# Changelog

All notable changes to COSMIC Updates will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-11

### Added
- 🔐 GPG-signed APT repository for secure package installation
- 🔑 Automated GPG signing in publish workflow
- 📦 Public key distribution via GitHub Pages
- 🧊 Custom penguin icon in applet picker (fixed)

### Changed
- 📝 Updated installation instructions with GPG key import steps
- 🔒 Replaced `trusted=yes` with proper `signed-by=` directive in APT sources
- 🎨 Desktop file now uses `tux-normal` icon instead of generic system icon

### Fixed
- ✅ Eliminated all APT "Ign:" warnings with proper repository structure
- 🖼️ Applet picker now shows custom penguin icon instead of generic tools icon

### Security
- 🔐 APT repository now cryptographically signed with GPG
- 🔑 Users can verify package authenticity

## [0.4.0] - 2025-11-08

### Added
- 🎉 AUR Support - Automatic detection and integration of paru/yay helpers
- 🔵 [AUR] badges - Visual indicators for packages from Arch User Repository
- 📊 Separate counters - Shows "X official + Y AUR" package breakdown
- ⚙️ Settings UI - Configure update check intervals
- 💾 Config file - Persistent settings at `~/.config/cosmic-updates/config.toml`
- 🔧 Configurable check intervals (5, 10, 15, 20, 30, 45, 60, 90, 120 minutes)
- 📦 Published to AUR - `cosmic-updates-bin` and `cosmic-updates-git` packages

### Changed
- 🎨 Improved header showing active package manager name
- 🎯 Smart AUR helper detection with preference order: paru > yay > pacman-only
- 📋 Enhanced package list display with AUR indicators

## [0.3.0] - 2025-11-07

### Added
- 🎉 Multi-package manager support - APT and Pacman
- 🔍 Automatic package manager detection at runtime
- 📦 Full Arch Linux/CachyOS/Manjaro support via Pacman
- 🗂️ Modular architecture with package manager traits

### Changed
- 🔄 Renamed from `cosmic-apt-checker` to `cosmic-updates` for universal support
- 🆔 Updated APP_ID to `com.vintagetechie.CosmicUpdates`
- 🏗️ Refactored codebase with trait-based package manager system

### Technical
- 📁 New package_manager module structure:
  - `mod.rs` - Core traits and detection
  - `apt.rs` - APT implementation
  - `pacman.rs` - Pacman implementation
  - `yay.rs` - Yay AUR helper
  - `paru.rs` - Paru AUR helper

## [0.2.0] - 2025-11-06

### Added
- 🎨 Color-coded version numbers (red for current, green for new)
- 📜 Scrollable package list for handling many updates
- 🏷️ Version count display in header

### Fixed
- 🎯 Fixed icon alignment issues in UI

### Improved
- 💅 Better visual hierarchy in update list
- 🖱️ Improved user experience for large update lists

## [0.1.x] - 2025-11-05

### Added
- 🎉 Initial release as `cosmic-apt-checker`
- 🧊 Custom penguin icons (normal and alert states)
- 📦 APT package manager support for Pop!_OS/Ubuntu/Debian
- ⚡ One-click upgrade functionality
- 🔐 Secure privilege escalation with pkexec
- 🖱️ Manual "Check Now" button
- 🔄 Automatic update checking
- 📋 Package list display with versions
- 🪟 COSMIC panel applet integration

### Technical
- 🦀 Built with Rust and libcosmic
- 🎨 COSMIC Desktop Environment integration
- 🏗️ Desktop applet with hover popup

---

## Release Links

- **Source Code**: https://github.com/VintageTechie/cosmic-ext-applet-updates
- **APT Repository**: https://apt.vintagetechie.com
- **AUR Packages**:
  - Binary: https://aur.archlinux.org/packages/cosmic-ext-applet-updates-bin
  - Git: https://aur.archlinux.org/packages/cosmic-ext-applet-updates-git
- **Issues**: https://github.com/VintageTechie/cosmic-ext-applet-updates/issues

---

[1.0.0]: https://github.com/VintageTechie/cosmic-ext-applet-updates/releases/tag/v1.0.0
[0.4.0]: https://github.com/VintageTechie/cosmic-ext-applet-updates/releases/tag/v0.4.0
[0.3.0]: https://github.com/VintageTechie/cosmic-ext-applet-updates/releases/tag/v0.3.0
[0.2.0]: https://github.com/VintageTechie/cosmic-ext-applet-updates/releases/tag/v0.2.0
