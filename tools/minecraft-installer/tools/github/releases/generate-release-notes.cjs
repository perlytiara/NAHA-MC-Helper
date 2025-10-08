#!/usr/bin/env node
/* eslint-env node */

/**
 * Unified Release Notes Generator for Minecraft Installer & Updater
 * Generates a single release notes file for both executables
 */

const fs = require('fs');
const { execSync } = require('child_process');

function generateReleaseNotes() {
  const buildDate = new Date().toISOString().replace('T', ' ').split('.')[0] + ' UTC';
  const commitSha = execSync('git rev-parse --short HEAD').toString().trim();

  const releaseNotes = `# Minecraft Installer & Updater - Latest Build

**🔄 Auto-updating Release** | Built: ${buildDate} | Commit: ${commitSha}

This is an automatically updated release that always contains the **latest builds** from the main branch.

## 🎉 Dual-Executable Release

This release includes **two separate executables** for all major platforms:

1. **minecraft-installer** - Install modpacks to launchers
2. **minecraft-updater** - Scan and update existing instances

---

### 📦 Downloads

#### Minecraft Installer

**Windows:**
- \`minecraft-installer-windows-x86_64.exe\` - Windows x64 (MSVC, recommended)
- \`minecraft-installer-windows-gnu-x86_64.exe\` - Windows x64 (GNU)

**Linux:**
- \`minecraft-installer-linux-x86_64\` - Linux x64

**macOS:**
- \`minecraft-installer-macos-intel-x86_64\` - macOS Intel
- \`minecraft-installer-macos-apple-silicon-aarch64\` - macOS Apple Silicon (M1/M2/M3)

#### Minecraft Updater

**Windows:**
- \`minecraft-updater-windows-x86_64.exe\` - Windows x64 (MSVC, recommended)
- \`minecraft-updater-windows-gnu-x86_64.exe\` - Windows x64 (GNU)

**Linux:**
- \`minecraft-updater-linux-x86_64\` - Linux x64

**macOS:**
- \`minecraft-updater-macos-intel-x86_64\` - macOS Intel
- \`minecraft-updater-macos-apple-silicon-aarch64\` - macOS Apple Silicon (M1/M2/M3)

---

### ✨ Key Features

#### Minecraft Installer
- 🎮 **Multi-Launcher Support**: AstralRinth, ModrinthApp, XMCL, PrismLauncher, Official Minecraft, MultiMC
- 📁 **Custom Path Installation**: Install directly to any directory
- 🌐 **GitHub API Integration**: Download modpacks from GitHub Releases
- ⚙️ **Automodpack Setup**: Automatic server configuration
- 💾 **Database Integration**: Automatic database injection for AstralRinth/ModrinthApp

#### Minecraft Updater
- 🔍 **Instance Scanner**: Auto-detects all Minecraft instances across launchers
- 🧠 **Intelligent Mod Updates**: Compares versions and only updates what's needed
- 🗑️ **Duplicate Removal**: Automatically removes old mod versions
- 🔒 **User Mod Protection**: Preserves mods not in the modpack
- 🎯 **Version Selection**: Update to specific versions or latest
- 💾 **Database Sync**: Updates launcher databases with new version info
- 📊 **JSON Output**: Perfect for Electron app integration

---

### 🚀 Quick Start

#### Install a Modpack
\`\`\`bash
# From mrpack file
minecraft-installer --mrpack "modpack.mrpack" --create-instance

# From GitHub API (latest)
minecraft-installer --download-neoforge --create-instance

# To custom path
minecraft-installer --mrpack "modpack.mrpack" --target-launcher other --custom-path "C:\\Games\\Minecraft" --create-instance
\`\`\`

#### Update Existing Instances
\`\`\`bash
# Scan all instances
minecraft-updater scan --format compact

# Interactive update (select from list)
minecraft-updater interactive --modpack-type neoforge

# Update specific instance to latest
minecraft-updater update --instance-path "C:\\path\\to\\instance" --modpack-type neoforge

# Update to specific version
minecraft-updater update --instance-path "C:\\path\\to\\instance" --modpack-type neoforge --version 0.0.18

# Get JSON output for Electron apps
minecraft-updater scan --format json
\`\`\`

---

### 📋 Supported Launchers

| Launcher | Installer | Updater | Database Sync |
|----------|-----------|---------|---------------|
| AstralRinth | ✅ | ✅ | ✅ |
| ModrinthApp | ✅ | ✅ | ✅ |
| XMCL | ✅ | ✅ | ❌ |
| PrismLauncher | ✅ | ✅ | ❌ |
| Official Minecraft | ✅ | ✅ | ❌ |
| MultiMC | ✅ | ✅ | ❌ |
| Custom Path | ✅ | ❌ | ❌ |

---

### 🔒 Checksums

All binaries include SHA256 checksums (\`.sha256\` files) for verification.

---

### 📚 Documentation

- [Quick Start](https://github.com/perlytiara/NAHA-MC-Helper/blob/main/tools/minecraft-installer/QUICK_START.md)
- [Updater Guide](https://github.com/perlytiara/NAHA-MC-Helper/blob/main/tools/minecraft-installer/UPDATER_GUIDE.md)
- [Build Guide](https://github.com/perlytiara/NAHA-MC-Helper/blob/main/tools/minecraft-installer/BUILD_AND_RELEASE.md)

---

**Repository**: https://github.com/perlytiara/NAHA-MC-Helper

**Latest Commit**: ${commitSha}

---

> ⚠️ **Note**: This release is automatically rebuilt and updated on every push to main. Always download the latest version for the most recent features and fixes.
`;

  return releaseNotes;
}

// Main execution
if (require.main === module) {
  const releaseNotes = generateReleaseNotes();
  const outputFile = 'RELEASE_NOTES.md';
  
  fs.writeFileSync(outputFile, releaseNotes);
  // eslint-disable-next-line no-undef
  console.log(`✅ Release notes generated: ${outputFile}`);
  // eslint-disable-next-line no-undef
  console.log(releaseNotes);
}

module.exports = { generateReleaseNotes };

