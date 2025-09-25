# Minecraft Installer v0.1.0 - Release Notes

## 🎉 New Features

### ✨ "Other" Launcher Type

- **Custom Path Installation**: Install Minecraft directly into any custom directory
- **No Subdirectories**: Files are installed directly into the specified path (no `.minecraft` folder)
- **Legacy Launcher Compatible**: Perfect for Legacy Launcher or any custom setup
- **Usage**: `--target-launcher other --custom-path "C:\path\to\install"`

### 🔧 Enhanced Launcher Support

- **ModrinthApp Support**: Full integration with ModrinthApp launcher
- **Improved Detection**: Better launcher type detection and prioritization
- **Database Integration**: Proper SQLite database injection for AstralRinth/ModrinthApp

### 📦 API Integration

- **Remote Downloads**: Download modpacks directly from NAHA API
- **NeoForge Support**: `--download-neoforge` for automatic NeoForge modpack installation
- **Fabric Support**: `--download-fabric` for automatic Fabric modpack installation
- **Automodpack Setup**: Automatic server fingerprint and client configuration

## 🛠️ Technical Improvements

### 🐛 Bug Fixes

- **XMCL Integration**: Fixed instance creation and file copying for XMCL
- **File Copying**: Improved `servers.dat` and other essential file copying
- **Path Validation**: Better custom path handling and validation
- **Error Handling**: More descriptive error messages and logging

### ⚡ Performance

- **Parallel Downloads**: Faster mod download with progress indicators
- **Optimized File Operations**: Improved directory creation and file copying
- **Memory Efficiency**: Better resource management during large installations

## 📋 Supported Launchers

| Launcher               | Status          | Features                                            |
| ---------------------- | --------------- | --------------------------------------------------- |
| **AstralRinth**        | ✅ Full Support | Profile creation, database integration, automodpack |
| **ModrinthApp**        | ✅ Full Support | Profile creation, database integration, automodpack |
| **XMCL**               | ✅ Full Support | Instance creation, proper file structure            |
| **PrismLauncher**      | ✅ Full Support | Instance creation, mmc-pack.json                    |
| **Official Minecraft** | ✅ Full Support | Profile creation, launcher_profiles.json            |
| **MultiMC**            | ✅ Full Support | Instance creation, multimc.cfg                      |
| **Other/Custom**       | ✅ **NEW**      | Direct path installation, no subdirectories         |

## 🚀 Usage Examples

### Basic Installation

```bash
# Install from mrpack file
minecraft-installer.exe --mrpack "modpack.mrpack" --create-instance

# Install to specific launcher
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher xmcl --create-instance
```

### Custom Path Installation (NEW)

```bash
# Install directly to custom path (like Legacy Launcher)
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher other --custom-path "C:\Games\Minecraft" --create-instance
```

### API Downloads (NEW)

```bash
# Download and install NeoForge modpack from API
minecraft-installer.exe --download-neoforge --create-instance

# Download and install Fabric modpack from API
minecraft-installer.exe --download-fabric --create-instance
```

### Advanced Usage

```bash
# Verbose logging
minecraft-installer.exe --mrpack "modpack.mrpack" --create-instance --verbose

# List available launchers
minecraft-installer.exe --list-launchers

# List available versions
minecraft-installer.exe --list-versions
```

## 📁 File Structure

### Standard Launcher Installation

```
launcher/
├── instances/
│   └── NAHA-NeoForge/
│       ├── .minecraft/          # or direct files for some launchers
│       ├── profile.json         # AstralRinth/ModrinthApp
│       ├── instance.json        # XMCL
│       └── mmc-pack.json        # PrismLauncher
```

### Custom Path Installation (NEW)

```
C:\Custom\Path\
├── mods/                        # Direct installation
├── config/
├── saves/
├── versions/
├── launcher_profiles.json
├── options.txt
└── ... (all Minecraft files directly here)
```

## 🔧 Build Information

- **Platform**: Windows x86_64
- **Size**: 10.4 MB
- **Dependencies**: None (statically linked)
- **Rust Version**: 1.90.0
- **Target**: x86_64-pc-windows-msvc

## 🐛 Known Issues

- Cross-compilation to Linux/macOS requires additional setup (see CROSS_COMPILE.md)
- Some launchers may require manual configuration after installation
- Large modpacks may take several minutes to download and install

## 🔮 Future Plans

- [ ] GitHub Actions for automated cross-platform builds
- [ ] GUI interface for easier usage
- [ ] Support for additional launcher types
- [ ] Modpack update functionality
- [ ] Backup and restore features

## 📞 Support

For issues, feature requests, or questions:

- Check the documentation in the repository
- Review the CROSS_COMPILE.md for build issues
- Test with the provided example modpacks

---

**Build Date**: September 24, 2025
**Version**: 0.1.0
**Platform**: Windows x86_64

