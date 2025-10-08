# 🤖 Cursor AI Integration Prompt

## System Overview

This Rust workspace builds **TWO cross-platform executables** for Minecraft modpack management:

### Executables

1. **`minecraft-installer`** (`src/main.rs`)
   - Installs modpacks to launchers (AstralRinth, ModrinthApp, XMCL, PrismLauncher, etc.)
   - Downloads from GitHub Releases API
   - Creates launcher instances with proper database injection
   - Sets up automodpack configuration

2. **`minecraft-updater`** (`src/bin/minecraft-updater.rs`)
   - Scans all Minecraft instances across launchers
   - Intelligently updates mods (compares versions, removes old, adds new, preserves user mods)
   - Updates launcher databases (AstralRinth/ModrinthApp)
   - Supports version selection (latest or specific version)
   - Outputs JSON for Electron integration

---

## Build Configuration

### Cargo.toml Structure

```toml
[[bin]]
name = "minecraft-installer"
path = "src/main.rs"

[[bin]]
name = "minecraft-updater"
path = "src/bin/minecraft-updater.rs"

[lib]
name = "minecraft_installer"
path = "src/lib.rs"
```

Both binaries share the same library code in `src/lib.rs`.

---

## GitHub Actions CI/CD

### Workflow File

`.github/workflows/build.yml`

### Trigger

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### Build Matrix

Builds for **5 platforms** in parallel:

- Windows x64 (MSVC)
- Windows x64 (GNU)
- Linux x64 (GNU)
- macOS x64 (Intel)
- macOS aarch64 (Apple Silicon)

### Output Per Release

- **10 binary files** (2 executables × 5 platforms)
- **10 checksum files** (.sha256)
- **1 GitHub release** with professional notes

### Total Build Time

~10-15 minutes (parallel execution)

---

## Directory Structure

```text
minecraft-installer/
├── src/
│   ├── main.rs                      # minecraft-installer entry
│   ├── lib.rs                       # Shared library
│   ├── bin/
│   │   └── minecraft-updater.rs     # minecraft-updater entry
│   ├── launcher_support.rs          # Launcher detection, instance creation
│   ├── updater.rs                   # Update logic, mod comparison
│   ├── installer.rs                 # Minecraft installation
│   ├── download.rs                  # Download manager
│   ├── java.rs                      # Java detection
│   ├── directories.rs               # Directory management
│   └── error.rs                     # Error types
├── .github/
│   └── workflows/
│       └── build.yml                # CI/CD workflow (BUILDS BOTH BINARIES)
├── Cargo.toml                       # Defines both binary targets
├── build.bat                        # Build both (Windows)
├── build.sh                         # Build both (Linux/macOS)
├── build-all.bat                    # Cross-platform build (Windows)
├── build-all.sh                     # Cross-platform build (Unix)
├── create-release.bat               # Tag & trigger release (Windows)
├── create-release.sh                # Tag & trigger release (Unix)
├── electron-integration.js          # Node.js wrapper for Electron
├── BUILD_AND_RELEASE.md            # Detailed build guide
├── INTEGRATION_SUMMARY.md          # Integration overview
├── RELEASE_QUICK_GUIDE.md          # Quick release instructions
└── README.md                        # Project documentation
```

---

## Key Implementation Details

### 1. GitHub API Integration

Both tools fetch modpacks from GitHub Releases:

```rust
// launcher_support.rs

// Fetch latest modpack
pub async fn fetch_modpack_info(&self, modpack_type: &str) -> Result<NahaModpackInfo>

// Fetch specific version
pub async fn fetch_modpack_info_version(&self, modpack_type: &str, target_version: &str) -> Result<NahaModpackInfo>
```

**API Endpoints:**

- Latest: `https://api.github.com/repos/perlytiara/NAHA-Minecraft-Modpacks/releases/latest`
- Specific: `https://api.github.com/repos/perlytiara/NAHA-Minecraft-Modpacks/releases/tags/NeoForge-0.0.18`

### 2. Intelligent Mod Updating

```rust
// updater.rs

// Main update function
pub async fn update_instance_mods_version(
    &self,
    instance_path: &Path,
    modpack_type: &str,
    version: Option<&str>,
) -> Result<UpdateResult>

// Mod name normalization
fn normalize_mod_name(&self, name: &str) -> String

// Smart mod comparison
async fn update_mods_intelligently(
    &self,
    instance_path: &Path,
    mrpack_index: &MrpackIndex,
    existing_mods: &HashMap<String, ModInfo>,
    modpack_info: &NahaModpackInfo,
) -> Result<UpdateResult>
```

**Algorithm:**

