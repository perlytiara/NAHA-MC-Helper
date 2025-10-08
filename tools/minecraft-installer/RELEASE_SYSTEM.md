# 🚀 Automated Release System

This project uses GitHub Actions to automatically build and release the Minecraft Installer for all major platforms whenever you create a version tag.

## 🎯 How It Works

### 1. Tag-Based Releases

- Create a version tag (e.g., `v0.1.0`, `v1.0.0-beta.1`)
- GitHub Actions automatically triggers the build process
- All platforms are built in parallel
- A GitHub release is created with all binaries

### 2. Supported Platforms

- **Windows**: x64 (MSVC + GNU toolchains)
- **Linux**: x64 + ARM64
- **macOS**: Intel + Apple Silicon

### 3. Automatic Features

- ✅ Cross-platform compilation
- ✅ Running tests on all platforms
- ✅ Binary optimization (stripping)
- ✅ SHA256 checksums for all files
- ✅ Professional release notes
- ✅ Caching for faster builds

## 🛠️ Creating a Release

### Option 1: Using the Release Scripts

**Windows:**

```cmd
create-release.bat 0.1.0
```

**Linux/macOS:**

```bash
chmod +x create-release.sh
./create-release.sh 0.1.0
```

### Option 2: Manual Git Commands

```bash
# 1. Make sure all changes are committed
git add .
git commit -m "Prepare for release v0.1.0"

# 2. Create and push the tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Option 3: GitHub Web Interface

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Choose "Create a new tag" and enter `v0.1.0`
4. Add release title and description
5. Click "Publish release"

## 📋 Version Naming Convention

Use [Semantic Versioning](https://semver.org/):

- **Major.Minor.Patch**: `1.0.0`, `0.1.0`
- **Pre-release**: `1.0.0-beta.1`, `0.1.0-alpha.2`
- **Build metadata**: `1.0.0+20240101`

### Examples

- `v0.1.0` - First stable release
- `v1.0.0` - Major stable release
- `v0.2.0-beta.1` - Beta release
- `v1.1.0-rc.1` - Release candidate

## 🎉 What Happens After You Create a Tag

### 1. Build Process (5-10 minutes)

```text
🔄 Building for Windows x64 (MSVC)...
🔄 Building for Windows x64 (GNU)...
🔄 Building for Linux x64...
🔄 Building for Linux ARM64...
🔄 Building for macOS Intel...
🔄 Building for macOS Apple Silicon...
```

### 2. Testing

- All builds run the test suite
- Ensures quality across platforms

### 3. Release Creation

- Downloads all built binaries
- Generates SHA256 checksums
- Creates professional release notes
- Publishes to GitHub Releases

### 4. Final Result

You get a GitHub release with:

- 6 platform-specific binaries
- SHA256 checksums for verification
- Detailed release notes
- Download statistics

## 📦 Release Artifacts

Each release includes:

### Windows

- `minecraft-installer-windows-x86_64.exe` (MSVC)
- `minecraft-installer-windows-gnu-x86_64.exe` (GNU)

### Linux

- `minecraft-installer-linux-x86_64` (Intel/AMD)
- `minecraft-installer-linux-aarch64` (ARM64)

### macOS

- `minecraft-installer-macos-intel-x86_64` (Intel Macs)
- `minecraft-installer-macos-apple-silicon-aarch64` (M1/M2 Macs)

## 🔍 Monitoring Builds

### Check Build Status

1. Go to: <https://github.com/perlytiara/minecraft-installer/actions>
2. Click on your release build
3. Monitor progress in real-time

### Build Logs

- Each platform shows detailed build logs
- Test results are visible
- Any failures are clearly marked

## 🚨 Troubleshooting

### Build Failures

If a build fails:

1. Check the build logs in GitHub Actions
2. Common issues:
   - Dependency installation problems
   - Rust toolchain issues
   - Platform-specific compilation errors

### Missing Binaries

- All platforms must build successfully for the release to be created
- If one platform fails, the entire release is cancelled
- Fix the issue and create a new tag

### Release Not Created

- Ensure the tag starts with `v` (e.g., `v0.1.0`)
- Check that GitHub Actions has permission to create releases
- Verify the workflow file is in `.github/workflows/`

## 🎯 Best Practices

### Before Creating a Release

1. **Test locally** on your platform
2. **Update documentation** if needed
3. **Commit all changes**
4. **Check for breaking changes**

### Release Notes

The system auto-generates release notes, but you can customize them by:

1. Editing the release after it's created
2. Modifying the workflow template
3. Adding a `CHANGELOG.md` file

### Version Strategy

- **Patch** (0.1.1): Bug fixes, no new features
- **Minor** (0.2.0): New features, backward compatible
- **Major** (1.0.0): Breaking changes, major features

## 🔧 Customization

### Adding New Platforms

Edit `.github/workflows/build.yml` and add to the matrix:

```yaml
- os: ubuntu-latest
  target: x86_64-unknown-linux-musl
  artifact_name: minecraft-installer-linux-musl-x86_64
  binary_name: minecraft-installer
```

### Modifying Release Notes

Edit the `release_notes` step in the workflow to customize the template.

### Adding Build Steps

Add new steps before the "Build" step:

```yaml
- name: Custom step
  run: echo "Do something custom"
```

## 📊 Benefits

✅ **Zero Setup**: No need to install cross-compilation tools  
✅ **Consistent**: Same build environment every time  
✅ **Fast**: Parallel builds across all platforms  
✅ **Reliable**: Automated testing and validation  
✅ **Professional**: Clean releases with checksums  
✅ **Scalable**: Easy to add new platforms  

---

**Ready to create your first release?** Run `./create-release.sh 0.1.0` and watch the magic happen! 🎉
