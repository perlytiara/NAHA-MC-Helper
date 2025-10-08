# 🚀 Build & Release Guide

Complete guide for building and releasing both `minecraft-installer` and `minecraft-updater` executables.

---

## 📦 What Gets Built

This project produces **TWO executables** for each platform:

1. **`minecraft-installer`** - Installs modpacks to launchers
2. **`minecraft-updater`** - Scans and updates existing instances

### Platforms Supported

- ✅ **Windows x64** (MSVC & GNU)
- ✅ **Linux x64** (GNU)
- ✅ **macOS Intel** (x86_64)
- ✅ **macOS Apple Silicon** (ARM64 - M1/M2/M3)

**Total binaries per release: 10** (5 platforms × 2 executables)

---

## 🏗️ Local Build

### Quick Build (Current Platform Only)

**Windows:**
```cmd
build.bat
```

**Linux/macOS:**
```bash
chmod +x build.sh
./build.sh
```

**Output:**
- `target/release/minecraft-installer.exe` (or no .exe on Unix)
- `target/release/minecraft-updater.exe` (or no .exe on Unix)

### Manual Build Commands

```bash
# Build both executables
cargo build --release --bin minecraft-installer
cargo build --release --bin minecraft-updater

# Build for specific platform
cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-installer
cargo build --release --target x86_64-pc-windows-msvc --bin minecraft-updater
```

---

## 🌐 Cross-Platform Build

### Using Build Scripts

**Windows (build-all.bat):**
```cmd
build-all.bat
```

**Linux/macOS (build-all.sh):**
```bash
chmod +x build-all.sh
./build-all.sh
```

This will attempt to build for all platforms and place binaries in `dist/`:
```
dist/
├── minecraft-installer-windows-x86_64.exe
├── minecraft-updater-windows-x86_64.exe
├── minecraft-installer-windows-gnu-x86_64.exe
├── minecraft-updater-windows-gnu-x86_64.exe
├── minecraft-installer-linux-x86_64
├── minecraft-updater-linux-x86_64
├── minecraft-installer-macos-intel-x86_64
├── minecraft-updater-macos-intel-x86_64
├── minecraft-installer-macos-apple-silicon-aarch64
└── minecraft-updater-macos-apple-silicon-aarch64
```

**Note:** Cross-compilation requires proper toolchains installed. See [CROSS_COMPILE.md](CROSS_COMPILE.md) for setup.

---

## 🤖 Automated Release (GitHub Actions)

### How It Works

The GitHub Actions workflow (`.github/workflows/build.yml`) automatically:

1. **Triggers** when you push a tag starting with `v` (e.g., `v1.0.0`)
2. **Builds** both executables for all 5 platforms in parallel
3. **Tests** each build to ensure quality
4. **Creates** SHA256 checksums for all binaries
5. **Publishes** a GitHub release with all 10 binaries + checksums

### Create a Release

#### Step 1: Prepare Your Code

```bash
# Make sure all changes are committed
git add .
git commit -m "feat: intelligent mod updates & database integration"
git push origin main
```

#### Step 2: Create and Push a Tag

**Option A: Using the release script (Recommended)**

Windows:
```cmd
create-release.bat 1.0.0
```

Linux/macOS:
```bash
chmod +x create-release.sh
./create-release.sh 1.0.0
```

**Option B: Manual Git commands**

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release v1.0.0"

# Push tag to GitHub
git push origin v1.0.0
```

#### Step 3: Monitor Build

1. Go to: `https://github.com/perlytiara/AstralRinth/actions`
2. Click on the latest "Build and Release" workflow
3. Watch the progress (typically 10-15 minutes for all platforms)

#### Step 4: Verify Release

Once complete, check:
- `https://github.com/perlytiara/AstralRinth/releases`

You should see a new release with:
- ✅ 10 binary files (2 per platform)
- ✅ 10 `.sha256` checksum files
- ✅ Comprehensive release notes
- ✅ Download counts starting at 0

---

## 📋 Release Artifacts

Each release includes:

### Windows Builds
```
minecraft-installer-windows-x86_64.exe           (MSVC - recommended)
minecraft-installer-windows-x86_64.exe.sha256
minecraft-updater-windows-x86_64.exe
minecraft-updater-windows-x86_64.exe.sha256

minecraft-installer-windows-gnu-x86_64.exe       (GNU - alternative)
minecraft-installer-windows-gnu-x86_64.exe.sha256
minecraft-updater-windows-gnu-x86_64.exe
minecraft-updater-windows-gnu-x86_64.exe.sha256
```

