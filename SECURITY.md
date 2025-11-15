# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability:

1. **DO NOT** open a public issue
2. Contact via GitHub private message or email (john@vintagetechie.com)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

Response time: Within 48 hours

## Security Considerations

This applet:
- Uses `pkexec` for privilege escalation (standard Linux security mechanism)
- Only runs `apt` commands (no arbitrary code execution)
- All icons and assets are verified and embedded
- No network connections except APT package manager

## Permissions Required

- Read: `/var/lib/dpkg/` (to check for running updates)
- Execute: `apt list --upgradable` (read-only package info)
- Elevated: `apt update` and `apt upgrade` (via pkexec authentication)
