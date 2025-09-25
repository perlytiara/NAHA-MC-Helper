# Launcher Integration Guide

This document explains how the Minecraft Installer integrates with different launchers and manages instances across multiple launcher platforms.

## Supported Launchers

### 🎮 Official Minecraft Launcher

- **Detection**: `launcher_profiles.json` + `versions/` directory
- **Instance Format**: Profiles in `launcher_profiles.json`
- **Directory Structure**:
  ```
  .minecraft/
  ├── launcher_profiles.json
  ├── versions/
  ├── libraries/
  ├── assets/
  └── instances/
      └── [instance-name]/
  ```

### 🚀 PrismLauncher

- **Detection**: `prismlauncher.cfg` + `instances/` directory
- **Instance Format**: MultiMC-style instances with `instance.cfg` and `mmc-pack.json`
- **Directory Structure**:
  ```
  PrismLauncher/
  ├── prismlauncher.cfg
  ├── accounts.json
  └── instances/
      └── [instance-name]/
          ├── instance.cfg
          ├── mmc-pack.json
          └── .minecraft/
  ```

### 🏴‍☠️ PrismLauncher-Cracked

- **Detection**: Same as PrismLauncher but with "Offline" accounts
- **Instance Format**: Identical to PrismLauncher
- **Special Features**: Offline authentication support

### 🎯 XMCL (X Minecraft Launcher)

- **Detection**: `config.json` + `instances.json`
- **Instance Format**: JSON-based instance configuration
- **Directory Structure**:
  ```
  .xmcl/
  ├── config.json
  ├── instances.json
  ├── libraries/
  ├── assets/
  └── instances/
      └── [instance-id]/
  ```

### 🌟 AstralRinth App

- **Detection**: `settings.json` + `profiles/` directory
- **Instance Format**: Profile-based with `profile.json`
- **Directory Structure**:
  ```
  AstralRinthApp/
  ├── settings.json
  ├── profiles/
  │   └── [profile-name]/
  │       ├── profile.json
  │       ├── saves/
  │       ├── mods/
  │       └── config/
  └── meta/
  ```

### 📦 MultiMC (Legacy)

- **Detection**: `multimc.cfg` + `instances/` directory
- **Instance Format**: Similar to PrismLauncher
- **Note**: Legacy support, PrismLauncher is recommended

## Usage Examples

### Basic Installation with Launcher Integration

```bash
# Install Minecraft and create instances in all detected launchers
minecraft-installer --version 1.20.1 --create-instance

# Install with specific mod loader
minecraft-installer --version 1.20.1 --loader fabric --create-instance

# List detected launchers
minecraft-installer --list-launchers
```

### Mrpack (Modrinth Modpack) Installation

```bash
# Install modpack and create launcher instances
minecraft-installer --mrpack modpack.mrpack --create-instance

# Install modpack to specific launcher
minecraft-installer --mrpack modpack.mrpack --target-launcher prism
```

### Advanced Options

```bash
# Target specific launcher type
minecraft-installer --version 1.20.1 --target-launcher astralrinth

# Install with verbose logging to see launcher detection
minecraft-installer --version 1.20.1 --create-instance --verbose
```

## Launcher Detection Process

The installer automatically detects launchers by:

1. **Scanning common directories**:
   - Windows: `%APPDATA%`, `%LOCALAPPDATA%`
   - macOS: `~/Library/Application Support/`
   - Linux: `~/.local/share/`, `~/`

2. **Identifying launcher signatures**:
   - Configuration files (`prismlauncher.cfg`, `settings.json`, etc.)
   - Directory structures (`instances/`, `profiles/`, etc.)
   - File patterns specific to each launcher

3. **Priority order for auto-installation**:
   1. AstralRinth App
   2. PrismLauncher
   3. XMCL
   4. Official Minecraft Launcher
   5. MultiMC
   6. PrismLauncher-Cracked

## Instance Creation Details

### Asset Management

The installer handles asset installation differently based on the launcher:

- **Shared Assets**: Official Minecraft Launcher, XMCL
  - Assets stored in global `assets/` directory
  - Instances reference shared assets

- **Instance-Specific**: PrismLauncher, AstralRinth
  - Each instance has its own game directory
  - Assets may be shared or duplicated based on launcher settings

