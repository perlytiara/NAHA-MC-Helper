# Minecraft Installer

A standalone Minecraft installer extracted from AstralRinth, designed to download and install Minecraft versions independently.

## Features

- ✅ Download and install any Minecraft version
- ✅ Automatic Java installation (correct version for each Minecraft version)
- ✅ Download game assets, libraries, and client files
- ✅ **Multi-launcher integration** (Official, PrismLauncher, XMCL, AstralRinth, MultiMC)
- ✅ **Automatic instance creation** in detected launchers
- ✅ **Mrpack (Modrinth modpack) support** with full installation
- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Progress bars and detailed logging
- ✅ SHA1 verification for all downloads
- 🚧 Mod loader support (planned: Forge, Fabric, Quilt, NeoForge)

## Installation

### From Source

1. Install Rust: https://rustup.rs/
2. Clone or download this repository
3. Build the installer:
   ```bash
   cargo build --release
   ```
4. The executable will be in `target/release/minecraft-installer`

### Pre-built Binaries

Download the latest release from the releases page.

## Usage

### Basic Installation

Install the latest Minecraft release:

```bash
minecraft-installer --version 1.20.1
```

Install to a custom directory:

```bash
minecraft-installer --version 1.20.1 --install-dir /path/to/minecraft
```

### Launcher Integration

Install Minecraft and automatically create instances in detected launchers:

```bash
minecraft-installer --version 1.20.1 --create-instance
```

List detected launchers:

```bash
minecraft-installer --list-launchers
```

### Modpack Installation

Install Modrinth modpack (.mrpack file):

```bash
minecraft-installer --mrpack modpack.mrpack --create-instance
```

### Advanced Options

Install with a mod loader (coming soon):

```bash
minecraft-installer --version 1.20.1 --loader fabric --loader-version stable
```

Force reinstall even if already installed:

```bash
minecraft-installer --version 1.20.1 --force
```

Enable verbose logging:

```bash
minecraft-installer --version 1.20.1 --verbose
```

### List Available Versions

List all available versions:

```bash
minecraft-installer --list-versions
```

List only release versions:

```bash
minecraft-installer --list-versions --type release
```

## Directory Structure

The installer creates the following directory structure:

```
installation-directory/
├── minecraft/
│   ├── versions/
│   │   └── 1.20.1/
│   │       ├── 1.20.1.jar
│   │       ├── 1.20.1.json
│   │       └── natives/
│   ├── libraries/
│   ├── assets/
│   │   ├── indexes/
│   │   └── objects/
│   └── launcher_profiles.json
├── java/
│   ├── java-8/
│   ├── java-17/
│   └── java-21/
├── instances/
└── logs/
```

## Supported Minecraft Versions

- All Minecraft versions from 1.6+ are supported
- Automatic Java version detection:
  - Java 8 for Minecraft 1.6 - 1.16
  - Java 17 for Minecraft 1.17 - 1.20.4
  - Java 21 for Minecraft 1.20.5+

## Platform Support

- **Windows**: x64, ARM64
- **macOS**: x64, ARM64 (Apple Silicon)
- **Linux**: x64, ARM64

## Integration with Existing Launchers

The installer creates standard Minecraft launcher profiles that can be imported into:

- Official Minecraft Launcher
- MultiMC
- PolyMC
- Prism Launcher
- Other compatible launchers

## Troubleshooting

### Java Installation Issues

If Java installation fails, you can:

1. Install Java manually from https://adoptium.net/
2. Set the `JAVA_HOME` environment variable
3. Ensure Java is in your system PATH

### Download Issues

If downloads fail:

1. Check your internet connection
2. Try again with `--force` flag
3. Check firewall/antivirus settings
4. Use `--verbose` for detailed error information

### Permission Issues

On Unix systems, you may need to make the installer executable:

```bash
chmod +x minecraft-installer
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=minecraft_installer=debug cargo run -- --version 1.20.1
```

### Project Structure

- `src/main.rs` - Command-line interface and main entry point
- `src/installer.rs` - Main installer logic
- `src/download.rs` - Download manager for Minecraft files
- `src/java.rs` - Java installation manager
- `src/directories.rs` - Directory structure management
- `src/error.rs` - Error types and handling

## License

This project is based on code from AstralRinth and follows the same licensing terms.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Acknowledgments

- Based on the excellent work from the AstralRinth project
- Uses Mojang's official Minecraft APIs
- Java runtime provided by Eclipse Adoptium
