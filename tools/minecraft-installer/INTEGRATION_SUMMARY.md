# 🎯 Minecraft Installer & Updater - Integration Summary

## Executive Summary

The project now consists of **two production-ready executables** that provide complete Minecraft modpack management:

1. **`minecraft-installer`** - Install modpacks to various launchers
2. **`minecraft-updater`** - Scan and update existing instances

Both tools are **cross-platform** (Windows, Linux, macOS) and include **automated GitHub Actions CI/CD** for releases.

---

## 🎯 For Cursor AI / Integration

### What You Need to Know

This codebase builds **TWO separate executables** from the same Rust workspace:

- **`minecraft-installer`** (main entry: `src/main.rs`)
  - Purpose: Install new modpack instances
  - Supports: 7 different launcher types
  - Features: GitHub API downloads, database injection, automodpack setup

- **`minecraft-updater`** (main entry: `src/bin/minecraft-updater.rs`)
  - Purpose: Update existing modpack instances
  - Supports: Instance scanning, intelligent mod comparison, version selection
  - Features: JSON output, database sync, duplicate cleanup

### Build Commands

```bash
# Build both executables (local platform)
cargo build --release --bin minecraft-installer
cargo build --release --bin minecraft-updater

# Or use scripts
./build.bat      # Windows
./build.sh       # Linux/macOS

# Output location
target/release/minecraft-installer[.exe]
target/release/minecraft-updater[.exe]
```

### Automated Multi-Platform Release

```bash
# Create and push a version tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# GitHub Actions automatically:
# - Builds for Windows (MSVC & GNU)
# - Builds for Linux (x64)
# - Builds for macOS (Intel & Apple Silicon)
# - Runs tests on all platforms
# - Creates GitHub release with all 10 binaries
# - Generates SHA256 checksums
```

---

## 📦 Release Artifacts

Each release produces **10 binaries** + **10 checksums** = **20 files total**:

### Windows (4 files)

- `minecraft-installer-windows-x86_64.exe` (MSVC)
- `minecraft-updater-windows-x86_64.exe` (MSVC)
- `minecraft-installer-windows-gnu-x86_64.exe` (GNU)
- `minecraft-updater-windows-gnu-x86_64.exe` (GNU)

### Linux (2 files)

- `minecraft-installer-linux-x86_64`
- `minecraft-updater-linux-x86_64`

### macOS (4 files)

- `minecraft-installer-macos-intel-x86_64`
- `minecraft-updater-macos-intel-x86_64`
- `minecraft-installer-macos-apple-silicon-aarch64`
- `minecraft-updater-macos-apple-silicon-aarch64`

---

## 🔧 Integration Points

### For Electron Apps

Both tools provide **JSON output** for programmatic integration:

```javascript
const { spawn } = require('child_process')

// Scan all instances
const scanner = spawn('minecraft-updater', ['scan', '--format', 'json'])
scanner.stdout.on('data', (data) => {
  const instances = JSON.parse(data)
  console.log(`Found ${instances.length} instances`)
})

// Update instance
const updater = spawn('minecraft-updater', [
  'update',
  '--instance-path', instancePath,
  '--modpack-type', 'neoforge',
  '--format', 'json'
])
updater.stdout.on('data', (data) => {
  const result = JSON.parse(data)
  console.log(`Updated ${result.updated_mods.length} mods`)
})
```

### Binary Locations in Electron App

```text
your-electron-app/
├── binaries/
│   ├── win32/
│   │   ├── minecraft-installer.exe
│   │   └── minecraft-updater.exe
│   ├── darwin/
│   │   ├── minecraft-installer
│   │   └── minecraft-updater
│   └── linux/
│       ├── minecraft-installer
│       └── minecraft-updater
└── main.js
```

---

## 🚀 Key Features Implemented

### Intelligent Mod Updating

✅ Compares mod versions between old and new modpacks  
✅ Removes old versions before downloading new ones  
✅ Preserves user-added mods (not in modpack)  
✅ Handles duplicate mods with complex naming  
✅ Deduplicates entries from mrpack files  

### Database Integration

✅ Updates AstralRinth `app.db` automatically  
✅ Updates ModrinthApp `app.db` automatically  
✅ Syncs version info and timestamps  
✅ Creates database entries if they don't exist  

### Version Selection

