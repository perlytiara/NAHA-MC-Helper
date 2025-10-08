# GitHub Release Tools for Minecraft Installer & Updater

This directory contains tools for managing GitHub releases for both **Minecraft Installer** and **Minecraft Updater**.

## 📁 Directory Structure

```text
releases/
├── generate-installer-release-notes.cjs    # Generate release notes for Minecraft Installer
├── generate-updater-release-notes.cjs      # Generate release notes for Minecraft Updater
├── manage-installer-releases.bat           # Manage releases for Installer (Windows)
├── manage-installer-releases.sh            # Manage releases for Installer (Linux/macOS)
├── manage-updater-releases.bat             # Manage releases for Updater (Windows)
├── manage-updater-releases.sh              # Manage releases for Updater (Linux/macOS)
├── notes/                                  # Generated release notes directory
│   ├── installer/                          # Installer release notes
│   └── updater/                            # Updater release notes
└── README.md                               # This file
```

## 🚀 Quick Start

### Generating Release Notes

#### For Minecraft Installer

```bash
# Generate release notes
node generate-installer-release-notes.cjs 1.1.2 "Multi-Launcher Support,Custom Path Installation,API Integration,Automodpack Setup"
```

#### For Minecraft Updater

```bash
# Generate release notes
node generate-updater-release-notes.cjs 1.0.0 "Auto-update detection,Cross-platform support,Improved error handling"
```

### Managing Releases

#### Managing Installer Releases

**Windows:**

```batch
# Delete all releases and tags
manage-installer-releases.bat delete-all

# Create a new release
manage-installer-releases.bat create 1.1.2 "Minecraft Installer v1.1.2" notes\installer\RELEASE_NOTES_INSTALLER_v1.1.2.md

# Reset everything and create a new release
manage-installer-releases.bat reset-and-create 1.1.2
```

**Linux/macOS:**

```bash
# Make script executable (first time only)
chmod +x manage-installer-releases.sh

# Delete all releases and tags
./manage-installer-releases.sh delete-all

# Create a new release
./manage-installer-releases.sh create 1.1.2 "Minecraft Installer v1.1.2" notes/installer/RELEASE_NOTES_INSTALLER_v1.1.2.md

# Reset everything and create a new release
./manage-installer-releases.sh reset-and-create 1.1.2
```

#### Managing Updater Releases (Alternative)

**Windows:**

```batch
# Delete all releases and tags
manage-updater-releases.bat delete-all

# Create a new release
manage-updater-releases.bat create 1.0.0 "Minecraft Updater v1.0.0" notes\updater\RELEASE_NOTES_UPDATER_v1.0.0.md

# Reset everything and create a new release
manage-updater-releases.bat reset-and-create 1.0.0
```

**Linux/macOS:**

```bash
# Make script executable (first time only)
chmod +x manage-updater-releases.sh

# Delete all releases and tags
./manage-updater-releases.sh delete-all

# Create a new release
./manage-updater-releases.sh create 1.0.0 "Minecraft Updater v1.0.0" notes/updater/RELEASE_NOTES_UPDATER_v1.0.0.md

# Reset everything and create a new release
./manage-updater-releases.sh reset-and-create 1.0.0
```

## 📦 Complete Release Workflow

### Complete Installer Release Workflow

1. **Build binaries for all platforms** (see `../../build-all.sh` or `../../build-all.bat`)

2. **Generate release notes:**

   ```bash
   node generate-installer-release-notes.cjs 1.1.2 "Feature 1,Feature 2,Bug Fix 3"
   ```

   This creates `notes/installer/RELEASE_NOTES_INSTALLER_v1.1.2.md`

3. **Create the GitHub release:**

   ```bash
   # Linux/macOS
   ./manage-installer-releases.sh create 1.1.2 "Minecraft Installer v1.1.2" notes/installer/RELEASE_NOTES_INSTALLER_v1.1.2.md

   # Windows
   manage-installer-releases.bat create 1.1.2 "Minecraft Installer v1.1.2" notes\installer\RELEASE_NOTES_INSTALLER_v1.1.2.md
   ```

4. **Upload binaries to the release** (manually or via `gh release upload`)

### Complete Updater Release Workflow

1. **Build binaries for all platforms**

2. **Generate release notes:**

   ```bash
   node generate-updater-release-notes.cjs 1.0.0 "Feature 1,Feature 2,Bug Fix 3"
   ```

   This creates `notes/updater/RELEASE_NOTES_UPDATER_v1.0.0.md`

3. **Create the GitHub release:**

   ```bash
   # Linux/macOS
   ./manage-updater-releases.sh create 1.0.0 "Minecraft Updater v1.0.0" notes/updater/RELEASE_NOTES_UPDATER_v1.0.0.md

   # Windows
   manage-updater-releases.bat create 1.0.0 "Minecraft Updater v1.0.0" notes\updater\RELEASE_NOTES_UPDATER_v1.0.0.md
   ```

4. **Upload binaries to the release**

## 📋 Release Note Format

The generated release notes include:

- 🎉 **Release title and version**
- 📦 **Download links** for all platforms (Windows, Linux, macOS)
- 🔐 **SHA256 checksums** for verification
- 📏 **File sizes** for each binary
- ✨ **Feature list**
- 🚀 **Quick start guide**
- 📂 **Source code links**
- 📋 **Version info** (version, date, repository)

### Example Output

```markdown
# Minecraft Installer v1.1.2

🎉 **New Release**

## 📦 Downloads

### 🖥️ Windows

- **[minecraft-installer-windows-x86_64.exe](...)** - Windows x64 (MSVC)
  Size: `9.89 MB` | SHA256: `e288a3...`
  ...
```

## 🔧 Platform-Specific Binaries

### Minecraft Installer

- `minecraft-installer-windows-x86_64.exe` - Windows (MSVC)
- `minecraft-installer-windows-gnu-x86_64.exe` - Windows (GNU)
- `minecraft-installer-linux-x86_64` - Linux x64
- `minecraft-installer-macos-intel-x86_64` - macOS Intel
- `minecraft-installer-macos-apple-silicon-aarch64` - macOS Apple Silicon

### Minecraft Updater

- `minecraft-updater-windows-x86_64.exe` - Windows (MSVC)
- `minecraft-updater-windows-gnu-x86_64.exe` - Windows (GNU)
- `minecraft-updater-linux-x86_64` - Linux x64
- `minecraft-updater-macos-intel-x86_64` - macOS Intel
- `minecraft-updater-macos-apple-silicon-aarch64` - macOS Apple Silicon

## 📝 Requirements

- **Node.js** (for running `.cjs` scripts)
- **GitHub CLI** (`gh`) - [Install here](https://cli.github.com/)
- **Git** (for tag management)
- **Proper permissions** to push tags and create releases on the respective repositories

## ⚠️ Important Notes

1. **Always build binaries before creating a release** - The scripts expect binaries in `../../dist/`
2. **SHA256 checksums are auto-generated** from files in the dist directory
3. **The `reset-and-create` command is destructive** - It deletes ALL existing releases and tags
4. **Test your release notes** before publishing
5. **Keep your GitHub CLI authenticated** - Run `gh auth login` if needed

## 🤝 Contributing

If you need to modify the release format:

1. Edit `generate-installer-release-notes.cjs` or `generate-updater-release-notes.cjs`
2. Update file paths in the management scripts if repository structure changes
3. Keep the GITHUB_REPO configuration updated

## 📞 Support

For issues related to:

- **Minecraft Installer**: <https://github.com/perlytiara/minecraft-installer>
- **Minecraft Updater**: <https://github.com/perlytiara/minecraft-updater>