1. Download new mrpack from GitHub
2. Deduplicate mrpack entries
3. Normalize mod names (handles complex names like `sodium-extra-0.6.0+mc1.21.1.jar`)
4. Compare existing mods with new mrpack
5. Remove old versions, download new versions
6. Preserve user-added mods
7. Clean up duplicates
8. Update launcher database

### 3. Database Integration

```rust
// updater.rs

async fn update_launcher_database(
    &self,
    instance_path: &Path,
    modpack_info: &NahaModpackInfo
) -> Result<()>
```

**Updates:**

- AstralRinth `app.db`
- ModrinthApp `app.db`
- Sets `modified` timestamp
- Updates `game_version` field
- Creates entry if doesn't exist

---

## Build & Release Commands

### Local Build (Current Platform)

```bash
# Build both executables
cargo build --release --bin minecraft-installer
cargo build --release --bin minecraft-updater

# Or use helper script
./build.bat   # Windows
./build.sh    # Linux/macOS
```

**Output:**

- `target/release/minecraft-installer[.exe]`
- `target/release/minecraft-updater[.exe]`

### Cross-Platform Build (Advanced)

```bash
./build-all.bat   # Windows
./build-all.sh    # Linux/macOS
```

**Output:** `dist/` with 10 binaries

### Automated Release

```bash
# Create release (triggers GitHub Actions)
./create-release.sh 1.0.0

# GitHub Actions builds everything automatically
```

**Output:** GitHub Release with 10 binaries + 10 checksums + release notes

---

## Usage Examples

### Minecraft Installer

```bash
# Download and install latest NeoForge modpack
minecraft-installer --download-neoforge --create-instance

# Install from local .mrpack file
minecraft-installer --mrpack modpack.mrpack --create-instance

# Install to specific launcher
minecraft-installer --mrpack modpack.mrpack --target-launcher astralrinth --create-instance

# Install to custom path
minecraft-installer --mrpack modpack.mrpack --target-launcher other --custom-path "C:\Games\Minecraft" --create-instance
```

### Minecraft Updater

```bash
# Scan all instances (compact format)
minecraft-updater scan --format compact

# Scan all instances (JSON for Electron)
minecraft-updater scan --format json

# Interactive update (user selects instance)
minecraft-updater interactive --modpack-type neoforge

# Update specific instance to latest
minecraft-updater update --instance-path "C:\path\to\instance" --modpack-type neoforge

# Update to specific version
minecraft-updater update --instance-path "C:\path\to\instance" --modpack-type neoforge --version 0.0.18

# Filter by launcher
minecraft-updater scan --launcher AstralRinth
minecraft-updater interactive --modpack-type neoforge --launcher ModrinthApp
```

---

## Electron Integration

### Node.js Wrapper

Located at: `electron-integration.js`

```javascript
const MinecraftManager = require('./electron-integration').MinecraftUpdaterIntegration
const manager = new MinecraftManager()

// Scan instances
const instances = await manager.scanInstances()
console.log(`Found ${instances.length} instances`)

// Get summary
const summary = await manager.getInstancesSummary()
console.log(`Total: ${summary.total}, AstralRinth: ${summary.byLauncher.AstralRinth}`)

// Update instance
const result = await manager.updateInstance(
  'C:\\Users\\user\\AppData\\Roaming\\AstralRinthApp\\profiles\\NAHA-Neoforge-1.21.1-0.0.18',
  'neoforge'
)
console.log(`Updated ${result.updated_mods.length} mods`)
```

---

## Important Files to Know

### Core Source Files

| File | Purpose | Lines | Key Functions |
|------|---------|-------|---------------|
| `src/main.rs` | Installer entry point | ~300 | CLI parsing, orchestration |
| `src/bin/minecraft-updater.rs` | Updater entry point | ~450 | CLI parsing, output formatting |
| `src/launcher_support.rs` | Launcher logic | ~1770 | `fetch_modpack_info`, `create_instance` |
| `src/updater.rs` | Update logic | ~1375 | `update_instance_mods_version`, `normalize_mod_name` |
| `src/lib.rs` | Library exports | ~20 | Module declarations |

### Build & Release Files

| File | Purpose |
|------|---------|
| `.github/workflows/build.yml` | **CI/CD workflow (MOST IMPORTANT)** |
| `build.bat` / `build.sh` | Local build scripts |
| `build-all.bat` / `build-all.sh` | Cross-platform build scripts |
| `create-release.bat` / `create-release.sh` | Release helper scripts |
| `Cargo.toml` | Binary definitions |

### Documentation Files

| File | Purpose |
|------|---------|
| `BUILD_AND_RELEASE.md` | Complete build & release guide |
| `INTEGRATION_SUMMARY.md` | Integration overview |
| `RELEASE_QUICK_GUIDE.md` | Quick release instructions |
| `README.md` | Project documentation |
| `CURSOR_AI_PROMPT.md` | This file |

---

## Common Tasks