✅ Can download/update to latest version  
✅ Can specify exact version (e.g., `--version 0.0.18`)  
✅ Uses GitHub Releases API for all versions  

### Launcher Support

✅ AstralRinth (with database sync)  
✅ ModrinthApp (with database sync)  
✅ XMCL  
✅ PrismLauncher  
✅ Official Minecraft Launcher  
✅ MultiMC  
✅ Custom paths  

---

## 📚 File Structure

```text
minecraft-installer/
├── src/
│   ├── main.rs                      # minecraft-installer entry point
│   ├── bin/
│   │   └── minecraft-updater.rs     # minecraft-updater entry point
│   ├── launcher_support.rs          # Launcher detection & instance creation
│   ├── updater.rs                   # Update logic & mod comparison
│   ├── installer.rs                 # Minecraft installation
│   ├── download.rs                  # Download management
│   ├── java.rs                      # Java detection & installation
│   ├── directories.rs               # Directory structure
│   └── error.rs                     # Error types
├── .github/
│   └── workflows/
│       └── build.yml                # CI/CD for all platforms
├── Cargo.toml                       # Defines both binaries
├── build.bat / build.sh             # Local build scripts
├── build-all.bat / build-all.sh     # Cross-platform build scripts
├── create-release.bat / create-release.sh  # Release helper scripts
├── electron-integration.js          # Node.js wrapper for Electron
├── BUILD_AND_RELEASE.md            # This guide
└── README.md                        # Project overview
```

---

## 🎓 Important Code Locations

### Fetching Modpack Info

```rust
// launcher_support.rs
pub async fn fetch_modpack_info(&self, modpack_type: &str) -> Result<NahaModpackInfo>
pub async fn fetch_modpack_info_version(&self, modpack_type: &str, target_version: &str) -> Result<NahaModpackInfo>
```

### Updating Mods

```rust
// updater.rs
pub async fn update_instance_mods_version(
    &self,
    instance_path: &Path,
    modpack_type: &str,
    version: Option<&str>,
) -> Result<UpdateResult>
```

### Database Updates

```rust
// updater.rs
async fn update_launcher_database(&self, instance_path: &Path, modpack_info: &NahaModpackInfo) -> Result<()>
```

### Mod Name Normalization

```rust
// updater.rs
fn normalize_mod_name(&self, name: &str) -> String
```

---

## 🔑 Key Technical Details

### Mod Comparison Algorithm

1. **Download latest mrpack** from GitHub Releases
2. **Deduplicate mrpack entries** (fixes duplicate files in mrpack)
3. **Normalize mod names** for comparison
   - Handles complex names like `reinforced-barrels-2.6.1+1.21.1$reinforced-core-4.0.2+1.21.1`
   - Distinguishes between `sodium` and `sodium-extra`
4. **Compare existing vs new mods**
   - Same filename → Skip (already up to date)
   - Different filename, same base → Update (remove old, download new)
   - Not in mrpack → Preserve (user mod)
5. **Clean up duplicates** after processing
6. **Update database** with new version info

### GitHub API Integration

**Endpoints Used:**

```text
GET https://api.github.com/repos/perlytiara/NAHA-Minecraft-Modpacks/releases/latest
GET https://api.github.com/repos/perlytiara/NAHA-Minecraft-Modpacks/releases/tags/NeoForge-0.0.18
```

**Tag Format:**

- Latest: `Latest` (tag name)
- Specific NeoForge: `NeoForge-X.X.XX`
- Specific Fabric: `Fabric-X.X.XX`

### Database Schema (AstralRinth/ModrinthApp)

**Table:** `profiles`

**Key Fields:**

- `path` (VARCHAR) - Profile directory name
- `name` (VARCHAR) - Display name
- `game_version` (VARCHAR) - Minecraft version
- `mod_loader` (VARCHAR) - Loader type (neoforge, fabric)
- `created` (INTEGER) - Creation timestamp (ms)
- `modified` (INTEGER) - Last modified timestamp (ms)
- `install_stage` (VARCHAR) - Installation status

**Update Query:**

```sql
UPDATE profiles 
SET modified = ?, game_version = ? 
WHERE path = ?
```

---

## 🎨 User Experience Flow

### Install Flow

