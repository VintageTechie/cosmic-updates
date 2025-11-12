#!/bin/bash
# Build .deb package for cosmic-ext-applet-updates

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./build-deb.sh VERSION"
    echo "Example: ./build-deb.sh 1.1.0"
    exit 1
fi

PKGNAME="cosmic-ext-applet-updates"
ARCH="amd64"
PKGDIR="${PKGNAME}_${VERSION}_${ARCH}"

echo "Building ${PKGNAME} version ${VERSION}..."

# Build release binary
cargo build --release

# Create package structure
rm -rf "$PKGDIR"
mkdir -p "$PKGDIR/DEBIAN"
mkdir -p "$PKGDIR/usr/bin"
mkdir -p "$PKGDIR/usr/share/applications"
mkdir -p "$PKGDIR/usr/share/icons/hicolor/scalable/apps"

# Copy files
cp target/release/cosmic-ext-applet-updates "$PKGDIR/usr/bin/"
chmod +x "$PKGDIR/usr/bin/cosmic-ext-applet-updates"

cp com.vintagetechie.CosmicExtAppletUpdates.desktop "$PKGDIR/usr/share/applications/"
cp icons/hicolor/scalable/apps/tux-normal.svg "$PKGDIR/usr/share/icons/hicolor/scalable/apps/"
cp icons/hicolor/scalable/apps/tux-alert.svg "$PKGDIR/usr/share/icons/hicolor/scalable/apps/"

# Create control file
cat > "$PKGDIR/DEBIAN/control" << CONTROL
Package: cosmic-ext-applet-updates
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Depends: cosmic-session
Maintainer: VintageTechie <https://vintagetechie.com\>
Description: Updates Applet for COSMIC Desktop
 A community COSMIC Desktop applet that monitors package updates with support for
 multiple package managers including APT, Pacman, and AUR helpers.
 .
 Features:
  - Custom penguin icons that change when updates are available
  - Color-coded version numbers (red for old, green for new)
  - Desktop notifications for new updates
  - Configurable check intervals (5-120 minutes)
  - Settings UI with persistent configuration
  - Scrollable package list for large updates
  - One-click upgrade with terminal progress
  - Auto-detects your package manager at runtime
Homepage: https://github.com/VintageTechie/cosmic-ext-applet-updates
CONTROL

# Create postinst script
cat > "$PKGDIR/DEBIAN/postinst" << 'POSTINST'
#!/bin/bash
set -e
if [ "$1" = "configure" ]; then
    gtk-update-icon-cache -f /usr/share/icons/hicolor/ 2>/dev/null || true
fi
exit 0
POSTINST
chmod +x "$PKGDIR/DEBIAN/postinst"

# Build package
dpkg-deb --build "$PKGDIR"

echo ""
echo "✅ Package created: ${PKGDIR}.deb"
ls -lh "${PKGDIR}.deb"
