# TODO

## Immediate (Before Next Release)

- [ ] Add unit tests for package manager parsers
  - [ ] APT parser tests
  - [ ] Pacman parser tests
  - [ ] Paru parser tests
  - [ ] Yay parser tests
- [ ] Add integration tests for update detection
- [ ] Add tests for config file handling
- [ ] Add tests for state management

## Soon (Next Few Versions)

### Package Manager Support
- [ ] DNF/YUM support (Fedora, RHEL, Rocky Linux, AlmaLinux)
- [ ] Zypper support (openSUSE, SUSE)
- [ ] APK support (Alpine Linux)
- [ ] XBPS support (Void Linux)

### Features
- [ ] Configurable notification preferences
- [ ] Update history/changelog view
- [ ] Option to exclude specific packages from updates
- [ ] System tray notifications

## Future (Nice to Have)

- [ ] NixOS support (complex - requires architectural changes)
- [ ] Flatpak support
- [ ] Snap support
- [ ] AppImage update checking
- [ ] Custom update schedules (e.g., only check on weekdays)
- [ ] Bandwidth throttling for large updates
- [ ] Pre/post update hooks

## Documentation

- [ ] Add developer documentation
- [ ] Add contribution guidelines
- [ ] Create architecture overview
- [ ] Document package manager detection logic

## Infrastructure

- [ ] Set up CI/CD for automated testing
- [ ] Add automated release workflow
- [ ] Set up code coverage reporting
