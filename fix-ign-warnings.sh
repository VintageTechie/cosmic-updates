#!/bin/bash
# Quick fix: Add stub files to existing repository to eliminate "Ign" warnings
# Run this ONCE on your pages branch

set -e

echo "🔧 Adding stub files to eliminate APT 'Ign' warnings..."
echo ""

# Check we're on pages branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "pages" ]; then
    echo "❌ Error: Must be run on pages branch"
    echo "   Current branch: $BRANCH"
    echo "   Run: git checkout pages"
    exit 1
fi

# Check directory structure exists
if [ ! -d "dists/stable/main/binary-amd64" ]; then
    echo "❌ Error: Repository structure not found"
    echo "   Expected: dists/stable/main/binary-amd64/"
    exit 1
fi

echo "📝 Creating stub files..."

# Translation stubs
mkdir -p "dists/stable/main/i18n"
touch "dists/stable/main/i18n/Translation-en"
gzip -9c "dists/stable/main/i18n/Translation-en" > "dists/stable/main/i18n/Translation-en.gz"
echo "✅ Translation-en"

# Components stub  
echo "" > "dists/stable/main/binary-amd64/Components"
gzip -9c "dists/stable/main/binary-amd64/Components" > "dists/stable/main/binary-amd64/Components.gz"
echo "✅ Components"

# command-not-found stub
echo "" > "dists/stable/main/binary-amd64/cnf-commands"  
gzip -9c "dists/stable/main/binary-amd64/cnf-commands" > "dists/stable/main/binary-amd64/cnf-commands.gz"
echo "✅ cnf-commands"

echo ""
echo "📋 Regenerating Release file with new checksums..."
cd dists/stable

cat > Release << EOF
Origin: VintageTechie
Label: COSMIC Updates
Suite: stable
Codename: stable
Architectures: amd64
Components: main
Description: APT update checker applet for COSMIC desktop
Date: $(date -Ru)
EOF

# Generate MD5Sum section
echo "MD5Sum:" >> Release
for file in main/binary-amd64/Packages main/binary-amd64/Packages.gz main/i18n/Translation-en main/i18n/Translation-en.gz main/binary-amd64/Components main/binary-amd64/Components.gz main/binary-amd64/cnf-commands main/binary-amd64/cnf-commands.gz; do
    if [ -f "$file" ]; then
        size=$(stat -c%s "$file")
        md5=$(md5sum "$file" | cut -d' ' -f1)
        printf " %s %7d %s\n" "$md5" "$size" "$file" >> Release
    fi
done

# Generate SHA256 section
echo "SHA256:" >> Release
for file in main/binary-amd64/Packages main/binary-amd64/Packages.gz main/i18n/Translation-en main/i18n/Translation-en.gz main/binary-amd64/Components main/binary-amd64/Components.gz main/binary-amd64/cnf-commands main/binary-amd64/cnf-commands.gz; do
    if [ -f "$file" ]; then
        size=$(stat -c%s "$file")
        sha256=$(sha256sum "$file" | cut -d' ' -f1)
        printf " %s %7d %s\n" "$sha256" "$size" "$file" >> Release
    fi
done

cd ../..

echo ""
echo "✅ Stub files created and Release updated!"
echo ""
echo "📊 New files:"
ls -lh dists/stable/main/i18n/
ls -lh dists/stable/main/binary-amd64/ | grep -E "(Components|cnf)"
echo ""
echo "📤 Now commit and push:"
echo "   git add dists/"
echo "   git commit -m 'Add stub files to eliminate APT Ign warnings'"
echo "   git push origin pages"
echo ""
echo "After pushing, 'Ign' warnings will be gone! 🎉"
