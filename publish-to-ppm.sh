#!/bin/bash
# Publish cosmic-ext-applet-updates to APT repository (GitHub Pages)

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./publish-to-ppm.sh VERSION"
    echo "Example: ./publish-to-ppm.sh 1.1.0"
    exit 1
fi

PKGNAME="cosmic-ext-applet-updates"
DEB_FILE="${PKGNAME}_${VERSION}_amd64.deb"
GPG_KEY="26D2DE96ED9B7964D2502FFD2A456B067EE07248"

if [ ! -f "$DEB_FILE" ]; then
    echo "Error: $DEB_FILE not found!"
    echo "Run ./build-deb.sh $VERSION first"
    exit 1
fi

echo "Publishing ${PKGNAME} ${VERSION} to APT repository..."

cp "$DEB_FILE" /tmp/
git checkout pages

mkdir -p pool/main
mkdir -p dists/stable/main/binary-amd64

cp "/tmp/$DEB_FILE" pool/main/
dpkg-scanpackages --arch amd64 pool/ > dists/stable/main/binary-amd64/Packages
gzip -kf dists/stable/main/binary-amd64/Packages

cd dists/stable
cat > Release << RELEASE
Origin: VintageTechie
Label: cosmic-ext-applet-updates
Suite: stable
Codename: stable
Architectures: amd64
Components: main
Description: Updates Applet for COSMIC APT Repository
Date: $(date -R -u)
Valid-Until: $(date -R -u -d '+90 days')
RELEASE

echo "MD5Sum:" >> Release
find main -type f | while read file; do
    echo " $(md5sum "$file" | cut -d' ' -f1) $(wc -c < "$file") $file" >> Release
done

echo "SHA256:" >> Release
find main -type f | while read file; do
    echo " $(sha256sum "$file" | cut -d' ' -f1) $(wc -c < "$file") $file" >> Release
done

echo "SHA512:" >> Release
find main -type f | while read file; do
    echo " $(sha512sum "$file" | cut -d' ' -f1) $(wc -c < "$file") $file" >> Release
done

echo ""
echo "Signing Release file (you'll need to enter your GPG passphrase)..."
gpg --digest-algo SHA512 --default-key $GPG_KEY --armor --detach-sign --output Release.gpg Release
gpg --digest-algo SHA512 --default-key $GPG_KEY --armor --clearsign --output InRelease Release

cd ../..

git add pool/main/$DEB_FILE \
        dists/stable/Release \
        dists/stable/Release.gpg \
        dists/stable/InRelease \
        dists/stable/main/binary-amd64/Packages*
git commit -m "Publish cosmic-ext-applet-updates ${VERSION} with GPG signatures"
git push github pages

git checkout main
rm /tmp/$DEB_FILE

echo ""
echo "✅ Published to pages branch with GPG signatures!"
echo "⏰ Wait 5-15 minutes for GitHub Pages to rebuild"
echo "🔗 Check: https://apt.vintagetechie.com"