```text
User runs: minecraft-installer --download-neoforge --create-instance

1. Fetch modpack info from GitHub API
2. Download .mrpack file
3. Extract modrinth.index.json
4. Download all mods from CDN
5. Extract config files
6. Detect available launchers
7. Create instance in preferred launcher
8. Copy mods, configs, saves, etc.
9. Set up automodpack configuration
10. Inject into launcher database (AstralRinth/ModrinthApp)
11. ✅ Done! Instance ready to launch
```

### Update Flow

```text
User runs: minecraft-updater interactive --modpack-type neoforge

1. Scan all launchers for instances
2. Group by launcher type
3. User selects launcher → User selects instance
4. Fetch latest modpack info from GitHub API
5. Download new .mrpack file
6. Extract modrinth.index.json
7. Analyze existing mods in instance
8. Build modpack mod name set from new mrpack
9. For each mod in new mrpack:
   - If same filename exists → Skip
   - If different version exists → Remove old, download new
   - If not exists → Download new
10. Preserve mods not in mrpack (user mods)
11. Clean up duplicate mods
12. Update launcher database
13. ✅ Done! Mods updated, user mods preserved
```

---

## 🎁 What This Gives You

### For End Users

- ✅ Easy modpack installation to any launcher
- ✅ Automatic mod updates without losing custom mods
- ✅ Support for all major launchers
- ✅ No manual file copying
- ✅ Database stays in sync

### For Developers/Integrators

- ✅ Two clean executables (installer + updater)
- ✅ JSON output for easy parsing
- ✅ Cross-platform binaries ready to distribute
- ✅ Automated builds via GitHub Actions
- ✅ Complete Electron integration support
- ✅ Well-documented codebase

### For DevOps

- ✅ Automated CI/CD pipeline
- ✅ Multi-platform builds in one click
- ✅ Checksums for security
- ✅ Professional release notes
- ✅ Version tagging workflow
- ✅ Build caching for speed

---

## 📊 Project Stats

- **Languages:** Rust 100%
- **Total Executables:** 2
- **Supported Platforms:** 5 (Windows MSVC, Windows GNU, Linux, macOS Intel, macOS ARM)
- **Supported Launchers:** 7
- **Total Source Files:** 9 Rust modules
- **Lines of Code:** ~4,000+ lines
- **Dependencies:** 20+ crates
- **Build Time:** ~5-10 minutes (all platforms via GitHub Actions)
- **Binary Size:** ~10-12 MB each (compressed: ~3-4 MB)

---

## 🔮 Next Steps

### To Create Your First Release

1. **Test locally:**

   ```bash
   cargo build --release
   cargo test
   ```

2. **Create release:**

   ```bash
   ./create-release.sh 1.0.0
   ```

3. **Monitor build:**

   - Visit: `https://github.com/perlytiara/AstralRinth/actions`
   - Wait 10-15 minutes

4. **Download & test:**

   - Visit: `https://github.com/perlytiara/AstralRinth/releases`
   - Download and test one binary

5. **Announce:**

   - Share with users
   - Update documentation links
   - Celebrate! 🎉

---

## 📖 Documentation

- **[BUILD_AND_RELEASE.md](BUILD_AND_RELEASE.md)** - Complete build and release guide
- **[README.md](README.md)** - Project overview and quick start
- **[QUICK_START.md](QUICK_START.md)** - User quick start guide
- **[UPDATER_GUIDE.md](UPDATER_GUIDE.md)** - Updater-specific documentation
- **[CROSS_COMPILE.md](CROSS_COMPILE.md)** - Cross-compilation setup
- **[RELEASE_SYSTEM.md](RELEASE_SYSTEM.md)** - Automated release system details

---

## ✅ What's Already Done

✅ Two separate executables configured in Cargo.toml  
✅ GitHub Actions workflow for all platforms  
✅ Intelligent mod update system  
✅ Database integration for AstralRinth/ModrinthApp  
✅ Version selection support  
✅ GitHub Releases API integration  
✅ Duplicate mod detection and removal  
✅ User mod preservation  
✅ JSON output for Electron integration  
✅ Build scripts for all platforms  
✅ Release helper scripts  
✅ Comprehensive documentation  

---

