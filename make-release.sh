#!/bin/bash
# Creates a release tarball for cosmic-updates

VERSION=${1:-1.0.0}
TARBALL="cosmic-updates-${VERSION}-x86_64.tar.gz"

echo "Building release ${VERSION} for cosmic-updates..."

# Build release binary
cargo build --release

# Create release directory
mkdir -p release/cosmic-updates
cp target/release/cosmic-updates release/cosmic-updates/
cp -r icons release/cosmic-updates/
cp com.vintagetechie.CosmicUpdates.desktop release/cosmic-updates/
cp LICENSE release/cosmic-updates/
cp README.md release/cosmic-updates/

# Create tarball
cd release
tar -czf "../${TARBALL}" cosmic-updates/
cd ..

echo ""
echo "✅ Release created: ${TARBALL}"
echo ""
echo "Calculate SHA256:"
sha256sum "${TARBALL}"
echo ""
echo "Next steps:"
echo "1. Create release on Codeberg: https://codeberg.org/VintageTechie/cosmic-updates/releases/new"
echo "2. Tag: v${VERSION}"
echo "3. Upload ${TARBALL}"
echo "4. Update AUR PKGBUILD with new SHA256"

# Cleanup
rm -rf release
