# GitHub Release Tools

## Unified Release Notes Generator

This directory contains a single unified release notes generator for both **Minecraft Installer** and **Minecraft Updater**.

### Usage

```bash
node generate-release-notes.cjs
```

This generates `RELEASE_NOTES.md` with:
- Current build date
- Latest commit SHA
- Download links for all platforms (both installer and updater)
- Feature descriptions
- Usage examples
- Documentation links

### Continuous Build System

The GitHub Actions workflow (`.github/workflows/build.yml`) automatically:
1. Builds both executables for all platforms on every push to main
2. Generates release notes using this script
3. Deletes the previous "latest" release
4. Creates a new "latest" release with all binaries

### Manual Generation

To preview release notes locally:

```bash
cd tools/github/releases
node generate-release-notes.cjs
cat RELEASE_NOTES.md
```

---

**No manual release management needed** - Everything is automated! 🚀