## 🎯 System Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                    GitHub Repository                         │
│                  (perlytiara/AstralRinth)                    │
└─────────────────────────────────────────────────────────────┘
                            │
                            ├─ Push tag: v1.0.0
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   GitHub Actions Workflow                    │
│                  (.github/workflows/build.yml)               │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ↓                   ↓                   ↓
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Windows    │    │    Linux     │    │    macOS     │
│   Runners    │    │   Runners    │    │   Runners    │
│              │    │              │    │              │
│  MSVC & GNU  │    │     x64      │    │ Intel & ARM  │
└──────────────┘    └──────────────┘    └──────────────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            ↓
                    Build & Test
                            ↓
        ┌───────────────────┴───────────────────┐
        ↓                                       ↓
┌──────────────────┐                  ┌──────────────────┐
│  installer bins  │                  │  updater bins    │
│  (5 platforms)   │                  │  (5 platforms)   │
└──────────────────┘                  └──────────────────┘
        │                                       │
        └───────────────────┬───────────────────┘
                            ↓
                  Create Checksums
                            ↓
                  ┌──────────────────┐
                  │  GitHub Release  │
                  │   10 binaries    │
                  │  10 checksums    │
                  │  Release notes   │
                  └──────────────────┘
                            ↓
                    ┌──────────────┐
                    │    Users     │
                    │   Download   │
                    └──────────────┘
```

---

## 🎪 Real-World Usage Example

### Scenario: Electron App Integration

```javascript
// main.js (Electron main process)
const path = require('path')
const { spawn } = require('child_process')

class MinecraftManager {
  constructor() {
    const platform = process.platform
    const ext = platform === 'win32' ? '.exe' : ''
    const binDir = path.join(__dirname, 'binaries', platform)
    
    this.installerPath = path.join(binDir, `minecraft-installer${ext}`)
    this.updaterPath = path.join(binDir, `minecraft-updater${ext}`)
  }

  // Scan for instances
  async scanInstances() {
    return new Promise((resolve, reject) => {
      const proc = spawn(this.updaterPath, ['scan', '--format', 'json'])
      let output = ''
      
      proc.stdout.on('data', data => output += data)
      proc.on('close', code => {
        if (code === 0) {
          resolve(JSON.parse(output))
        } else {
          reject(new Error(`Scan failed with code ${code}`))
        }
      })
    })
  }

  // Update instance
  async updateInstance(instancePath, modpackType, version = null) {
    return new Promise((resolve, reject) => {
      const args = [
        'update',
        '--instance-path', instancePath,
        '--modpack-type', modpackType,
        '--format', 'json'
      ]
      
      if (version) {
        args.push('--version', version)
      }
      
      const proc = spawn(this.updaterPath, args)
      let output = ''
      
      proc.stdout.on('data', data => output += data)
      proc.on('close', code => {
        if (code === 0) {
          resolve(JSON.parse(output))
        } else {
          reject(new Error(`Update failed with code ${code}`))
        }
      })
    })
  }

  // Install new modpack
  async installModpack(modpackType, targetLauncher = null) {
    return new Promise((resolve, reject) => {
      const args = [
        `--download-${modpackType}`,
        '--create-instance'
      ]
      
      if (targetLauncher) {
        args.push('--target-launcher', targetLauncher)
      }
      
      const proc = spawn(this.installerPath, args)
      
      proc.on('close', code => {
        if (code === 0) {
          resolve({ success: true })
        } else {
          reject(new Error(`Installation failed with code ${code}`))
        }
      })
    })
  }
}

module.exports = MinecraftManager
```

---

## 🎯 Summary for Cursor AI

**Task:** Create multi-platform builds of two Rust executables with automated GitHub releases.

**Solution Implemented:**

1. **Cargo.toml Configuration:**

   - Defined two binary targets: `minecraft-installer` and `minecraft-updater`
   - Both share the same library code (`src/lib.rs`)

2. **GitHub Actions Workflow:**

   - Builds both executables for 5 platforms (10 binaries total)
   - Runs tests on all platforms
   - Creates SHA256 checksums
   - Publishes GitHub release with professional notes

3. **Build Scripts:**

   - Local: `build.bat` / `build.sh` - Build for current platform
   - Cross: `build-all.bat` / `build-all.sh` - Build for all platforms
   - Release: `create-release.bat` / `create-release.sh` - Tag and trigger CI/CD

4. **Integration:**
   - JSON output for Electron apps
   - Node.js wrapper provided (`electron-integration.js`)
   - Database sync for launcher integration
   - Version selection support

**To Deploy:**

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

GitHub Actions handles the rest! 🚀
