# Minecraft Binaries Integration

This document explains how the NAHA MC Helper integrates the Minecraft Installer and Updater binaries across all platforms.

## Overview

The application bundles pre-built binaries for `minecraft-installer` and `minecraft-updater` for all supported platforms:

- **Windows**: `x86_64` (MSVC and GNU versions)
- **macOS**: Intel (`x86_64`) and Apple Silicon (`aarch64`)
- **Linux**: `x86_64`

## Architecture

### Binary Storage

**Development:**

```text
tools/minecraft-installer-releases/
├── minecraft-installer-windows-x86_64.exe
├── minecraft-installer-windows-gnu-x86_64.exe
├── minecraft-installer-linux-x86_64
├── minecraft-installer-macos-intel-x86_64
├── minecraft-installer-macos-apple-silicon-aarch64
├── minecraft-updater-windows-x86_64.exe
├── minecraft-updater-windows-gnu-x86_64.exe
├── minecraft-updater-linux-x86_64
├── minecraft-updater-macos-intel-x86_64
└── minecraft-updater-macos-apple-silicon-aarch64
```

**Production (after packaging):**

```text
resources/
└── binaries/
    ├── (same files as above)
```

### Utility Module

**Location:** `src/shared/utils/minecraft-binaries.js`

This module provides cross-platform binary management:

```javascript
import { 
  getBinaryPath,           // Get full path to binary
  spawnMinecraftBinary,    // Spawn binary process
  checkBinariesExist       // Check if binaries exist
} from './src/shared/utils/minecraft-binaries.js';
```

#### Key Functions

**`getBinariesDir()`**

- Returns the binaries directory path
- Automatically switches between development and production paths

**`getBinaryName(baseName)`**

- Returns the platform-specific binary name
- Detects Windows/macOS/Linux
- Detects Apple Silicon vs Intel on macOS

**`getBinaryPath(binaryType)`**

- Returns full path to `'minecraft-installer'` or `'minecraft-updater'`
- Combines `getBinariesDir()` and `getBinaryName()`

**`spawnMinecraftBinary(binaryType, args)`**

- Spawns a binary with given arguments
- Returns `ChildProcess` for streaming output

**`checkBinariesExist()`**

- Returns `{ installer: boolean, updater: boolean }`
- Checks if binaries are available

## Usage Examples

### Scanning Minecraft Instances

```javascript
// In main.js
import { spawnMinecraftBinary, checkBinariesExist } from './src/shared/utils/minecraft-binaries.js';

// Check if binary exists
const status = checkBinariesExist();
if (!status.updater) {
  console.log('Binary not found, using fallback');
  return mockData;
}

// Spawn the updater
const args = ['scan', '--format', 'json'];
const process = spawnMinecraftBinary('minecraft-updater', args);

process.stdout.on('data', (data) => {
  console.log(data.toString());
});

process.on('close', (code) => {
  console.log(`Process exited with code ${code}`);
});
```

### Updating an Instance

```javascript
// In main.js
const args = [
  'update',
  '--instance-path', instancePath,
  '--modpack-type', modpackType,
  '--format', 'json'
];

const process = spawnMinecraftBinary('minecraft-updater', args);
```

### Installing from GitHub

```javascript
// In MinecraftInstallerService.js
import { spawnMinecraftBinary } from '../utils/minecraft-binaries.js';

const args = ['--download-neoforge', '--create-instance'];
const process = spawnMinecraftBinary('minecraft-installer', args);
```

## Packaging Configuration

### Electron Forge Config (`forge.config.cjs`)

```javascript
extraResource: [
  {
    from: 'tools/minecraft-installer-releases',
    to: 'binaries',
    filter: ['**/*']
  }
]
```

This copies all binaries to the `resources/binaries` directory when packaging the app.

### Ignored Files

The `tools/` directory is ignored during packaging except for the binaries:

```javascript
ignore: [
  /^\/tools\//,  // Ignore entire tools directory
  // ... other patterns
]
```

Only the files specified in `extraResource` are included.

## Downloading Latest Binaries

### Manual Download Script

**Location:** `tools/download-minecraft-binaries.cjs`

