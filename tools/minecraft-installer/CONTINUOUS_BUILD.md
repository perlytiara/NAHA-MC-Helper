# 🔄 Continuous Build System

## Overview

The Minecraft Installer and Updater tools now use a **continuous build system** that automatically builds and releases to the same "latest" tag on every push to the main branch.

## How It Works

### Automatic Builds

Every time you push changes to the `main` branch, the GitHub Actions workflow automatically:

1. **Deletes** the previous "latest" release and tag
2. **Builds** both executables for all 5 platforms:
   - Windows x64 (MSVC)
   - Windows x64 (GNU)
   - Linux x64
   - macOS Intel
   - macOS Apple Silicon (ARM64)
3. **Creates** SHA256 checksums for all binaries
4. **Publishes** a new "latest" release with all binaries

### Always Up-to-Date

- The "latest" tag always points to the most recent build from main
- Users can always download the newest version from the same URL
- Previous builds are automatically replaced (no version clutter)

## Triggering a Build

### Method 1: Push to Main (Automatic)

```bash
git add .
git commit -m "Add new feature"
git push origin main
```

The build starts automatically and completes in ~10-15 minutes.

### Method 2: Manual Trigger

1. Go to: <https://github.com/perlytiara/NAHA-MC-Helper/actions>
2. Click on "Build and Release Minecraft Tools"
3. Click "Run workflow"
4. Select branch: `main`
5. Click "Run workflow"

## Download URLs

The binaries are always available at:

```text
https://github.com/perlytiara/NAHA-MC-Helper/releases/download/latest/minecraft-installer-windows-x86_64.exe
https://github.com/perlytiara/NAHA-MC-Helper/releases/download/latest/minecraft-updater-windows-x86_64.exe
https://github.com/perlytiara/NAHA-MC-Helper/releases/download/latest/minecraft-installer-linux-x86_64
https://github.com/perlytiara/NAHA-MC-Helper/releases/download/latest/minecraft-updater-linux-x86_64
... etc
```

These URLs never change, so you can hardcode them in your Electron app.

## Monitoring Builds

### Check Build Status

1. Go to: <https://github.com/perlytiara/NAHA-MC-Helper/actions>
2. Click on the latest "Build and Release Minecraft Tools" workflow
3. Monitor progress in real-time

### Build Time

- **Parallel builds** across 5 platforms: ~10-15 minutes total
- Each platform builds both binaries simultaneously

### Build Artifacts

Each build produces:

- **10 binary files** (2 executables × 5 platforms)
- **10 checksum files** (SHA256)
- **Release notes** (auto-generated with build date and commit)

## Release Information

### Release Page

Always available at: <https://github.com/perlytiara/NAHA-MC-Helper/releases/tag/latest>

### Release Notes

Each build includes:

- Build timestamp
- Commit SHA
- Full feature list
- Usage examples
- Download links for all platforms

## Benefits

### For Developers

✅ **No manual versioning** - Just push to main
✅ **Automatic builds** - No need to run build scripts locally
✅ **Consistent releases** - Same URL, always updated
✅ **Easy rollback** - Just revert the commit and push

### For Users

✅ **Always latest** - One URL for the newest version
✅ **No confusion** - No multiple versions to choose from
✅ **Automatic updates** - Apps can check for updates easily

### For Integration

✅ **Stable URLs** - Hardcode download URLs in your app
✅ **JSON API** - Use GitHub API to check build info
✅ **Checksums** - Verify downloads programmatically

## Workflow Configuration

The workflow is located at: `.github/workflows/build-minecraft-tools.yml`

### Key Configuration

```yaml
env:
  RELEASE_TAG: latest  # Always use the same tag
```

### Triggers

```yaml
on:
  push:
    branches:
      - main  # Trigger on push to main
  workflow_dispatch:  # Allow manual triggering
```

## Migration from Old System

### Before (Version Tags)

- Created new releases: v1.0.0, v1.0.1, v1.0.2...
- Multiple download URLs
- Users confused about which version to download
- Required manual versioning

### After (Continuous Latest)

- Always one release: "latest"
- Same download URLs
- Users always get the newest version
- Automatic on every push

## Best Practices

### Commit Messages

Since every push creates a release, use clear commit messages:

```bash
git commit -m "fix: resolve mod loading issue"
git commit -m "feat: add support for Quilt loader"
git commit -m "docs: update installation guide"
```

### Testing Before Push

Since builds are automatic, test locally first:

```bash
cd tools/minecraft-installer
cargo test
cargo build --release
# Test the binaries
git push origin main  # Only when ready
```

### Hotfixes

For urgent fixes:

```bash
# Fix the issue
git add .
git commit -m "hotfix: critical installation bug"
git push origin main
# Wait 15 minutes for new build
# Verify the fix in the new release
```

## Troubleshooting

### Build Failed

1. Check the Actions tab: <https://github.com/perlytiara/NAHA-MC-Helper/actions>
2. Click on the failed workflow
3. Read the error logs
4. Fix the issue and push again

### Release Not Created

Check that:

- ✅ GitHub Actions has permission to create releases
- ✅ `GITHUB_TOKEN` is available (automatic)
- ✅ All platform builds succeeded
- ✅ Workflow file exists at `.github/workflows/build-minecraft-tools.yml`

### Old Tag Not Deleted

If the old "latest" tag isn't deleted:

- Delete it manually: `gh release delete latest --yes`
- Delete the tag: `git push --delete origin latest`
- Re-run the workflow

## Advanced: Checking for Updates

### In Your Electron App

```javascript
const GITHUB_API = 'https://api.github.com/repos/perlytiara/NAHA-MC-Helper/releases/tags/latest'

async function checkForUpdates() {
  const response = await fetch(GITHUB_API)
  const data = await response.json()
  
  const publishedAt = new Date(data.published_at)
  const commitSha = data.target_commitish
  
  console.log('Latest build:', publishedAt)
  console.log('Commit:', commitSha)
  
  return {
    buildDate: publishedAt,
    commit: commitSha,
    assets: data.assets  // All binaries
  }
}
```

### Download Latest Binary

```javascript
const downloadUrl = 'https://github.com/perlytiara/NAHA-MC-Helper/releases/download/latest/minecraft-installer-windows-x86_64.exe'

// This URL never changes and always gives you the latest build
```

## Summary

The continuous build system provides:

- 🔄 **Automatic builds** on every push
- 📦 **Always up-to-date** binaries
- 🔗 **Stable URLs** for downloads
- ⚡ **Fast iterations** without version management
- ✅ **Simple workflow** for developers

Just push to main, and the rest is automatic! 🚀