### Task: Build Locally for Testing

```bash
cargo build --release
# Test binaries in target/release/
```

### Task: Build for All Platforms

```bash
./build-all.sh
# Binaries in dist/
```

### Task: Create a Release

```bash
./create-release.sh 1.0.0
# Wait 15 minutes
# Download from GitHub releases
```

### Task: Test a Specific Binary

```bash
# Build just the updater
cargo build --release --bin minecraft-updater

# Run it
target/release/minecraft-updater scan --format compact
```

### Task: Update GitHub Actions Workflow

Edit: `.github/workflows/build.yml`

Common changes:

- Add new platform to matrix
- Change release note template
- Add build dependencies
- Modify artifact names

---

## Debugging

### Local Build Fails

```bash
# Check errors
cargo build --release --bin minecraft-installer 2>&1 | more
cargo build --release --bin minecraft-updater 2>&1 | more

# Run tests
cargo test

# Check specific module
cargo check --bin minecraft-updater
```

### GitHub Actions Build Fails

1. Go to: `https://github.com/perlytiara/AstralRinth/actions`
2. Click failed workflow
3. Click failed job
4. Expand failing step
5. Read error message
6. Fix locally
7. Create new tag: `v1.0.1`

### Release Not Created

Check:

- ✅ Tag starts with `v` (e.g., `v1.0.0` not `1.0.0`)
- ✅ Tag was pushed: `git push origin v1.0.0`
- ✅ Workflow file exists: `.github/workflows/build.yml`
- ✅ Repository has `GH_TOKEN` secret
- ✅ All platform builds succeeded

---

## Quick Reference Card

### Build Commands

```bash
cargo build --release                              # Both binaries (current platform)
cargo build --release --bin minecraft-installer    # Just installer
cargo build --release --bin minecraft-updater      # Just updater
./build.bat / ./build.sh                          # Both binaries with output
./build-all.sh                                    # All platforms (cross-compile)
```

### Release Commands

```bash
./create-release.sh 1.0.0    # Create and push tag (triggers CI/CD)
git tag -a v1.0.0 -m "..."   # Manual tag creation
git push origin v1.0.0       # Manual tag push
```

### Test Commands

```bash
cargo test                                         # Run all tests
cargo test --lib                                  # Library tests only
target/release/minecraft-updater scan             # Test updater
target/release/minecraft-installer --list-launchers  # Test installer
```

---

## For Cursor AI: Key Context

When working with this codebase, remember:

1. **Two separate binaries** from one codebase
   - Don't forget to build both when making changes
   - Both share `src/lib.rs` modules

2. **GitHub Actions builds automatically**
   - On tag push starting with `v`
   - Builds for 5 platforms (10 binaries total)
   - Creates release with checksums

3. **Mod updating is intelligent**
   - Normalizes mod names for comparison
   - Removes old versions before updating
   - Preserves user-added mods
   - Handles complex filenames

4. **Database integration is critical**
   - AstralRinth and ModrinthApp need database updates
   - Updates happen automatically after mod updates
   - Uses rusqlite for SQLite interaction

5. **Version selection is supported**
   - Can update to latest or specific version
   - Uses GitHub Releases API tags

---

## Quick Integration Example

Here's how to integrate both tools into an Electron app:

```javascript
// electron-app/main.js
const path = require('path')
const { spawn } = require('child_process')

const platform = process.platform
const ext = platform === 'win32' ? '.exe' : ''
const binDir = path.join(__dirname, 'binaries', platform)

const INSTALLER = path.join(binDir, `minecraft-installer${ext}`)
const UPDATER = path.join(binDir, `minecraft-updater${ext}`)

// Scan instances
function scanInstances() {
  return new Promise((resolve, reject) => {
    const proc = spawn(UPDATER, ['scan', '--format', 'json'])
    let output = ''
    proc.stdout.on('data', d => output += d)
    proc.on('close', code => {
      code === 0 ? resolve(JSON.parse(output)) : reject(code)
    })
  })
}

// Update instance
function updateInstance(instancePath, modpackType, version = null) {
  return new Promise((resolve, reject) => {
    const args = ['update', '--instance-path', instancePath, '--modpack-type', modpackType, '--format', 'json']
    if (version) args.push('--version', version)
    
    const proc = spawn(UPDATER, args)
    let output = ''
    proc.stdout.on('data', d => output += d)
    proc.on('close', code => {
      code === 0 ? resolve(JSON.parse(output)) : reject(code)
    })
  })
}

// Install modpack
function installModpack(modpackType, targetLauncher = null) {
  return new Promise((resolve, reject) => {
    const args = [`--download-${modpackType}`, '--create-instance']
    if (targetLauncher) args.push('--target-launcher', targetLauncher)
    
    const proc = spawn(INSTALLER, args)
    proc.on('close', code => {
      code === 0 ? resolve({ success: true }) : reject(code)
    })
  })
}
```

