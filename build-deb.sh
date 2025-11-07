#!/bin/bash
# Build .deb package for cosmic-updates

set -e  # Exit on error

VERSION=${1:-"0.3.0"}
PACKAGE_NAME="cosmic-updates"
ARCH="amd64"
DEB_NAME="${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"

echo "🔨 Building ${DEB_NAME}..."
echo ""

# Step 1: Build the release binary
echo "📦 Building release binary..."
cargo build --release
echo "✅ Binary built"
echo ""

# Step 2: Clean and prepare debian directory
echo "🗂️  Preparing package structure..."
rm -rf debian/usr debian/DEBIAN/postinst debian/DEBIAN/postrm
mkdir -p debian/usr/bin
mkdir -p debian/usr/share/applications
mkdir -p debian/usr/share/icons/hicolor/scalable/apps
mkdir -p debian/DEBIAN

# Step 3: Copy files to debian structure
echo "📋 Copying files..."
cp target/release/cosmic-updates debian/usr/bin/
chmod 0755 debian/usr/bin/cosmic-updates

cp com.vintagetechie.CosmicUpdates.desktop debian/usr/share/applications/
chmod 0644 debian/usr/share/applications/com.vintagetechie.CosmicUpdates.desktop

cp icons/hicolor/scalable/apps/tux-normal.svg debian/usr/share/icons/hicolor/scalable/apps/
cp icons/hicolor/scalable/apps/tux-alert.svg debian/usr/share/icons/hicolor/scalable/apps/
chmod 0644 debian/usr/share/icons/hicolor/scalable/apps/*.svg

echo "✅ Files copied"
echo ""

# Step 4: Update control file version
echo "📝 Updating control file..."
cat > debian/DEBIAN/control << EOF
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: VintageTechie <https://vintagetechie.com>
Description: Universal package update checker applet for COSMIC desktop
 A COSMIC desktop applet that monitors package updates from multiple
 package managers (APT, Pacman) with a custom penguin mascot.
 Shows updates in panel, allows one-click upgrades, and auto-checks
 every 30 minutes.
Depends: cosmic-panel
EOF

echo "✅ Control file updated"
echo ""

# Step 5: Create postinst script
echo "📝 Creating postinst script..."
cat > debian/DEBIAN/postinst << 'EOF'
#!/bin/bash
# Update icon cache after installation
if [ "$1" = "configure" ]; then
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
fi
exit 0
EOF
chmod 0755 debian/DEBIAN/postinst

# Step 6: Create postrm script
echo "📝 Creating postrm script..."
cat > debian/DEBIAN/postrm << 'EOF'
#!/bin/bash
# Update icon cache after removal
if [ "$1" = "remove" ] || [ "$1" = "purge" ]; then
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
fi
exit 0
EOF
chmod 0755 debian/DEBIAN/postrm

echo "✅ Maintainer scripts created"
echo ""

# Step 7: Build the .deb package
echo "🔧 Building .deb package..."
dpkg-deb --build --root-owner-group debian "${DEB_NAME}"

# Step 8: Move to root directory
mv "${DEB_NAME}" ./ 2>/dev/null || true

echo ""
echo "✅ Package built successfully: ${DEB_NAME}"
echo ""
echo "📊 Package info:"
dpkg-deb --info "${DEB_NAME}"
echo ""
echo "📦 Package contents:"
dpkg-deb --contents "${DEB_NAME}"
echo ""
echo "🎉 Ready to install or deploy!"
echo ""
echo "To test locally:"
echo "  sudo apt install ./${DEB_NAME}"
echo ""
echo "To remove package files from debian/ (keep the .deb):"
echo "  rm -rf debian/usr debian/DEBIAN/postinst debian/DEBIAN/postrm"