### Linux Builds
```
minecraft-installer-linux-x86_64
minecraft-installer-linux-x86_64.sha256
minecraft-updater-linux-x86_64
minecraft-updater-linux-x86_64.sha256
```

### macOS Builds
```
minecraft-installer-macos-intel-x86_64           (Intel Macs)
minecraft-installer-macos-intel-x86_64.sha256
minecraft-updater-macos-intel-x86_64
minecraft-updater-macos-intel-x86_64.sha256

minecraft-installer-macos-apple-silicon-aarch64  (M1/M2/M3 Macs)
minecraft-installer-macos-apple-silicon-aarch64.sha256
minecraft-updater-macos-apple-silicon-aarch64
minecraft-updater-macos-apple-silicon-aarch64.sha256
```

---

## 🔍 Version Naming

Use [Semantic Versioning](https://semver.org/):

- **`v1.0.0`** - Major release (breaking changes)
- **`v0.2.0`** - Minor release (new features, backward compatible)
- **`v0.1.1`** - Patch release (bug fixes)
- **`v1.0.0-beta.1`** - Pre-release (marked as pre-release on GitHub)
- **`v1.0.0-rc.1`** - Release candidate

---

## 🎯 Release Checklist

Before creating a release:

- [ ] All tests pass locally (`cargo test`)
- [ ] Both binaries build successfully (`./build.bat` or `./build.sh`)
- [ ] Documentation is up to date
- [ ] Version number updated in `Cargo.toml` (if needed)
- [ ] CHANGELOG or release notes prepared
- [ ] All changes committed and pushed to `main`

After creating the tag:

- [ ] GitHub Actions workflow completes successfully
- [ ] All 10 binaries are present in the release
- [ ] All checksums are generated
- [ ] Release notes look correct
- [ ] Download and test one binary to verify

---

## 🧪 Testing Builds

### Test Locally Built Binaries

```bash
# Test installer
target/release/minecraft-installer --list-launchers
target/release/minecraft-installer --download-neoforge --create-instance

# Test updater
target/release/minecraft-updater scan --format compact
target/release/minecraft-updater interactive --modpack-type neoforge
```

### Test Downloaded Release Binaries

```bash
# Download from GitHub release
wget https://github.com/perlytiara/AstralRinth/releases/download/v1.0.0/minecraft-installer-linux-x86_64
wget https://github.com/perlytiara/AstralRinth/releases/download/v1.0.0/minecraft-updater-linux-x86_64

# Make executable
chmod +x minecraft-installer-linux-x86_64
chmod +x minecraft-updater-linux-x86_64

# Verify checksum
wget https://github.com/perlytiara/AstralRinth/releases/download/v1.0.0/minecraft-installer-linux-x86_64.sha256
shasum -a 256 -c minecraft-installer-linux-x86_64.sha256

# Test
./minecraft-installer-linux-x86_64 --list-launchers
./minecraft-updater-linux-x86_64 scan --format compact
```

---

## 🔧 Troubleshooting

### Build Fails on GitHub Actions

**Check the logs:**
1. Go to Actions tab on GitHub
2. Click the failed workflow
3. Click on the failed job
4. Expand the failing step

**Common issues:**
- Dependency installation failed → Check Linux/macOS dependency steps
- Compilation error → Test locally first with `cargo build --release`
- Test failure → Run `cargo test` locally to fix

### Missing Binaries in Release

If some binaries are missing:
- Check if the build step succeeded for that platform
- Verify the artifact upload step completed
- Check the "Organize files and create checksums" step

### Release Not Created

Make sure:
- Tag starts with `v` (e.g., `v1.0.0`, not just `1.0.0`)
- Tag was pushed to GitHub (`git push origin v1.0.0`)
- GitHub Actions has permission to create releases (check repository settings)
- Workflow file is at `.github/workflows/build.yml`

---

## 📊 Build Matrix

The GitHub Actions workflow builds this matrix:

| Platform | OS Runner | Target Triple | Installer | Updater |
|----------|-----------|---------------|-----------|---------|
| **Windows MSVC** | windows-latest | x86_64-pc-windows-msvc | ✅ | ✅ |
| **Windows GNU** | windows-latest | x86_64-pc-windows-gnu | ✅ | ✅ |
| **Linux x64** | ubuntu-latest | x86_64-unknown-linux-gnu | ✅ | ✅ |
| **macOS Intel** | macos-latest | x86_64-apple-darwin | ✅ | ✅ |
| **macOS ARM** | macos-latest | aarch64-apple-darwin | ✅ | ✅ |

**Total CI time:** ~10-15 minutes (parallel builds)

---

## 🎉 Release Workflow Summary

```
┌─────────────────────────────────────────────┐
│ Developer creates tag: v1.0.0                │
│ $ git tag -a v1.0.0 -m "Release v1.0.0"     │
│ $ git push origin v1.0.0                     │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ GitHub Actions Triggered                     │
│ - Detects tag push                           │
│ - Starts build workflow                      │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Parallel Builds (5 platforms)                │
│ ┌─────────────────────────────────────────┐ │
│ │ Windows MSVC                            │ │
│ │ ├── minecraft-installer.exe             │ │
│ │ └── minecraft-updater.exe               │ │
│ └─────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────┐ │
│ │ Windows GNU                             │ │
│ │ ├── minecraft-installer.exe             │ │
│ │ └── minecraft-updater.exe               │ │
│ └─────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────┐ │
│ │ Linux x64                               │ │
│ │ ├── minecraft-installer                 │ │
│ │ └── minecraft-updater                   │ │
│ └─────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────┐ │
│ │ macOS Intel                             │ │
│ │ ├── minecraft-installer                 │ │
│ │ └── minecraft-updater                   │ │
│ └─────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────┐ │
│ │ macOS Apple Silicon                     │ │
│ │ ├── minecraft-installer                 │ │
│ │ └── minecraft-updater                   │ │
│ └─────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Test Suite Runs                              │
│ - Library tests on all platforms             │
│ - Ensures no platform-specific bugs          │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Binary Post-Processing                       │
│ - Strip debug symbols (Linux/macOS)          │
│ - Set executable permissions                 │
│ - Rename with platform suffix                │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Create Checksums                             │
│ - Generate SHA256 for each binary            │
│ - Create .sha256 files                       │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ GitHub Release Created                       │
│ - Title: "Minecraft Tools v1.0.0"           │
│ - 10 binary files                            │
│ - 10 checksum files                          │
│ - Professional release notes                 │
│ - Changelog link                             │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ ✅ Release Published!                        │
│ Users can now download from:                 │
│ github.com/perlytiara/AstralRinth/releases   │
└─────────────────────────────────────────────┘
```

---

## 🎯 Quick Commands

### Local Development Build
```bash
cargo build --release
```

### Build Both Binaries
```bash
cargo build --release --bin minecraft-installer
cargo build --release --bin minecraft-updater
```

### Build for Specific Platform
```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

### Create a Release
```bash
# Using script
./create-release.sh 1.0.0

# Or manually
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

---

## 📝 Release Notes Format

The GitHub Actions workflow automatically generates release notes with:

### Header
```markdown
# Minecraft Installer & Updater v1.0.0

## 🎉 Dual-Executable Release

This release includes **two separate executables** for all major platforms
```

### Download Section
- Lists all binaries organized by type (Installer / Updater)
- Grouped by platform (Windows / Linux / macOS)
- Shows file sizes and architecture

### Features Section
- Highlights key features of installer
- Highlights key features of updater
- Shows what's new in this version

### Quick Start Section
- Basic usage examples for installer
- Basic usage examples for updater
- Integration examples for Electron

### Launcher Support Table
- Shows which launchers work with which tool
- Indicates database sync support
- Shows automodpack compatibility

### Checksums Section
- Instructions for verifying downloads
- Platform-specific verification commands

---

## 🔒 Security & Verification

### Verify Downloaded Binaries

**Windows (PowerShell):**
```powershell
# Get hash of downloaded file
Get-FileHash minecraft-installer-windows-x86_64.exe -Algorithm SHA256

# Compare with checksum file
Get-Content minecraft-installer-windows-x86_64.exe.sha256
```

**Linux/macOS:**
```bash
# Verify using checksum file
shasum -a 256 -c minecraft-installer-linux-x86_64.sha256

# Or manually compare
shasum -a 256 minecraft-installer-linux-x86_64
cat minecraft-installer-linux-x86_64.sha256
```

### Build Reproducibility

The GitHub Actions builds are reproducible because:
- Uses fixed Rust toolchain version (defined in `rust-toolchain.toml`)
- Uses locked dependency versions (`Cargo.lock` is committed)
- Uses official GitHub-hosted runners
- Same build flags for all platforms

---

## 🎨 Customizing the Workflow

### Add a New Platform

Edit `.github/workflows/build.yml`:

```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  platform_suffix: linux-aarch64
```

### Change Release Notes Template

Edit the "Create release notes" step in `.github/workflows/build.yml`:

```yaml
- name: Create release notes
  id: release_notes
  run: |
    cat > release_notes.md << 'EOF'
    # Your custom release notes template here
    EOF
```

### Add Build Steps

Add before the "Build both binaries" step:

```yaml
- name: Custom build step
  run: |
    echo "Doing custom build preparation..."
    # Your custom commands
```

---

## 📦 Binary Sizes

Typical sizes (uncompressed):

| Binary | Windows | Linux | macOS |
|--------|---------|-------|-------|
| **minecraft-installer** | ~10-12 MB | ~11-13 MB | ~10-12 MB |
| **minecraft-updater** | ~10-12 MB | ~11-13 MB | ~10-12 MB |

**Note:** Sizes vary slightly between platforms due to:
- Platform-specific system libraries
- Debug symbol stripping (Linux/macOS only)
- Compression differences

---

## 🚨 Common Issues

### Issue: "Cross-compilation failed"
**Solution:** Install the target toolchain:
```bash
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Issue: "OpenSSL not found" (Linux builds)
**Solution:** Install OpenSSL development libraries:
```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install openssl-devel
```

### Issue: "GitHub release not created"
**Solution:** Check these:
1. Tag pushed successfully: `git push origin v1.0.0`
2. `GH_TOKEN` secret exists in repository settings
3. Workflow has write permissions for releases
4. No build failures in any platform

### Issue: "Binary doesn't run on target platform"
**Solution:**
- Ensure you downloaded the correct platform binary
- On Linux/macOS, make sure file is executable: `chmod +x binary`
- Check system requirements (glibc version for Linux)

---

## 💡 Best Practices

### Before Each Release

1. **Test locally** on your development platform
2. **Run all tests**: `cargo test`
3. **Update documentation** if APIs changed
4. **Check for breaking changes** in dependencies
5. **Review commit history** since last release
6. **Update version in Cargo.toml** if needed

### Release Frequency

- **Patch releases**: As needed for bug fixes
- **Minor releases**: Monthly or when significant features are added
- **Major releases**: When breaking changes are necessary

### Release Communication

After publishing:
- Announce in project Discord/community
- Update documentation links
- Create migration guide if breaking changes
- Pin important releases

---

## 📈 Monitoring Releases

### Download Statistics

View download counts:
- Go to: `https://github.com/perlytiara/AstralRinth/releases`
- Each release shows download count per asset
- Total downloads shown at the bottom

### Build Success Rate

Monitor CI/CD health:
- Go to: `https://github.com/perlytiara/AstralRinth/actions`
- Check "Build and Release" workflow history
- Aim for 100% success rate on tagged releases

---

## 🎓 Example Release Flow

```bash
# 1. Development complete
git add .
git commit -m "feat: add intelligent mod comparison"
git push origin main

# 2. Create release
./create-release.sh 1.0.0

# Output:
# 🚀 Creating release v1.0.0...
# 📝 Creating tag v1.0.0...
# 📤 Pushing tag to remote...
# ✅ Release v1.0.0 created successfully!
# 
# 🎉 GitHub Actions will now automatically:
#    • Build for all platforms (Windows, Linux, macOS)
#    • Run tests
#    • Create a GitHub release with all binaries
#    • Generate checksums for verification

# 3. Wait for build (10-15 minutes)
# Monitor at: https://github.com/perlytiara/AstralRinth/actions

# 4. Release published!
# Download from: https://github.com/perlytiara/AstralRinth/releases/tag/v1.0.0

# 5. Test the release
# Download one binary and verify it works
```

---

## 🌟 Advanced: Manual Release Creation

If GitHub Actions is unavailable, you can manually build and create a release:

```bash
# 1. Build all platforms locally (requires cross-compilation setup)
./build-all.sh

# 2. Create checksums
cd dist
for file in minecraft-*; do
  shasum -a 256 "$file" > "${file}.sha256"
done

# 3. Create release on GitHub manually
# - Go to Releases → Draft a new release
# - Create tag: v1.0.0
# - Upload all files from dist/
# - Add release notes
# - Publish
```

---

## ✅ Success Criteria

A successful release has:

- ✅ All 10 binaries present (2 per platform)
- ✅ All 10 checksum files present
- ✅ All binaries download correctly
- ✅ At least one binary tested and working
- ✅ Release notes are clear and accurate
- ✅ Version tag follows semantic versioning
- ✅ No build failures in GitHub Actions
- ✅ Changelog/release notes mention key changes

---

**Ready to release?** Just run `./create-release.sh [version]` and let automation do the rest! 🚀

