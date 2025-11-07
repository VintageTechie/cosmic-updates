# COSMIC APT Update Checker

A COSMIC desktop applet for Pop!_OS that monitors APT package updates with a custom penguin mascot.

## Features

- 🐧 Custom penguin icons that change when updates are available
- 🎨 Color-coded version numbers (red for old, green for new)
- 📜 Scrollable package list for large updates
- 📦 Shows detailed list of available updates
- ⚡ One-click upgrade with terminal progress
- 🔄 Auto-checks every 30 minutes
- 🔒 Secure privilege escalation with pkexec
- 🏷️ Version display in header

## Installation (Pop!_OS)

### Option 1: APT Repository (Recommended)

Add the repository for automatic updates:
```bash
echo "deb [arch=amd64 trusted=yes] https://vintagetechie.codeberg.page/cosmic-apt-checker stable main" | sudo tee /etc/apt/sources.list.d/cosmic-apt-checker.list
sudo apt update
sudo apt install cosmic-apt-checker
```

### Option 2: Direct .deb Download

**[📥 Download cosmic-apt-checker_0.2.0_amd64.deb](https://vintagetechie.codeberg.page/cosmic-apt-checker/pool/main/cosmic-apt-checker_0.2.0_amd64.deb)**
```bash
wget https://vintagetechie.codeberg.page/cosmic-apt-checker/pool/main/cosmic-apt-checker_0.2.0_amd64.deb
sudo apt install ./cosmic-apt-checker_0.2.0_amd64.deb
```

### Add to Panel

After installation:
- Right-click your COSMIC panel
- Select **Panel Configuration**
- Find **"APT Update Checker"** in the applets list
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

## Uninstalling
```bash
sudo apt remove cosmic-apt-checker
```

## Building from Source (Optional)
```bash
# Clone the repository
git clone https://codeberg.org/VintageTechie/cosmic-apt-checker.git
cd cosmic-apt-checker

# Install just if you don't have it
cargo install just

# Build and install
cargo build --release
sudo just install
```

## Development

Built with Rust and [libcosmic](https://github.com/pop-os/libcosmic) specifically for Pop!_OS and COSMIC desktop.

**Debug mode:**
```bash
DEBUG_APT_CHECKER=1 cosmic-panel  # Shows 30 fake updates for testing scrollbar
```

## Changelog

### Version 0.2.0 (2025-11-06)
- 🎨 **Color-coded version numbers** - Old versions in red, new versions in green
- 📜 **Scrollable package list** - Handles 20+ updates gracefully with scrollbar
- 🏷️ **Version display** - Shows applet version in popup header
- 🎯 **Fixed icon alignment** - Icon now properly centered in panel
- 🧪 **Enhanced debug mode** - Now shows 30 test packages for thorough testing
- ✨ **Rich text support** - Using proper text spans for colored text

### Version 0.1.4 (2025-11-06)
- ✨ Enhanced popup UI with penguin icon in header
- 🎨 Added status indicator emojis (✓, ⚠, 🔄, ⚙, ❌)
- 📦 Package list now shows emoji indicators
- 🎯 Improved spacing and layout for compact, dynamic sizing
- 🛠 Fixed popup click handler for consistent behavior

### Version 0.1.3 (2025-11-05)
- 🎨 Improved icon coloring and design
- 🐧 Colored penguin mascot with alert badge
- 🔧 Bug fixes and performance improvements

### Version 0.1.2 (2025-11-05)
- 🔄 Enhanced update checking reliability
- 🛠 Various bug fixes

### Version 0.1.1 (2025-11-05)
- 🎉 Initial release
- 🐧 Custom penguin icon design
- ⚡ Basic update checking functionality

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

MIT License - see [LICENSE](LICENSE) file

## Credits

Developed by VintageTechie for the Pop!_OS / COSMIC community 🚀

Custom penguin artwork - original design, no copyright issues!

### Support Development

If you find this applet useful, consider supporting development:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/vintagetechie)

## Repository

- **Source Code:** https://codeberg.org/VintageTechie/cosmic-apt-checker
- **APT Repository:** https://vintagetechie.codeberg.page/cosmic-apt-checker
