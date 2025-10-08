#!/usr/bin/env node

/**
 * Release Notes Generator for Minecraft Updater
 *
 * This script generates formatted release notes with file listings and SHA256 checksums.
 * Usage: node generate-updater-release-notes.cjs <version> <changes>
 * Example: node generate-updater-release-notes.cjs 1.0.0 "Auto-update detection,Cross-platform support,Improved error handling"
 */

const fs = require('fs')
const path = require('path')
const crypto = require('crypto')

// Configuration
const GITHUB_REPO = 'perlytiara/minecraft-updater'
const DIST_DIR = path.join(__dirname, '..', '..', '..', 'dist')

/**
 * Calculate SHA256 checksum for a file
 */
function calculateSHA256(filePath) {
  try {
    const fileBuffer = fs.readFileSync(filePath)
    const hashSum = crypto.createHash('sha256')
    hashSum.update(fileBuffer)
    return hashSum.digest('hex')
  } catch (error) {
    console.error(`Error calculating SHA256 for ${filePath}:`, error.message)
    return 'N/A'
  }
}

/**
 * Format file size to human-readable format
 */
function formatFileSize(bytes) {
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  if (bytes === 0) return '0 Bytes'
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return (bytes / Math.pow(1024, i)).toFixed(2) + ' ' + sizes[i]
}

/**
 * Get file info (size and SHA256)
 */
function getFileInfo(fileName) {
  const filePath = path.join(DIST_DIR, fileName)
  try {
    const stats = fs.statSync(filePath)
    return {
      size: formatFileSize(stats.size),
      sha256: calculateSHA256(filePath),
    }
  } catch (error) {
    return { size: 'N/A', sha256: 'N/A' }
  }
}

/**
 * Generate release notes
 */
function generateReleaseNotes(version, changes) {
  const changesList = Array.isArray(changes)
    ? changes
    : changes.split(',').map((c) => c.trim())

  // File configurations for each platform
  const files = {
    windows: [
      {
        name: 'minecraft-updater-windows-x86_64.exe',
        desc: 'Windows x64 (MSVC)',
      },
      {
        name: 'minecraft-updater-windows-gnu-x86_64.exe',
        desc: 'Windows x64 (GNU)',
      },
    ],
    linux: [{ name: 'minecraft-updater-linux-x86_64', desc: 'Linux x64' }],
    macos: [
      { name: 'minecraft-updater-macos-intel-x86_64', desc: 'macOS Intel' },
      {
        name: 'minecraft-updater-macos-apple-silicon-aarch64',
        desc: 'macOS Apple Silicon',
      },
    ],
  }

  // Get info for all files
  const fileInfos = {}
  Object.keys(files).forEach((platform) => {
    files[platform].forEach((f) => {
      fileInfos[f.name] = getFileInfo(f.name)
    })
  })

  const downloadLinks = (platformFiles) => {
    return platformFiles
      .map((f) => {
        const info = fileInfos[f.name]
        return `- **[${f.name}](https://github.com/${GITHUB_REPO}/releases/download/v${version}/${f.name})** - ${f.desc}
  Size: \`${info.size}\` | SHA256: \`${info.sha256}\``
      })
      .join('\n')
  }

  const releaseNotes = `# Minecraft Updater v${version}

🎉 **New Release**

This release includes builds for all major platforms:

## 📦 Downloads

### 🖥️ Windows
${downloadLinks(files.windows)}

### 🐧 Linux
${downloadLinks(files.linux)}

### 🍎 macOS
${downloadLinks(files.macos)}

## ✨ Features

${changesList.map((change) => `- **${change}**`).join('\n')}

## 🚀 Quick Start

\`\`\`bash
# Check for updates
./minecraft-updater --check-updates

# Update specific instance
./minecraft-updater --instance "MyModpack" --update

# Auto-update all instances
./minecraft-updater --auto-update-all
\`\`\`

## 📋 Checksums

All files include SHA256 checksums for verification.

### 📂 Source Code
- [Source code (zip)](https://github.com/${GITHUB_REPO}/archive/refs/tags/v${version}.zip) 📜
- [Source code (tar.gz)](https://github.com/${GITHUB_REPO}/archive/refs/tags/v${version}.tar.gz) 📜

---

**Thank you for using Minecraft Updater!** We're thrilled to bring you these improvements. Let us know your feedback, and happy gaming! 😄

## 📋 Version Info
- **Version**: ${version}
- **Release Date**: ${new Date().toISOString().split('T')[0]}
- **GitHub Repository**: [${GITHUB_REPO}](https://github.com/${GITHUB_REPO})
`

  return releaseNotes
}

// Main execution
if (require.main === module) {
  const args = process.argv.slice(2)

  if (args.length < 2) {
    console.error(
      'Usage: node generate-updater-release-notes.cjs <version> <changes>',
    )
    console.error(
      'Example: node generate-updater-release-notes.cjs 1.0.0 "Auto-update detection,Cross-platform support"',
    )
    process.exit(1)
  }

  const [version, changesStr] = args
  const changes = changesStr.split(',').map((c) => c.trim())

  const releaseNotes = generateReleaseNotes(version, changes)

  // Write to file
  const notesDir = path.join(__dirname, 'notes', 'updater')
  if (!fs.existsSync(notesDir)) {
    fs.mkdirSync(notesDir, { recursive: true })
  }

  const outputFile = path.join(notesDir, `RELEASE_NOTES_UPDATER_v${version}.md`)
  fs.writeFileSync(outputFile, releaseNotes)

  console.log(`✅ Release notes generated: ${outputFile}`)
  console.log('\n' + releaseNotes)
}

module.exports = { generateReleaseNotes }
