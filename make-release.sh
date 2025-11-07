#!/bin/bash
# Creates a release for Pop!_OS users

VERSION=${1:-0.1.0}
TARBALL="cosmic-apt-checker-${VERSION}-pop-os.tar.gz"

echo "Building release ${VERSION} for Pop!_OS..."

# Build release binary
cargo build --release

# Create release directory
mkdir -p release/cosmic-apt-checker
cp target/release/cosmic-apt-checker release/cosmic-apt-checker/
cp -r icons release/cosmic-apt-checker/
cp dev.vintagetechie.CosmicAptChecker.desktop release/cosmic-apt-checker/
cp LICENSE release/cosmic-apt-checker/
cp README.md release/cosmic-apt-checker/

# Create simple install script
cat > release/cosmic-apt-checker/install.sh << 'INSTALL'
#!/bin/bash
echo "Installing COSMIC APT Checker for Pop!_OS..."
sudo install -Dm0755 cosmic-apt-checker /usr/bin/cosmic-apt-checker
sudo install -Dm0644 dev.vintagetechie.CosmicAptChecker.desktop /usr/share/applications/dev.vintagetechie.CosmicAptChecker.desktop
sudo install -Dm0644 icons/hicolor/scalable/apps/tux-normal.svg /usr/share/icons/hicolor/scalable/apps/tux-normal.svg
sudo install -Dm0644 icons/hicolor/scalable/apps/tux-alert.svg /usr/share/icons/hicolor/scalable/apps/tux-alert.svg
echo "Updating icon cache..."
sudo gtk-update-icon-cache /usr/share/icons/hicolor/ 2>/dev/null || true
echo ""
echo "✅ Installation complete!"
echo ""
echo "To use the applet:"
echo "1. Restart COSMIC panel: killall cosmic-panel && cosmic-panel &"
echo "2. Right-click panel → Panel Configuration"
echo "3. Add 'APT Update Checker' to your panel"
echo ""
INSTALL

chmod +x release/cosmic-apt-checker/install.sh

# Create tarball
cd release
tar -czf "../${TARBALL}" cosmic-apt-checker/
cd ..

echo ""
echo "✅ Release created: ${TARBALL}"
echo ""
echo "Next steps:"
echo "1. Test the installation locally"
echo "2. Create release on Codeberg"
echo "3. Upload ${TARBALL}"

# Cleanup
rm -rf release
