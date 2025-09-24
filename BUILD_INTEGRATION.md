# Minecraft Installer Integration

This document explains how the `minecraft-installer.exe` is integrated into the NAHA MC Helper build process.

## Overview

The `minecraft-installer.exe` is a Rust-based command-line tool located in `tools/minecraft-installer/` that handles the actual installation of Minecraft modpacks. It needs to be built and included in the final Electron application.

## Build Process

### 1. Building the Minecraft Installer

The Rust-based minecraft-installer must be built before packaging the Electron app:

```bash
# Windows
cd tools/minecraft-installer
build.bat

# Linux/macOS
cd tools/minecraft-installer
./build.sh
```

This creates `tools/minecraft-installer/target/release/minecraft-installer.exe`

### 2. Building the Electron App

The build process is configured to automatically include the minecraft-installer.exe:

```bash
# Build everything (installer + frontend + Electron app)
npm run dist

# Or use the convenience scripts
# Windows
build-with-installer.bat

# Linux/macOS
./build-with-installer.sh
```

## Configuration Files

### package.json

The `build` section configures electron-builder to include the minecraft-installer.exe:

```json
{
  "build": {
    "extraResources": [
      {
        "from": "tools/minecraft-installer/target/release/minecraft-installer.exe",
        "to": "minecraft-installer.exe"
      }
    ]
  }
}
```

### forge.config.cjs

The Electron Forge configuration ensures the executable is included:

```javascript
{
  packagerConfig: {
    extraResource: [
      {
        from: 'tools/minecraft-installer/target/release/minecraft-installer.exe',
        to: 'minecraft-installer.exe'
      }
    ]
  }
}
```

### MinecraftInstallerService.js

The service automatically detects the correct path for the executable:

- **Development**: `tools/minecraft-installer/target/release/minecraft-installer.exe`
- **Production**: `process.resourcesPath/minecraft-installer.exe`

## File Structure

```
NAHA-MC-Helper/
├── tools/
│   └── minecraft-installer/
│       ├── src/           # Rust source code
│       ├── target/
│       │   └── release/
│       │       └── minecraft-installer.exe  # Built executable
│       └── build.bat      # Windows build script
├── src/
│   └── shared/
│       └── services/
│           └── MinecraftInstallerService.js  # Service that uses the executable
├── package.json           # Build configuration
├── forge.config.cjs       # Forge configuration
└── build-with-installer.bat  # Convenience build script
```

## How It Works

1. **Development**: The `MinecraftInstallerService` looks for the executable in the `tools/` directory
2. **Production**: The executable is packaged as an extra resource and accessed via `process.resourcesPath`
3. **IPC Communication**: The main process uses the service to communicate with the minecraft-installer.exe
4. **Real-time Updates**: Progress updates are streamed from the installer to the frontend

## Troubleshooting

### Executable Not Found

If the minecraft-installer.exe is not found:

1. Ensure it's built: `cd tools/minecraft-installer && build.bat`
2. Check the path in `MinecraftInstallerService.js`
3. Verify the build configuration in `package.json`

### Build Failures

If the build fails:

1. Check that Rust is installed and `cargo` is available
2. Ensure all dependencies are installed: `npm install`
3. Try building the installer separately first

### Runtime Issues

If the installer doesn't work at runtime:

1. Check the console logs for path information
2. Verify the executable has proper permissions
3. Test the installer manually from the command line

## Development vs Production

- **Development**: Uses the executable directly from the `tools/` directory
- **Production**: Uses the packaged executable from `extraResources`
- The service automatically detects which environment it's running in
