# COSMIC Updates

A universal package update checker applet for COSMIC Desktop that supports multiple package managers with a custom penguin mascot.

## Features

- 🐧 Custom penguin icons that change when updates are available
- 📦 **Multi-distro support**: Works with APT (Debian/Ubuntu/Pop!_OS) and Pacman (Arch/Manjaro/CachyOS)
- 🎨 Color-coded version numbers (red for old, green for new)
- 📜 Scrollable package list for large updates
- ⚡ One-click upgrade with terminal progress
- 🔄 Auto-checks every 30 minutes
- 🔒 Secure privilege escalation with pkexec
- 🏷️ Version display in header
- 🔍 Auto-detects your package manager at runtime

## Supported Distributions

### APT-based:
- Pop!_OS ✅
- Ubuntu ✅
- Debian ✅
- Linux Mint ✅

### Pacman-based:
- CachyOS ✅
- Arch Linux ✅
- Manjaro ✅
- EndeavourOS ✅

## Installation

### Pop!_OS / Ubuntu / Debian

#### Option 1: APT Repository (Recommended)

Add the repository for automatic updates:
```bash
echo "deb [arch=amd64 trusted=yes] https://vintagetechie.codeberg.page/cosmic-updates stable main" | sudo tee /etc/apt/sources.list.d/cosmic-updates.list
sudo apt update
sudo apt install cosmic-updates
```

#### Option 2: Direct .deb Download

