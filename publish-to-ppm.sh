#!/bin/bash
# Publish .deb package to APT repository on Codeberg Pages
# Uses the 'pages' branch in the same repository

set -e

VERSION=${1:-"0.3.0"}
PACKAGE_NAME="cosmic-updates"
ARCH="amd64"
DEB_FILE="${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"

# Check if .deb file exists
if [ ! -f "${DEB_FILE}" ]; then
    echo "❌ Error: ${DEB_FILE} not found!"
    echo "Run ./build-deb.sh ${VERSION} first to create the package."
    exit 1
fi

echo "📦 Publishing ${DEB_FILE} to APT repository..."
echo ""

# Copy .deb to /tmp so we can access it from pages branch
cp "${DEB_FILE}" "/tmp/${DEB_FILE}"

# Save current branch
CURRENT_BRANCH=$(git branch --show-current)

# Stash any uncommitted changes
git stash push -m "Auto-stash before switching to pages branch"

# Switch to pages branch
echo "🔀 Switching to pages branch..."
git checkout pages

# Create directory structure at ROOT level
mkdir -p pool/main
mkdir -p dists/stable/main/binary-${ARCH}

# Copy .deb from /tmp to pool
echo "📋 Copying ${DEB_FILE} to pool..."
cp "/tmp/${DEB_FILE}" "pool/main/"

# Generate Packages file
echo "📝 Generating Packages file..."
dpkg-scanpackages --arch "${ARCH}" pool/ > dists/stable/main/binary-${ARCH}/Packages
cat dists/stable/main/binary-${ARCH}/Packages | gzip -9 > dists/stable/main/binary-${ARCH}/Packages.gz

# Generate Release file with proper formatting
echo "📝 Generating Release file..."
cd dists/stable

cat > Release << EOF
Origin: VintageTechie
Label: COSMIC Updates
Suite: stable
Codename: stable
Architectures: ${ARCH}
Components: main
Description: Universal package update checker applet for COSMIC desktop
Date: $(date -Ru)
EOF

# Add MD5 checksums with proper format ( hash size filename)
echo "MD5Sum:" >> Release
find main -type f | while read file; do
    printf " %s %s %s\n" \
        "$(md5sum "$file" | cut -d' ' -f1)" \
        "$(stat -c%s "$file")" \
        "$file"
done >> Release

# Add SHA256 checksums with proper format
echo "SHA256:" >> Release
find main -type f | while read file; do
    printf " %s %s %s\n" \
        "$(sha256sum "$file" | cut -d' ' -f1)" \
        "$(stat -c%s "$file")" \
        "$file"
done >> Release

# Go back to repo root
cd ../..

echo ""
echo "✅ APT repository updated on pages branch!"
echo ""
echo "📊 Package info:"
dpkg-deb --info "pool/main/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
echo ""

# Show what we're committing
echo "📝 Changes to commit:"
git status --short

# Commit and push
echo ""
echo "💾 Committing changes..."
git add pool/ dists/ .gitignore
git commit -m "Release ${PACKAGE_NAME} version ${VERSION}"

echo ""
echo "🚀 Pushing to Codeberg Pages..."
git push origin pages

# Switch back to original branch
echo ""
echo "🔙 Switching back to ${CURRENT_BRANCH}..."
git checkout ${CURRENT_BRANCH}

# Restore any stashed changes
if git stash list | grep -q "Auto-stash before switching"; then
    git stash pop
fi

# Clean up temp file
rm -f "/tmp/${DEB_FILE}"

echo ""
echo "✅ Published successfully!"
echo ""
echo "📦 Package available at:"
echo "  https://vintagetechie.codeberg.page/cosmic-updates/pool/main/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
echo ""
echo "📄 Wait 5-10 minutes for Codeberg Pages to rebuild, then users can:"
echo "  sudo apt update"
echo "  sudo apt install cosmic-updates"
echo ""
echo "📋 Current version: ${VERSION}"
