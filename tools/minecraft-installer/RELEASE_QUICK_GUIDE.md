# 🚀 Quick Release Guide

## One-Command Release

```bash
# Windows
create-release.bat 1.0.0

# Linux/macOS
./create-release.sh 1.0.0
```

That's it! GitHub Actions handles everything else.

---

## What Happens Automatically

### ⏱️ Timeline: ~10-15 minutes total

```text
0:00  ✅ Tag created and pushed
0:01  🔄 GitHub Actions triggered
      
0:02  🏗️  Building on 5 platforms simultaneously:
      ├─ Windows MSVC (minecraft-installer + minecraft-updater)
      ├─ Windows GNU (minecraft-installer + minecraft-updater)
      ├─ Linux x64 (minecraft-installer + minecraft-updater)
      ├─ macOS Intel (minecraft-installer + minecraft-updater)
      └─ macOS ARM (minecraft-installer + minecraft-updater)

0:08  🧪 Running tests on all platforms

0:10  📦 Creating artifacts:
      ├─ Stripping debug symbols
      ├─ Setting permissions
      └─ Uploading to GitHub

0:12  🔐 Generating SHA256 checksums

0:13  📝 Creating release notes

0:14  🎉 Publishing GitHub Release

0:15  ✅ DONE! Release is live
```

---

## What You Get

### GitHub Release Contains

**10 Executables:**

```text
✅ minecraft-installer-windows-x86_64.exe
✅ minecraft-installer-windows-gnu-x86_64.exe
✅ minecraft-installer-linux-x86_64
✅ minecraft-installer-macos-intel-x86_64
✅ minecraft-installer-macos-apple-silicon-aarch64

✅ minecraft-updater-windows-x86_64.exe
✅ minecraft-updater-windows-gnu-x86_64.exe
✅ minecraft-updater-linux-x86_64
✅ minecraft-updater-macos-intel-x86_64
✅ minecraft-updater-macos-apple-silicon-aarch64
```

**10 Checksums:**

```text
✅ [each binary].sha256
```

**Plus:**

- 📝 Auto-generated release notes
- 📊 Feature highlights
- 🚀 Quick start examples
- 📋 Launcher compatibility table
- 🔒 Verification instructions

---

## Quick Test

After release is published:

```bash
# Download for your platform
wget https://github.com/perlytiara/AstralRinth/releases/download/v1.0.0/minecraft-updater-linux-x86_64

# Make executable
chmod +x minecraft-updater-linux-x86_64

# Test it works
./minecraft-updater-linux-x86_64 --version
./minecraft-updater-linux-x86_64 scan --format compact
```

---

## Version Numbers

| Type | Example | When to Use |
|------|---------|-------------|
| Major | `v1.0.0` | Breaking changes |
| Minor | `v0.2.0` | New features (compatible) |
| Patch | `v0.1.1` | Bug fixes only |
| Beta | `v1.0.0-beta.1` | Testing phase |
| RC | `v1.0.0-rc.1` | Release candidate |

---

## That's It

Creating a release is literally:

1. Run `./create-release.sh 1.0.0`
2. Wait 15 minutes
3. Download and test

GitHub Actions does all the heavy lifting! 🎉