---

## File Locations After Build

### Local Build

```text
target/
└── release/
    ├── minecraft-installer[.exe]
    └── minecraft-updater[.exe]
```

### Cross-Platform Build

```text
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

### GitHub Release

```text
https://github.com/perlytiara/AstralRinth/releases/tag/v1.0.0
├── minecraft-installer-windows-x86_64.exe
├── minecraft-installer-windows-x86_64.exe.sha256
├── minecraft-updater-windows-x86_64.exe
├── minecraft-updater-windows-x86_64.exe.sha256
├── ... (18 more files for other platforms)
└── Release notes (auto-generated)
```

---

## GitHub Actions Workflow Highlights

### Build Step

```yaml
- name: Build both binaries
  run: |
    cargo build --release --target ${{ matrix.target }} --bin minecraft-installer
    cargo build --release --target ${{ matrix.target }} --bin minecraft-updater
```

### Prepare Binaries

```yaml
- name: Prepare binaries (Windows)
  if: matrix.os == 'windows-latest'
  run: |
    mkdir -p dist
    cp "target/${{ matrix.target }}/release/minecraft-installer.exe" "dist/minecraft-installer-${{ matrix.platform_suffix }}.exe"
    cp "target/${{ matrix.target }}/release/minecraft-updater.exe" "dist/minecraft-updater-${{ matrix.platform_suffix }}.exe"
```

### Upload Artifacts

```yaml
- name: Upload installer artifact
  uses: actions/upload-artifact@v4
  with:
    name: installer-${{ matrix.platform_suffix }}
    path: dist/minecraft-installer-${{ matrix.platform_suffix }}*

- name: Upload updater artifact
  uses: actions/upload-artifact@v4
  with:
    name: updater-${{ matrix.platform_suffix }}
    path: dist/minecraft-updater-${{ matrix.platform_suffix }}*
```

---

## Complete Release Workflow

```text
┌──────────────────────────────────────┐
│  Developer: ./create-release.sh 1.0.0│
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  Git: Create & Push Tag v1.0.0       │
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  GitHub: Detect Tag Push             │
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  Actions: Trigger build.yml Workflow │
└──────────────────────────────────────┘
                 ↓
    ┌────────────┴────────────┐
    ↓                         ↓
┌─────────┐              ┌─────────┐
│ Windows │              │  Linux  │
│ Build   │              │  Build  │
└─────────┘              └─────────┘
    ↓                         ↓
┌─────────┐              ┌─────────┐
│  macOS  │              │  Tests  │
│  Build  │              │  Pass   │
└─────────┘              └─────────┘
    │                         │
    └────────────┬────────────┘
                 ↓
┌──────────────────────────────────────┐
│  Organize: 10 Binaries → upload/     │
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  Generate: SHA256 Checksums          │
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  Create: Release Notes               │
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  Publish: GitHub Release v1.0.0      │
│  - 10 binaries                       │
│  - 10 checksums                      │
│  - Release notes                     │
└──────────────────────────────────────┘
                 ↓
┌──────────────────────────────────────┐
│  ✅ DONE! Users Can Download         │
└──────────────────────────────────────┘
```

---

## Summary for AI Agents

**Project Type:** Rust workspace with dual binaries  
**Build System:** Cargo + GitHub Actions  
**CI/CD:** Automated multi-platform builds on tag push  
**Platforms:** Windows (MSVC + GNU), Linux, macOS (Intel + ARM)  
**Output:** 10 binaries per release (2 executables × 5 platforms)  
**Release Trigger:** Git tag starting with `v`  
**Build Time:** ~10-15 minutes (parallel)  
**Integration:** JSON output for Electron/Node.js  
**Documentation:** Comprehensive (6+ markdown files)  

**Key Commands:**

- Build: `cargo build --release`
- Release: `./create-release.sh [version]`
- Test: `cargo test`

**Key Files:**

- Workflow: `.github/workflows/build.yml`
- Installer: `src/main.rs`
- Updater: `src/bin/minecraft-updater.rs`
- Shared: `src/launcher_support.rs`, `src/updater.rs`

**Integration Points:**

- Scan: `minecraft-updater scan --format json`
- Update: `minecraft-updater update --instance-path ... --modpack-type ... --format json`
- Install: `minecraft-installer --download-neoforge --create-instance`

---

**For full details, see:**

- [BUILD_AND_RELEASE.md](BUILD_AND_RELEASE.md) - Complete guide
- [INTEGRATION_SUMMARY.md](INTEGRATION_SUMMARY.md) - Technical overview
- [RELEASE_QUICK_GUIDE.md](RELEASE_QUICK_GUIDE.md) - Quick instructions
