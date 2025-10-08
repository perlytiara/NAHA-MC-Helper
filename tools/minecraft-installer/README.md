# Minecraft Installer & Updater

A comprehensive Minecraft modpack management system with **two separate executables**:

1. **`minecraft-installer`** - Install modpacks to various launchers
2. **`minecraft-updater`** - Scan and intelligently update existing instances

## ✨ Features

### Minecraft Installer
- **Multi-Launcher Support**: Works with AstralRinth, ModrinthApp, XMCL, PrismLauncher, Official Minecraft Launcher, MultiMC, and custom paths
- **Custom Path Installation**: Install directly to any directory (perfect for Legacy Launcher)
- **GitHub API Integration**: Download modpacks directly from GitHub Releases
- **Automodpack Setup**: Automatic server fingerprint and client configuration
- **Database Integration**: Automatic database injection for AstralRinth/ModrinthApp

### Minecraft Updater
- **Intelligent Mod Updates**: Compares versions and only updates what's needed
- **Instance Scanner**: Auto-detects all Minecraft instances across launchers
- **Duplicate Removal**: Automatically removes old mod versions
- **User Mod Protection**: Preserves mods not in the modpack
- **Version Selection**: Update to specific versions or latest
- **Database Sync**: Updates launcher databases with new version info
- **JSON Output**: Perfect for Electron app integration

### Cross-Platform
- Builds for Windows (x64), Linux (x64), macOS (Intel & Apple Silicon)

## 🚀 Quick Start

### Download
Get the latest release from the [Releases](https://github.com/perlytiara/AstralRinth/releases) page.

Download both executables for your platform:
- `minecraft-installer-[platform].exe` / `minecraft-installer-[platform]`
- `minecraft-updater-[platform].exe` / `minecraft-updater-[platform]`

### Installer Usage
```bash
# Install from mrpack file
minecraft-installer --mrpack "modpack.mrpack" --create-instance

# Install to specific launcher
minecraft-installer --mrpack "modpack.mrpack" --target-launcher xmcl --create-instance

# Install to custom path
minecraft-installer --mrpack "modpack.mrpack" --target-launcher other --custom-path "C:\Games\Minecraft" --create-instance

# Download from GitHub API
minecraft-installer --download-neoforge --create-instance
minecraft-installer --download-fabric --create-instance
```

### Updater Usage
```bash
# Scan all instances
minecraft-updater scan --format compact

# Interactive update (select from list)
minecraft-updater interactive --modpack-type neoforge

# Update specific instance to latest
minecraft-updater update --instance-path "C:\path\to\instance" --modpack-type neoforge

# Update to specific version
minecraft-updater update --instance-path "C:\path\to\instance" --modpack-type neoforge --version 0.0.18

# Get JSON output for Electron apps
minecraft-updater scan --format json
```

## 📋 Supported Launchers

| Launcher | Installer | Updater | Database Sync | Automodpack |
|----------|-----------|---------|---------------|-------------|
| **AstralRinth** | ✅ | ✅ | ✅ | ✅ |
| **ModrinthApp** | ✅ | ✅ | ✅ | ✅ |
| **XMCL** | ✅ | ✅ | ❌ | ✅ |
| **PrismLauncher** | ✅ | ✅ | ❌ | ✅ |
| **Official Minecraft** | ✅ | ✅ | ❌ | ✅ |
| **MultiMC** | ✅ | ✅ | ❌ | ✅ |
| **Custom Path** | ✅ | ❌ | ❌ | ✅ |

## 🛠️ Building from Source

### Prerequisites
- Rust 1.90+ 
- Git

### Build Both Executables
```bash
git clone https://github.com/perlytiara/AstralRinth.git
cd AstralRinth/minecraft-installer

# Build both executables
cargo build --release --bin minecraft-installer
cargo build --release --bin minecraft-updater

# Or use the build script
./build.bat   # Windows
./build.sh    # Linux/macOS
```

Executables will be in `target/release/`:
- `minecraft-installer.exe` / `minecraft-installer`
- `minecraft-updater.exe` / `minecraft-updater`

### Automated Multi-Platform Builds
The project uses GitHub Actions to automatically build for all platforms:

```bash
# Create a release tag
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions will automatically:
# - Build for Windows, Linux, macOS (Intel & ARM)
# - Run tests on all platforms
# - Create a GitHub release with all binaries
# - Generate SHA256 checksums
```

See [CROSS_COMPILE.md](CROSS_COMPILE.md) for manual cross-compilation instructions.

## 📖 Documentation

- [Quick Start Guide](QUICK_START.md) - Get up and running quickly
- [Release Notes](RELEASE_NOTES.md) - Detailed feature overview
- [Cross-Compilation Guide](CROSS_COMPILE.md) - Build for multiple platforms
- [Usage Examples](USAGE.md) - Advanced usage examples

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with Rust for performance and reliability
- Supports the Minecraft modding community
- Compatible with major Minecraft launchers

---

**Made with ❤️ for the Minecraft community**