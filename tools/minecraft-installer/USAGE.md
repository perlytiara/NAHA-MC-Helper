# Minecraft Installer Usage Guide

## Quick Start

### 1. List Available Versions

```bash
minecraft-installer --list-versions
```

This will show you all available Minecraft versions:

```
🎮 Available Minecraft Versions
═════════════════════════════════
Latest Release: 1.20.1
Latest Snapshot: 23w31a

Recent Versions (all types):
─────────────────────────────────
1.20.1          release
1.20            release
1.19.4          release
1.19.3          release
1.19.2          release
```

### 2. Install a Specific Version

```bash
minecraft-installer --version 1.20.1
```

### 3. Install to Custom Directory

```bash
minecraft-installer --version 1.20.1 --install-dir /path/to/minecraft
```

## Command Line Options

### Required Options

- `--version <VERSION>` - Minecraft version to install (e.g., "1.20.1", "1.19.4")
  - Not required when using `--list-versions`

### Optional Options

- `--install-dir <PATH>` - Custom installation directory
  - Default: System's data directory + "MinecraftInstaller"
  - Windows: `%APPDATA%\MinecraftInstaller`
  - macOS: `~/Library/Application Support/MinecraftInstaller`
  - Linux: `~/.local/share/MinecraftInstaller`

- `--loader <LOADER>` - Mod loader to install
  - Options: `vanilla` (default)
  - Future: `forge`, `fabric`, `quilt`, `neoforge`

- `--loader-version <VERSION>` - Loader version
  - Options: `stable` (default), `latest`, or specific version

- `--force` - Force reinstall even if already installed

- `--verbose` - Enable detailed logging

- `--list-versions` - List available Minecraft versions

- `--version-type <TYPE>` - Filter versions by type when listing
  - Options: `release`, `snapshot`, `alpha`, `beta`

## Examples

### Basic Installation

```bash
# Install latest release
minecraft-installer --version 1.20.1

# Install specific version
minecraft-installer --version 1.19.4

# Install snapshot
minecraft-installer --version 23w31a
```

### Custom Installation Directory

```bash
# Windows
minecraft-installer --version 1.20.1 --install-dir "C:\Games\Minecraft"

# macOS/Linux
minecraft-installer --version 1.20.1 --install-dir "~/Games/Minecraft"
```

### Version Management

```bash
# List all versions
minecraft-installer --list-versions

# List only releases
minecraft-installer --list-versions --version-type release

# List only snapshots
minecraft-installer --list-versions --version-type snapshot
```

### Advanced Options

```bash
# Force reinstall
minecraft-installer --version 1.20.1 --force

# Verbose logging
minecraft-installer --version 1.20.1 --verbose

# Future: Mod loaders (not yet implemented)
minecraft-installer --version 1.20.1 --loader fabric --loader-version stable
```

## Installation Process

When you run the installer, it will:

1. **Create Directory Structure**

   ```
   installation-directory/
   ├── minecraft/
   │   ├── versions/1.20.1/
   │   ├── libraries/
   │   ├── assets/
   │   └── launcher_profiles.json
   ├── java/
   ├── instances/
   └── logs/
   ```

2. **Download Required Java**
   - Automatically detects required Java version
   - Downloads from Eclipse Adoptium if needed
   - Java 8 for MC 1.6-1.16
   - Java 17 for MC 1.17-1.20.4
   - Java 21 for MC 1.20.5+

3. **Download Minecraft Files**
   - Client JAR file
   - Version JSON metadata
   - Game libraries
   - Assets (textures, sounds, etc.)
   - Native libraries

4. **Create Launcher Profile**
   - Creates `launcher_profiles.json`
   - Compatible with official launcher
   - Can be imported into MultiMC, PolyMC, etc.

## Integration with Launchers

### Official Minecraft Launcher

1. Copy `launcher_profiles.json` to your `.minecraft` folder
2. Or import the profile through the launcher

### MultiMC/PolyMC/Prism Launcher

1. Create new instance
2. Import from the created instance directory
3. Or copy files manually

### Custom Launchers

The installer creates standard Minecraft directory structure that most launchers can import.

## Troubleshooting

### Common Issues

**Java Installation Failed**

```bash
# Try with verbose logging
minecraft-installer --version 1.20.1 --verbose

# Install Java manually and ensure it's in PATH
```

**Download Failed**

```bash
# Check internet connection and try again
minecraft-installer --version 1.20.1 --force

# Use verbose logging to see detailed errors
minecraft-installer --version 1.20.1 --verbose
```

**Permission Denied**

```bash
# On Unix systems, ensure executable permissions
chmod +x minecraft-installer

# Try different install directory
minecraft-installer --version 1.20.1 --install-dir ~/minecraft-test
```

**Version Not Found**

```bash
# List available versions first
minecraft-installer --list-versions

# Make sure version string is exact
minecraft-installer --version 1.20.1  # ✓ Correct
minecraft-installer --version 1.20    # ✗ May not work
```

### Verbose Logging

For detailed troubleshooting, use verbose mode:

```bash
minecraft-installer --version 1.20.1 --verbose
```

This will show:

- Download progress for each file
- Java installation steps
- File verification (SHA1 checks)
- Directory creation
- Error details

### Log Files

Installation logs are saved to:

- Windows: `%APPDATA%\MinecraftInstaller\logs\`
- macOS: `~/Library/Application Support/MinecraftInstaller/logs/`
- Linux: `~/.local/share/MinecraftInstaller/logs/`

## Performance Tips

### Faster Downloads

- Use wired internet connection
- Close other bandwidth-heavy applications
- Some versions have many small asset files that take time

### Disk Space

- Minecraft versions typically require 100-500MB
- Java installations require 50-150MB each
- Assets are shared between versions

### Multiple Versions

- You can install multiple Minecraft versions
- Each gets its own directory under `versions/`
- Libraries and assets are shared when possible