**[📥 Download cosmic-updates_0.3.0_amd64.deb](https://vintagetechie.codeberg.page/cosmic-updates/pool/main/cosmic-updates_0.3.0_amd64.deb)**
```bash
wget https://vintagetechie.codeberg.page/cosmic-updates/pool/main/cosmic-updates_0.3.0_amd64.deb
sudo apt install ./cosmic-updates_0.3.0_amd64.deb
```

### Arch Linux / CachyOS / Manjaro

#### Build from Source
```bash
git clone https://codeberg.org/VintageTechie/cosmic-updates.git
cd cosmic-updates
cargo install just
just install
just restart-panel
```

*AUR package coming soon!*

### Add to Panel

After installation:
- Right-click your COSMIC panel
- Select **Panel Configuration**
- Find **"Updates"** in the applets list
- Click to add it to your panel

That's it! 🎉

## Usage

- **Normal penguin** 🐧 = System is up to date
- **Alert penguin** 🐧⚠ = Updates available!
- Click the icon to see updates with **color-coded versions**
  - 🔴 **Red** = Current version (what you have)
  - 🟢 **Green** = New version (what's available)
- Scroll through long update lists
- Click "Upgrade" to install (opens terminal)
- Click "Check Now" for manual check

### Package Manager Detection

The applet automatically detects your system's package manager:
- **APT systems**: Uses `apt list --upgradable` and `apt upgrade`
- **Pacman systems**: Uses `checkupdates` and `pacman -Syu`

No configuration needed! 🎯

## Uninstalling

### APT-based systems:
```bash
sudo apt remove cosmic-updates
```

### Pacman-based systems:
```bash
just uninstall
```

## Building from Source

### Prerequisites
- Rust toolchain (latest stable)
- libcosmic development libraries
- COSMIC Desktop Environment

### Build Instructions
```bash
git clone https://codeberg.org/VintageTechie/cosmic-updates.git
cd cosmic-updates
cargo build --release
just install
```

### Development Commands (using just)
```bash
just build          # Build release binary
just install        # Install to system
just restart-panel  # Restart COSMIC panel
just debug          # Run with debug mode (fake packages)
just check          # Run cargo check
just lint           # Run cargo clippy
just fmt            # Format code
just clean          # Clean build artifacts
```

### Debug Mode
Test with fake packages:
```bash
DEBUG_APT_CHECKER=1 cosmic-panel
```

## Architecture

### Module Structure
```
src/
├── main.rs                    # Main applet code
└── package_manager/
    ├── mod.rs                 # Package manager trait & enum
    ├── apt.rs                 # APT implementation
    └── pacman.rs              # Pacman implementation
```

### Adding New Package Managers

To add support for a new package manager:

1. Create a new file in `src/package_manager/` (e.g., `dnf.rs`)
2. Implement these methods:
   - `check_updates()` - List available updates
   - `run_upgrade()` - Launch upgrade in terminal
   - `is_running()` - Check if package manager is running
   - `name()` - Return package manager name
3. Add detection in `mod.rs::detect_package_manager()`
4. Add variant to `PackageManager` enum

See `apt.rs` or `pacman.rs` for examples.

## Changelog

### Version 0.3.0 (2025-11-07)
- 🎉 **Multi-package manager support** - APT and Pacman!
- 🔄 **Renamed to cosmic-updates** - Universal support
- 🏗️ **Refactored architecture** - Modular design
- 🔍 **Auto-detection** - Detects package manager
- 🆔 **Updated APP_ID** - `com.vintagetechie.CosmicUpdates`
- 📦 **Arch/CachyOS support** - Full Pacman integration

### Version 0.2.0 (2025-11-06)
- 🎨 Color-coded version numbers
- 📜 Scrollable package list
- 🏷️ Version display in header
- 🎯 Fixed icon alignment

### Version 0.1.x (2025-11-05)
- Initial releases as cosmic-apt-checker
- Basic APT functionality

## Contributing

Contributions welcome! 

- **Bug reports**: Open an issue on [Codeberg](https://codeberg.org/VintageTechie/cosmic-updates/issues)
- **Code contributions**: Fork, create feature branch, test, submit PR
- **New package managers**: See "Adding New Package Managers" above

## License

MIT License - see [LICENSE](LICENSE)

## Credits

Developed by [VintageTechie](https://vintagetechie.com) for the COSMIC community 🚀

Built with [Rust](https://www.rust-lang.org/) 🦀 and [libcosmic](https://github.com/pop-os/libcosmic)

### Support Development

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/vintagetechie)

## Links

- **Website:** https://vintagetechie.com
- **Source:** https://codeberg.org/VintageTechie/cosmic-updates
- **APT Repo:** https://vintagetechie.codeberg.page/cosmic-updates
- **Issues:** https://codeberg.org/VintageTechie/cosmic-updates/issues

## Roadmap

### 🎯 Near-Term (v0.4.x)

#### AUR Support
- [ ] Detect AUR helpers (`yay`, `paru`, `pikaur`)
- [ ] Check AUR package updates separately
- [ ] Combine official repo + AUR updates in one view
- [ ] Show AUR packages with special indicator
- [ ] Handle AUR helper configuration

#### Package Distribution
- [ ] Publish AUR package (`cosmic-updates`)
- [ ] Publish AUR git package (`cosmic-updates-git`)
- [ ] RPM packages for Fedora
- [ ] openSUSE packages

### 🚀 Mid-Term (v0.5.x)

#### Additional Package Managers
- [ ] DNF support (Fedora, RHEL, CentOS)
- [ ] Zypper support (openSUSE, SUSE)
- [ ] Nix package manager support
- [ ] Snap support
- [ ] AppImage update checking

#### Flatpak Distribution
- [ ] Create Flatpak manifest
- [ ] Test with COSMIC Flatpak environment
- [ ] Submit to System76's COSMIC Flatpak repository
- [ ] Eventually submit to Flathub

### ✨ Long-Term Features

#### Smart Notifications
- [ ] Desktop notifications when updates available
- [ ] Configurable notification frequency
- [ ] System restart alerts (kernel/critical updates)
- [ ] Update categories (security vs regular)
- [ ] Silent mode option

#### Enhanced Update Management
- [ ] One-click upgrade (no terminal needed)
- [ ] Progress indicators during updates
- [ ] Update history/changelog viewer
- [ ] Failed update recovery
- [ ] Rollback capability
- [ ] Selective package updates (pick which to install)

#### Scheduling & Automation
- [ ] Configurable auto-check interval
- [ ] Schedule updates for specific times
- [ ] Auto-update option for security patches
- [ ] Update reminders

#### UI/UX Improvements
- [ ] Settings panel for configuration
- [ ] Multiple icon themes
- [ ] Package details popup (size, dependencies, changelog)
- [ ] Search/filter in update list
- [ ] Tooltips with package descriptions
- [ ] Keyboard shortcuts
- [ ] Different view modes (compact/detailed)

#### Advanced Features
- [ ] Update statistics and analytics
- [ ] Bandwidth usage monitoring
- [ ] Mirror selection/optimization
- [ ] Multiple language support (i18n)
- [ ] Integration with system logs
- [ ] Export update reports

---

Made with ❤️ for the COSMIC Desktop community
