#!/bin/bash
# Publish cosmic-updates to APT repository (Codeberg Pages)

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./publish-to-ppm.sh VERSION"
    echo "Example: ./publish-to-ppm.sh 0.4.0"
    exit 1
fi

PKGNAME="cosmic-updates"
DEB_FILE="${PKGNAME}_${VERSION}_amd64.deb"
GPG_KEY="26D2DE96ED9B7964D2502FFD2A456B067EE07248"

if [ ! -f "$DEB_FILE" ]; then
    echo "Error: $DEB_FILE not found!"
    echo "Run ./build-deb.sh $VERSION first"
    exit 1
fi

echo "Publishing ${PKGNAME} ${VERSION} to APT repository..."

# Copy deb to temp
cp "$DEB_FILE" /tmp/

# Switch to pages branch
git checkout pages

# Create structure
mkdir -p pool/main
mkdir -p dists/stable/main/binary-amd64

# Copy deb to pool
cp "/tmp/$DEB_FILE" pool/main/

# Generate Packages file (run from repo root, not from subdirectory)
dpkg-scanpackages --arch amd64 pool/ > dists/stable/main/binary-amd64/Packages
gzip -kf dists/stable/main/binary-amd64/Packages

# Generate Release file
cd dists/stable
cat > Release << RELEASE
Origin: VintageTechie
Label: cosmic-updates
Suite: stable
Codename: stable
Architectures: amd64
Components: main
Description: COSMIC Updates APT Repository
RELEASE

# Add checksums
echo "MD5Sum:" >> Release
find main -type f | while read file; do
    md5sum "$file" | awk '{print " " $1 " " $2}' >> Release
done

echo "SHA256:" >> Release
find main -type f | while read file; do
    sha256sum "$file" | awk '{print " " $1 " " $2}' >> Release
done

# Sign the Release file
echo ""
echo "Signing Release file (you'll need to enter your GPG passphrase)..."
gpg --default-key $GPG_KEY --armor --detach-sign --output Release.gpg Release
gpg --default-key $GPG_KEY --armor --clearsign --output InRelease Release

cd ../..

# Commit and push (now includes signature files)
git add pool/main/$DEB_FILE \
        dists/stable/Release \
        dists/stable/Release.gpg \
        dists/stable/InRelease \
        dists/stable/main/binary-amd64/Packages*
git commit -m "Publish cosmic-updates ${VERSION} with GPG signatures"
git push origin pages

# Switch back to main
git checkout master

# Clean up
rm /tmp/$DEB_FILE

echo ""
echo "✅ Published to pages branch with GPG signatures!"
echo "⏰ Wait 5-15 minutes for Codeberg Pages to rebuild"
echo "🔗 Check: https://vintagetechie.codeberg.page/cosmic-updates/"
