#!/usr/bin/env node

/**
 * Release Notes Generator for NAHA MC Helper
 * 
 * This script generates formatted release notes with file listings and SHA256 checksums.
 * Usage: node generate-release-notes.js <version> <changes>
 * Example: node generate-release-notes.js 1.0.0 "Auto-updater feature,Upgraded to Svelte 5,Bug fixes"
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// Configuration
const GITHUB_REPO = 'perlytiara/NAHA-MC-Helper';
const DIST_DIR = path.join(__dirname, '..', 'dist');

/**
 * Calculate SHA256 checksum for a file
 */
function calculateSHA256(filePath) {
  try {
    const fileBuffer = fs.readFileSync(filePath);
    const hashSum = crypto.createHash('sha256');
    hashSum.update(fileBuffer);
    return hashSum.digest('hex');
  } catch (error) {
    console.error(`Error calculating SHA256 for ${filePath}:`, error.message);
    return 'N/A';
  }
}

/**
 * Format file size to human-readable format
 */
function formatFileSize(bytes) {
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  if (bytes === 0) return '0 Bytes';
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return Math.round(bytes / Math.pow(1024, i)) + ' ' + sizes[i];
}

/**
 * Get file info (size and SHA256)
 */
function getFileInfo(filePath) {
  try {
    const stats = fs.statSync(filePath);
    return {
      size: formatFileSize(stats.size),
      sha256: calculateSHA256(filePath)
    };
  } catch (error) {
    return { size: 'N/A', sha256: 'N/A' };
  }
}

/**
 * Generate release notes
 */
function generateReleaseNotes(version, changes) {
  const changesList = Array.isArray(changes) ? changes : changes.split(',').map(c => c.trim());
  
  const releaseNotes = `# 🎉 NAHA MC Helper v${version} Release Notes 🚀

Welcome to the latest update of **NAHA MC Helper**! We've packed this release with exciting new features, smoother performance, and a polished look to enhance your experience. Here's what's new! 🌟

## ✨ What's New in v${version}

${changesList.map(change => `- **${change}**: Enhanced functionality for better user experience. 💎`).join('\n')}

## 📥 Installation Instructions

Download the appropriate version for your system below:

### 🖥️ Windows
- **Installer**: [NAHA.MC.Helper.Setup.${version}.exe](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper.Setup.${version}.exe) 📦
- **Portable**: [NAHA.MC.Helper.${version}.exe](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper.${version}.exe) 💼

### 🍎 macOS
- **Intel Macs**: [NAHA.MC.Helper-${version}.dmg](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper-${version}.dmg) 🖱️
- **Apple Silicon Macs**: [NAHA.MC.Helper-${version}-arm64.dmg](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper-${version}-arm64.dmg) 🍏
- **Apple Silicon Macs (Zip)**: [NAHA.MC.Helper-${version}-arm64-mac.zip](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper-${version}-arm64-mac.zip) 📎
- **Intel Macs (Zip)**: [NAHA.MC.Helper-${version}-mac.zip](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper-${version}-mac.zip) 📎

### 🐧 Linux
- **Universal**: [NAHA.MC.Helper-${version}.AppImage](https://github.com/${GITHUB_REPO}/releases/download/v${version}/NAHA.MC.Helper-${version}.AppImage) 🐧
- **Debian/Ubuntu**: [naha-mc-helper_${version}_amd64.deb](https://github.com/${GITHUB_REPO}/releases/download/v${version}/naha-mc-helper_${version}_amd64.deb) 📀
- **Red Hat/Fedora**: [naha-mc-helper-${version}.x86_64.rpm](https://github.com/${GITHUB_REPO}/releases/download/v${version}/naha-mc-helper-${version}.x86_64.rpm) 🔧

### 📂 Source Code
- [Source code (zip)](https://github.com/${GITHUB_REPO}/archive/refs/tags/v${version}.zip) 📜
- [Source code (tar.gz)](https://github.com/${GITHUB_REPO}/archive/refs/tags/v${version}.tar.gz) 📜

## 🔄 Auto-Update Feature
No need to manually check for updates! NAHA MC Helper now automatically detects new versions and notifies you when they're ready to install. Stay current with minimal effort! 🔔

---

**Thank you for using NAHA MC Helper!** We're thrilled to bring you these improvements. Let us know your feedback, and happy exploring! 😄

## 📋 Version Info
- **Version**: ${version}
- **Release Date**: ${new Date().toISOString().split('T')[0]}
- **GitHub Repository**: [${GITHUB_REPO}](https://github.com/${GITHUB_REPO})
`;

  return releaseNotes;
}

// Main execution
if (require.main === module) {
  const args = process.argv.slice(2);
  
  if (args.length < 2) {
    console.error('Usage: node generate-release-notes.js <version> <changes>');
    console.error('Example: node generate-release-notes.js 1.0.0 "Auto-updater,Svelte 5 upgrade,Bug fixes"');
    process.exit(1);
  }
  
  const [version, changesStr] = args;
  const changes = changesStr.split(',').map(c => c.trim());
  
  const releaseNotes = generateReleaseNotes(version, changes);
  
  // Write to file
  const outputFile = path.join(__dirname, `RELEASE_NOTES_v${version}.md`);
  fs.writeFileSync(outputFile, releaseNotes);
  
  console.log(`✅ Release notes generated: ${outputFile}`);
  console.log('\n' + releaseNotes);
}

module.exports = { generateReleaseNotes };