```bash
node tools/download-minecraft-binaries.cjs
```

This script:

1. Fetches the latest release from GitHub
2. Downloads all 10 binaries (5 installer + 5 updater)
3. Saves to `tools/minecraft-installer-releases/`
4. Checks file sizes to avoid re-downloading unchanged files

### Integration with CI/CD

You can integrate this into your build process:

```json
{
  "scripts": {
    "prebuild": "node tools/download-minecraft-binaries.cjs",
    "build": "electron-forge make"
  }
}
```

## Platform Detection

### Windows

- Uses MSVC build by default (`-windows-x86_64.exe`)
- GNU build available as fallback (`-windows-gnu-x86_64.exe`)

### macOS

- Detects `process.arch`
- `arm64` → Apple Silicon build (`-macos-apple-silicon-aarch64`)
- `x64` → Intel build (`-macos-intel-x86_64`)

### Linux

- Uses `x86_64` build (`-linux-x86_64`)

## Error Handling

### Binary Not Found

```javascript
const status = checkBinariesExist();
if (!status.installer) {
  console.warn('Installer binary not found');
  // Provide fallback or show error to user
}
```

### Process Execution Errors

```javascript
const process = spawnMinecraftBinary('minecraft-updater', args);

process.on('error', (err) => {
  console.error('Failed to spawn binary:', err);
  // Handle error (missing binary, permission issues, etc.)
});

process.on('close', (code) => {
  if (code !== 0) {
    console.error(`Process exited with code ${code}`);
    // Handle non-zero exit code
  }
});
```

## Development vs Production

### Development

- Binaries loaded from `tools/minecraft-installer-releases/`
- Can be updated by running the download script
- Changes immediately visible (no rebuild needed)

### Production

- Binaries bundled in `resources/binaries/`
- Accessed via `process.resourcesPath`
- Must rebuild app to update binaries

## Testing

### Check Binary Status

```javascript
// From renderer process via IPC
const status = await window.nahaAPI.minecraftUpdater.getBinaryStatus();
console.log('Updater available:', status.updater);
console.log('Installer available:', status.installer);
console.log('Platform:', status.platform);
console.log('Architecture:', status.arch);
```

### Manual Testing

1. Start the app
2. Open DevTools (F12)
3. Run in console:

   ```javascript
   window.nahaAPI.minecraftUpdater.getBinaryStatus()
   ```

## Troubleshooting

### Binary Not Executing

**Problem:** Binary doesn't run (Permission denied on Linux/macOS)

**Solution:** Set executable permission:

```bash
chmod +x tools/minecraft-installer-releases/minecraft-*-linux-x86_64
chmod +x tools/minecraft-installer-releases/minecraft-*-macos-*
```

### Wrong Binary Selected

**Problem:** App uses wrong binary for platform

**Solution:** Check platform detection:

```javascript
console.log('Platform:', process.platform);
console.log('Arch:', process.arch);
console.log('Binary path:', getBinaryPath('minecraft-installer'));
```

### Binary Not Found in Production

**Problem:** Binary works in dev but not in packaged app

**Solution:** Verify `forge.config.cjs` includes the binaries in `extraResource`

### Old Binaries

**Problem:** Using outdated binaries

**Solution:** Re-download latest:

```bash
node tools/download-minecraft-binaries.cjs
```

## Future Improvements

1. **Auto-update binaries**: Check for updates and download automatically
2. **Binary verification**: Verify SHA256 checksums
3. **Fallback mechanism**: Download missing binaries on first run
4. **Binary caching**: Cache in user data directory
5. **Version checking**: Display binary version in UI

## Related Files

- `src/shared/utils/minecraft-binaries.js` - Binary utility module
- `tools/download-minecraft-binaries.cjs` - Download script
- `forge.config.cjs` - Packaging configuration
- `main.js` - Main process IPC handlers
- `preload.js` - IPC bridge to renderer

## Repository Links

- **Minecraft Installer**: <https://github.com/perlytiara/minecraft-installer>
- **Latest Release**: <https://github.com/perlytiara/minecraft-installer/releases/tag/latest>
