# Minecraft Installer Platform Integration

## Overview

The NAHA MC Helper application now supports cross-platform minecraft-installer integration using platform-specific executables from the `tools/minecraft-installer-releases/` folder.

## Platform-Specific Executables

The following executables are available for different platforms:

### Windows

- `minecraft-installer-windows-x86_64.exe` - Windows x64 (MSVC)
- `minecraft-installer-windows-gnu-x86_64.exe` - Windows x64 (GNU)

### macOS

- `minecraft-installer-macos-intel-x86_64` - macOS Intel x64
- `minecraft-installer-macos-apple-silicon-aarch64` - macOS Apple Silicon (ARM64)

### Linux

- `minecraft-installer-linux-x86_64` - Linux x64

## Implementation Details

### 1. MinecraftInstallerService Updates

The `MinecraftInstallerService` class has been updated to:

- **Automatic Platform Detection**: Detects the current platform (`win32`, `darwin`, `linux`) and architecture (`x64`, `arm64`)
- **Dynamic Executable Selection**: Chooses the correct executable based on platform and architecture
- **Fallback Support**: Falls back to Windows x64 executable for unknown platforms
- **Enhanced Logging**: Provides detailed logging for debugging platform detection

### 2. Path Resolution

#### Development Mode

```text
tools/minecraft-installer-releases/{executable-name}
```

#### Production Mode (Packaged App)

```text
resources/installer/{executable-name}
```

### 3. Build Configuration

The `package.json` electron-builder configuration has been updated:

```json
"extraResources": [
  {
    "from": "tools/minecraft-installer-releases/",
    "to": "installer/",
    "filter": ["**/*"]
  }
]
```

This ensures all platform-specific executables are included in the packaged application.

### 4. Build Scripts

Both `build-all.sh` and `build-all.bat` have been updated to:

- Build executables for all supported platforms
- Copy executables to both `dist/` and `../minecraft-installer-releases/`
- Provide clear output showing where files are placed

## Platform Detection Logic

```javascript
function getPlatformExecutableName() {
  const platform = process.platform;
  const arch = process.arch;
  
  if (platform === 'win32') {
    return arch === 'x64' 
      ? 'minecraft-installer-windows-x86_64.exe'
      : 'minecraft-installer-windows-gnu-x86_64.exe';
  } else if (platform === 'darwin') {
    return arch === 'arm64'
      ? 'minecraft-installer-macos-apple-silicon-aarch64'
      : 'minecraft-installer-macos-intel-x86_64';
  } else if (platform === 'linux') {
    return 'minecraft-installer-linux-x86_64';
  } else {
    // Fallback to Windows x64
    return 'minecraft-installer-windows-x86_64.exe';
  }
}
```

## Usage

The service automatically selects the correct executable when instantiated:

```javascript
const installerService = new MinecraftInstallerService();
// Automatically uses the correct platform-specific executable
```

## Testing

Platform detection has been tested with all supported combinations:

- ✅ Windows x64 → `minecraft-installer-windows-x86_64.exe`
- ✅ Windows ia32 → `minecraft-installer-windows-gnu-x86_64.exe`
- ✅ macOS Intel → `minecraft-installer-macos-intel-x86_64`
- ✅ macOS Apple Silicon → `minecraft-installer-macos-apple-silicon-aarch64`
- ✅ Linux x64 → `minecraft-installer-linux-x86_64`
- ✅ Linux ARM64 → `minecraft-installer-linux-x86_64` (fallback)
- ✅ Unknown platforms → `minecraft-installer-windows-x86_64.exe` (fallback)

## Benefits

1. **Native Performance**: Each platform uses its native executable
2. **Better Compatibility**: Platform-specific builds handle OS-specific features
3. **Automatic Selection**: No manual configuration required
4. **Fallback Support**: Graceful handling of unknown platforms
5. **Easy Maintenance**: Single service handles all platforms

## File Structure

```text
tools/
├── minecraft-installer/
│   ├── build-all.sh          # Updated to copy to releases folder
│   ├── build-all.bat         # Updated to copy to releases folder
│   └── ... (source code)
└── minecraft-installer-releases/
    ├── minecraft-installer-windows-x86_64.exe
    ├── minecraft-installer-windows-gnu-x86_64.exe
    ├── minecraft-installer-macos-intel-x86_64
    ├── minecraft-installer-macos-apple-silicon-aarch64
    └── minecraft-installer-linux-x86_64
```

## Future Enhancements

- Support for additional architectures (ARM64 Linux, etc.)
- Dynamic executable downloading for missing platforms
- Version management for platform-specific executables
