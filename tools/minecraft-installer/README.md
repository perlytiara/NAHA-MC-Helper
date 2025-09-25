# Minecraft Installer

A standalone Minecraft modpack installer that supports multiple launchers and can install modpacks directly from `.mrpack` files or download them from APIs.

## ✨ Features

- **Multi-Launcher Support**: Works with AstralRinth, ModrinthApp, XMCL, PrismLauncher, Official Minecraft Launcher, MultiMC, and custom paths
- **Custom Path Installation**: Install directly to any directory (perfect for Legacy Launcher)
- **API Integration**: Download modpacks directly from NAHA API
- **Automodpack Setup**: Automatic server fingerprint and client configuration
- **Cross-Platform**: Builds for Windows, Linux, and macOS

## 🚀 Quick Start

### Download
Get the latest release from the [Releases](https://github.com/perlytiara/minecraft-installer/releases) page.

### Basic Usage
```bash
# Install from mrpack file
minecraft-installer.exe --mrpack "modpack.mrpack" --create-instance

# Install to specific launcher
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher xmcl --create-instance

# Install to custom path (NEW!)
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher other --custom-path "C:\Games\Minecraft" --create-instance
```

### API Downloads
```bash
# Download NeoForge modpack from API
minecraft-installer.exe --download-neoforge --create-instance

# Download Fabric modpack from API
minecraft-installer.exe --download-fabric --create-instance
```

## 📋 Supported Launchers

| Launcher | Status | Features |
|----------|--------|----------|
| **AstralRinth** | ✅ Full Support | Profile creation, database integration, automodpack |
| **ModrinthApp** | ✅ Full Support | Profile creation, database integration, automodpack |
| **XMCL** | ✅ Full Support | Instance creation, proper file structure |
| **PrismLauncher** | ✅ Full Support | Instance creation, mmc-pack.json |
| **Official Minecraft** | ✅ Full Support | Profile creation, launcher_profiles.json |
| **MultiMC** | ✅ Full Support | Instance creation, multimc.cfg |
| **Other/Custom** | ✅ **NEW** | Direct path installation, no subdirectories |

## 🛠️ Building from Source

### Prerequisites
- Rust 1.90+ 
- Git

### Build
```bash
git clone https://github.com/perlytiara/minecraft-installer.git
cd minecraft-installer
cargo build --release
```

### Cross-Platform Builds
See [CROSS_COMPILE.md](CROSS_COMPILE.md) for detailed instructions on building for different platforms.

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