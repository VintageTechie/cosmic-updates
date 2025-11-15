# Updates Applet for COSMIC

A universal package update checker applet for COSMIC Desktop that supports multiple package managers with AUR support, desktop notifications, and configurable settings.

![Version](https://img.shields.io/badge/version-1.1.2-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Linux-orange.svg)

## Features

- **Custom penguin mascot** - Changes when updates are available
- **Color-coded versions** - Red for old, green for new versions
- **Desktop notifications** - Alerts when new updates are available
- **Multi-package manager support**:
  - APT (Debian, Ubuntu, Pop!_OS)
  - Pacman (Arch, Manjaro, CachyOS)
  - AUR support via paru or yay
- **Configurable settings** - Check intervals, notifications, urgency threshold, terminal preference
- **Scrollable package list** - Handles large update lists
- **Auto-detection** - Detects your package manager and terminal automatically
- **One-click upgrades** - Terminal window for progress tracking
- **Persistent configuration** - Settings saved across sessions
- **AUR badges** - Visual distinction for AUR packages
- **Separate counters** - Shows official vs AUR update counts

## Installation

### Pop!_OS / Ubuntu / Debian

#### Option 1: APT Repository (Recommended)

Add the repository for automatic updates:

```bash
wget -qO - https://apt.vintagetechie.com/cosmic-ext-applet-updates-keyring.asc | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/cosmic-ext-applet-updates.gpg > /dev/null
echo "deb [signed-by=/etc/apt/trusted.gpg.d/cosmic-ext-applet-updates.gpg] https://apt.vintagetechie.com stable main" | sudo tee /etc/apt/sources.list.d/cosmic-ext-applet-updates.list
sudo apt update
sudo apt install cosmic-ext-applet-updates
```

#### Option 2: Direct .deb Download

**[Download cosmic-ext-applet-updates_1.1.2_amd64.deb](https://apt.vintagetechie.com/pool/main/cosmic-ext-applet-updates_1.1.2_amd64.deb)**

```bash
wget https://apt.vintagetechie.com/pool/main/cosmic-ext-applet-updates_1.1.2_amd64.deb
sudo apt install ./cosmic-ext-applet-updates_1.1.2_amd64.deb
```

### Arch Linux / CachyOS / Manjaro

#### Option 1: AUR Binary Package (Recommended)

```bash
paru -S cosmic-ext-applet-updates-bin
```

#### Option 2: AUR Source Package

```bash
paru -S cosmic-ext-applet-updates-git
```

### Building from Source

Requirements:
- Rust 1.70 or later
- libcosmic (via system packages or git)
- cargo
- pacman-contrib (for Arch-based systems)

```bash
git clone https://github.com/VintageTechie/cosmic-ext-applet-updates.git
cd cosmic-ext-applet-updates
cargo build --release
sudo cp target/release/cosmic-ext-applet-updates /usr/bin/
sudo cp com.vintagetechie.CosmicExtAppletUpdates.desktop /usr/share/applications/
sudo cp icons/hicolor/scalable/apps/tux-*.svg /usr/share/icons/hicolor/scalable/apps/
killall cosmic-panel && cosmic-panel &
```

## Usage

### Add to Panel

After installation:
- Open COSMIC Settings
- Go to **Desktop → Panel → Configure panel applets**
- Click **Add applet**
- Find **"Updates Applet for COSMIC"** in the applets list
- Click to add it to your panel

### Icons

- **Normal penguin** - System is up to date
- **Alert penguin** - Updates available

### Interface

Click the applet icon to:
- View available updates with color-coded versions
- See separate counts for official and AUR packages
- Click **Upgrade** to install updates (opens terminal)
- Click **Check Now** to manually refresh
- Access **Settings** to configure behavior

### Settings

Configure the applet behavior:
- **Check Interval**: Choose from 5 to 120 minutes (applies immediately)
- **Enable Notifications**: Toggle desktop notifications on/off
- **Urgency Threshold**: Set when notifications become urgent (default: 10 updates)
- **Terminal Preference**: Choose your preferred terminal or use auto-detection
- Settings are automatically saved to `~/.config/cosmic-ext-applet-updates/config.toml`

### Notifications

The applet sends desktop notifications when:
- Updates go from 0 to any number (new updates detected)
- The number of available updates increases

Notification urgency automatically escalates when update count exceeds your configured threshold.

## Package Manager Detection

The applet automatically detects your package manager:

| Distribution | Package Manager | AUR Support |
|--------------|----------------|-------------|
| Pop!_OS, Ubuntu, Debian | APT | No |
| Arch, Manjaro, CachyOS | Pacman | Yes (paru/yay) |

For Arch-based systems, AUR support preference order:
1. paru (if installed)
2. yay (if installed)
3. Pacman only (no AUR)

## Development

### Project Structure

```
cosmic-ext-applet-updates/
├── src/
│   ├── main.rs              # Main applet logic
│   ├── config.rs            # Settings management
│   ├── state.rs             # State tracking for notifications
│   └── package_manager/     # Package manager implementations
│       ├── mod.rs           # Trait definition
│       ├── apt.rs           # APT implementation
│       ├── pacman.rs        # Pacman implementation
│       ├── paru.rs          # Paru AUR helper
│       └── yay.rs           # Yay AUR helper
├── icons/                   # Penguin icons
├── build-deb.sh            # Build .deb package
├── publish-to-ppm.sh       # Publish to APT repository
└── justfile                # Build automation
```

### Adding New Package Managers

To add support for a new package manager:

1. Create a new module in `src/package_manager/` (e.g., `dnf.rs`)
2. Implement the `PackageManager` trait:
   ```rust
   pub trait PackageManager {
       fn check_updates(&self) -> Vec<PackageUpdate>;
       fn upgrade(&self);
   }
   ```
3. Add detection logic in `src/main.rs`
4. Test thoroughly on the target distribution

### Building Packages

**Debian package:**
```bash
./build-deb.sh 1.1.2
```

**Publish to APT repository:**
```bash
./publish-to-ppm.sh 1.1.2
```

## Configuration

Configuration file: `~/.config/cosmic-ext-applet-updates/config.toml`

```toml
check_interval_minutes = 30
enable_notifications = true
urgency_threshold = 10
```

**Note:** Configuration automatically migrates from the old `~/.config/cosmic-updates/` location if present.

## Migration from cosmic-updates

If you previously had `cosmic-updates` installed:

**APT (Pop!_OS/Ubuntu/Debian):**
```bash
sudo apt remove cosmic-updates
sudo rm /etc/apt/sources.list.d/cosmic-updates.list
sudo rm /etc/apt/trusted.gpg.d/cosmic-updates.gpg
```

Then follow the installation instructions above. Your settings will automatically migrate.

**AUR (Arch/CachyOS/Manjaro):**

The old `cosmic-updates-git` and `cosmic-updates-bin` packages are now transitional packages that automatically install the new versions. Simply update:

```bash
paru -Syu
```

Or explicitly install the new package:

```bash
paru -S cosmic-ext-applet-updates-git
```

## Uninstalling

**Pop!_OS / Ubuntu / Debian:**
```bash
sudo apt remove cosmic-ext-applet-updates
sudo rm /etc/apt/sources.list.d/cosmic-ext-applet-updates.list
```

**Arch / Manjaro / CachyOS:**
```bash
paru -R cosmic-ext-applet-updates-bin
```

## Changelog

### Version 1.1.2 (2025-11-14)
**Reliability & Usability Improvements**
- Fixed upgrade detection using process monitoring (pgrep)
- Settings now apply immediately without restart
- Configurable terminal emulator with auto-detection
- Improved parser robustness with comprehensive validation
- Fixed notification icon name
- Enhanced error logging for parser failures

### Version 1.1.0 (2025-11-12)
**Notifications & Namespace Rename**
- Desktop notifications for new updates
- Notification settings (enable/disable, urgency threshold)
- State tracking for intelligent notification triggers
- **BREAKING:** Renamed to `cosmic-ext-applet-updates` per System76 guidance
- Updated APP_ID to `com.vintagetechie.CosmicExtAppletUpdates`
- Config auto-migrates from `~/.config/cosmic-updates/`
- Display name: "Updates Applet for COSMIC"
- Repository moved to GitHub

### Version 1.0.0 (2025-11-11)
**Production Release**
- GPG-signed APT repository
- Migrated to GitHub Pages hosting
- Fixed applet picker icon (tux-normal)
- Comprehensive documentation
- Production-ready stability

### Version 0.4.0 (2025-11-09)
**AUR Support Release**
- Full AUR support via paru/yay
- Visual [AUR] badges for AUR packages
- Separate counters (official vs AUR)
- Settings UI with configurable check intervals
- Persistent configuration (TOML)
- Published to AUR

### Version 0.3.0 (2025-11-07)
**Universal Package Manager Support**
- Renamed to cosmic-updates
- Refactored architecture - modular design
- Auto-detection of package manager
- Updated APP_ID
- Arch/CachyOS support - Full Pacman integration

### Version 0.2.0 (2025-11-06)
**UI Enhancements**
- Color-coded version numbers (red/green)
- Scrollable package list
- Version display in header
- Fixed icon alignment

### Version 0.1.x (2025-11-05)
**Initial Release**
- Initial releases as cosmic-apt-checker
- Basic APT functionality
- Custom penguin icons
- One-click upgrades

## Contributing

Contributions are welcome! Here's how you can help:

### Bug Reports
Open an issue on [GitHub](https://github.com/VintageTechie/cosmic-ext-applet-updates/issues) with:
- Description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Your system info (distro, version, package manager)

### Code Contributions
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Test your changes thoroughly
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

### New Package Managers
See "Adding New Package Managers" in the Development section above.

## Security

For security issues, please see [SECURITY.md](SECURITY.md) or email packages@vintagetechie.com.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

Special thanks to Ben for his patience and understanding when I disappeared down the rabbit hole developing this project.

## Credits

Developed by [VintageTechie](https://vintagetechie.com) for the COSMIC community.

Built with:
- [Rust](https://www.rust-lang.org/)
- [libcosmic](https://github.com/pop-os/libcosmic)

## Links

- **Website**: https://vintagetechie.com
- **GitHub**: https://github.com/VintageTechie/cosmic-ext-applet-updates
- **APT Repository**: https://apt.vintagetechie.com
- **AUR Packages**: 
  - [cosmic-ext-applet-updates-bin](https://aur.archlinux.org/packages/cosmic-ext-applet-updates-bin)
  - [cosmic-ext-applet-updates-git](https://aur.archlinux.org/packages/cosmic-ext-applet-updates-git)
- **Issues**: https://github.com/VintageTechie/cosmic-ext-applet-updates/issues

## Roadmap

### Near-Term (v1.2.x)
- [ ] DNF support (Fedora)
- [ ] Zypper support (openSUSE)
- [ ] Flatpak update detection
- [ ] rpm-ostree support (Fedora Atomic)

### Mid-Term (v1.3.x)
- [ ] Update history viewer
- [ ] Update scheduling
- [ ] Selective package updates
- [ ] System restart notifications

### Long-Term (v2.0+)
- [ ] Enhanced analytics and statistics
- [ ] Update rollback support
- [ ] Automated update management
- [ ] Integration with COSMIC Store

---

Made for the COSMIC Desktop community
