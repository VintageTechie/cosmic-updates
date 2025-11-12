# Justfile for cosmic-ext-applet-updates
# Quick commands for building, installing, and managing the applet

# Default recipe - show available commands
default:
    @just --list

# Build release binary (as user)
build:
    cargo build --release

# Install to system (builds first, then uses sudo)
install: build
    sudo install -Dm0755 target/release/cosmic-ext-applet-updates /usr/bin/cosmic-ext-applet-updates
    sudo install -Dm0644 com.vintagetechie.CosmicExtAppletUpdates.desktop /usr/share/applications/com.vintagetechie.CosmicExtAppletUpdates.desktop
    sudo install -Dm0644 icons/hicolor/scalable/apps/tux-normal.svg /usr/share/icons/hicolor/scalable/apps/tux-normal.svg
    sudo install -Dm0644 icons/hicolor/scalable/apps/tux-alert.svg /usr/share/icons/hicolor/scalable/apps/tux-alert.svg
    sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true

# Uninstall from system
uninstall:
    sudo rm -f /usr/bin/cosmic-ext-applet-updates
    sudo rm -f /usr/share/applications/com.vintagetechie.CosmicExtAppletUpdates.desktop
    sudo rm -f /usr/share/icons/hicolor/scalable/apps/tux-normal.svg
    sudo rm -f /usr/share/icons/hicolor/scalable/apps/tux-alert.svg
    sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true

# Restart COSMIC panel
restart-panel:
    killall cosmic-panel || true
    cosmic-panel &

# Build and install in one step
deploy: install restart-panel

# Run with debug mode
debug: install
    killall cosmic-panel || true
    DEBUG_APT_CHECKER=1 cosmic-panel &

# Build .deb package (requires version number)
deb VERSION:
    ./build-deb.sh {{VERSION}}

# Publish to APT repo (requires version number)
publish VERSION:
    ./publish-to-ppm.sh {{VERSION}}

# Full release workflow
release VERSION: build
    ./build-deb.sh {{VERSION}}
    sudo apt install ./cosmic-ext-applet-updates_{{VERSION}}_amd64.deb
    just restart-panel

# Clean build artifacts
clean:
    cargo clean
    rm -rf debian/usr debian/DEBIAN/postinst debian/DEBIAN/postrm
    rm -f cosmic-ext-applet-updates_*.deb

# Run cargo check
check:
    cargo check

# Run cargo clippy
lint:
    cargo clippy

# Format code
fmt:
    cargo fmt