- **Hybrid Approach**: MultiMC
  - Global libraries and assets
  - Instance-specific game directories

### Directory Structure Creation

For each launcher type, the installer creates:

1. **Core Minecraft directories**:
   - `saves/` - World saves
   - `resourcepacks/` - Resource packs
   - `shaderpacks/` - Shader packs (if supported)
   - `mods/` - Mod files
   - `config/` - Configuration files

2. **Launcher-specific files**:
   - Configuration files (`instance.cfg`, `profile.json`, etc.)
   - Metadata files (`mmc-pack.json`, etc.)
   - Launcher integration files

### Mod Loader Integration

The installer supports creating instances with mod loaders:

- **Vanilla**: No mod loader, pure Minecraft
- **Fabric**: Lightweight mod loader
- **Forge**: Traditional mod loader
- **Quilt**: Fabric fork with additional features
- **NeoForge**: Modern Forge fork

Each launcher handles mod loaders differently:

- **PrismLauncher**: Components in `mmc-pack.json`
- **XMCL**: Runtime configuration in instance JSON
- **AstralRinth**: Loader specification in `profile.json`
- **Official**: Manual version selection

## Mrpack Support

### Modrinth Modpack Format

The installer fully supports mrpack files:

1. **Extraction**: Unzips the mrpack file
2. **Index Parsing**: Reads `modrinth.index.json`
3. **File Downloads**: Downloads mods and dependencies
4. **Override Application**: Applies config files and overrides
5. **Instance Creation**: Creates launcher-compatible instances

### Mrpack Structure Handling

```
modpack.mrpack (ZIP file)
├── modrinth.index.json     # Modpack metadata and file list
└── overrides/              # Files to copy to instance
    ├── config/            # Configuration files
    ├── kubejs/            # KubeJS scripts
    └── ...                # Other override files
```

### Installation Process

1. **Parse modpack metadata**:
   - Minecraft version
   - Mod loader type and version
   - Required mods list

2. **Download mod files**:
   - Verify SHA1 hashes
   - Handle multiple download URLs
   - Skip client-incompatible mods

3. **Apply overrides**:
   - Copy config files
   - Set up custom scripts
   - Apply resource packs

4. **Create launcher instances**:
   - Generate launcher-specific configuration
   - Set up mod loader integration
   - Configure memory and Java settings

## Testing

### Automated Tests

Run the comprehensive test suite:

```bash
# Windows
test-launcher-integration.bat

# Unix
./test-launcher-integration.sh

# Rust tests
cargo test launcher_structures
```

### Manual Testing

1. **Launcher Detection**:

   ```bash
   minecraft-installer --list-launchers
   ```

2. **Instance Creation**:

   ```bash
   minecraft-installer --version 1.20.1 --create-instance --verbose
   ```

3. **Mrpack Installation**:
   ```bash
   minecraft-installer --mrpack test-modpack.mrpack --create-instance
   ```

### Test Scenarios

The test suite covers:

- ✅ Launcher detection for all supported types
- ✅ Instance creation with various configurations
- ✅ Mrpack installation and extraction
- ✅ Mod loader integration
- ✅ Asset management across different launchers
- ✅ Error handling and recovery
- ✅ Cross-platform compatibility

## Troubleshooting

### Common Issues

**Launcher Not Detected**:

- Ensure launcher is installed in standard location
- Check launcher-specific configuration files exist
- Use `--verbose` for detailed detection logs

**Instance Creation Failed**:

- Verify write permissions to launcher directory
- Check available disk space
- Ensure launcher is not running during instance creation

**Mrpack Installation Issues**:

- Verify mrpack file is not corrupted
- Check internet connection for mod downloads
- Ensure target directory has write permissions

### Debug Information

Enable verbose logging for detailed information:

```bash
minecraft-installer --verbose --list-launchers
minecraft-installer --verbose --version 1.20.1 --create-instance
```

This provides:

- Launcher detection process
- File system operations
- Network requests and responses
- Error details and stack traces

## Future Enhancements

Planned improvements:

- [ ] ATLauncher integration
- [ ] Technic Launcher support
- [ ] CurseForge modpack support (.zip format)
- [ ] Instance migration between launchers
- [ ] Launcher preference configuration
- [ ] Bulk instance management
- [ ] Launcher-specific optimization settings






