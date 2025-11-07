# Install paths
prefix := '/usr'
bin-dir := prefix / 'bin'
share-dir := prefix / 'share'
icons-dir := share-dir / 'icons/hicolor/scalable/apps'
applications-dir := share-dir / 'applications'

# Install the applet
install:
    install -Dm0755 target/release/cosmic-apt-checker {{bin-dir}}/cosmic-apt-checker
    install -Dm0644 dev.vintagetechie.CosmicAptChecker.desktop {{applications-dir}}/dev.vintagetechie.CosmicAptChecker.desktop
    install -Dm0644 icons/hicolor/scalable/apps/tux-normal.svg {{icons-dir}}/tux-normal.svg
    install -Dm0644 icons/hicolor/scalable/apps/tux-alert.svg {{icons-dir}}/tux-alert.svg
    gtk-update-icon-cache {{share-dir}}/icons/hicolor/ || true

# Uninstall the applet
uninstall:
    rm -f {{bin-dir}}/cosmic-apt-checker
    rm -f {{applications-dir}}/dev.vintagetechie.CosmicAptChecker.desktop
    rm -f {{icons-dir}}/tux-normal.svg
    rm -f {{icons-dir}}/tux-alert.svg
    gtk-update-icon-cache {{share-dir}}/icons/hicolor/ || true
