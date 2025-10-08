# Quick Start Guide

## 🚀 Getting Started

### 1. Download the Installer

- Get `minecraft-installer-windows-x86_64.exe` from the `dist/` folder
- No additional dependencies required!

### 2. Basic Usage

#### Install from mrpack file

```bash
minecraft-installer.exe --mrpack "your-modpack.mrpack" --create-instance
```

#### Install to specific launcher

```bash
minecraft-installer.exe --mrpack "your-modpack.mrpack" --target-launcher xmcl --create-instance
```

#### Install to custom path (NEW)

```bash
minecraft-installer.exe --mrpack "your-modpack.mrpack" --target-launcher other --custom-path "C:\Games\Minecraft" --create-instance
```

### 3. Available Launchers

Check what launchers are detected:

```bash
minecraft-installer.exe --list-launchers
```

Common launcher names:

- `astralrinth` - AstralRinth
- `modrinthapp` - ModrinthApp
- `xmcl` - XMCL
- `prism` - PrismLauncher
- `official` - Official Minecraft Launcher
- `multimc` - MultiMC
- `other` - Custom path installation

### 4. API Downloads (NEW!)

Download modpacks directly from NAHA API:

```bash
# NeoForge modpack
minecraft-installer.exe --download-neoforge --create-instance

# Fabric modpack
minecraft-installer.exe --download-fabric --create-instance
```

### 5. Get Help

```bash
minecraft-installer.exe --help
```

## 🎯 Common Use Cases

### For Legacy Launcher Users

```bash
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher other --custom-path "C:\Users\%USERNAME%\AppData\Roaming\.minecraft" --create-instance
```

### For XMCL Users

```bash
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher xmcl --create-instance
```

### For AstralRinth Users

```bash
minecraft-installer.exe --mrpack "modpack.mrpack" --target-launcher astralrinth --create-instance
```

## 🔧 Troubleshooting

### Verbose Logging

Add `--verbose` to see detailed installation progress:

```bash
minecraft-installer.exe --mrpack "modpack.mrpack" --create-instance --verbose
```

### Check Available Versions

```bash
minecraft-installer.exe --list-versions
```

### Common Issues

- **"No compatible launchers found"**: Make sure you have a supported launcher installed
- **"Custom path does not exist"**: The installer will create the directory automatically
- **"Failed to download"**: Check your internet connection and try again

## 📁 Output Locations

After installation, your modpack will be available in:

- **XMCL**: `%USERPROFILE%\.xmcl\instances\NAHA-NeoForge\`
- **AstralRinth**: `%USERPROFILE%\AppData\Roaming\AstralRinth\profiles\NAHA-NeoForge\`
- **PrismLauncher**: `%USERPROFILE%\AppData\Roaming\PrismLauncher\instances\NAHA-NeoForge\`
- **Custom Path**: Whatever path you specified with `--custom-path`

## 🎉 Success

Once installed, launch your launcher and you should see the new instance ready to play!
