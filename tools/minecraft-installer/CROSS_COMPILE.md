# Cross-Compilation Setup Guide

This guide explains how to set up cross-compilation for the minecraft-installer to build for multiple platforms.

## Current Status

✅ **Windows (x86_64-pc-windows-msvc)**: Works out of the box  
❌ **Windows GNU**: Requires MinGW-w64  
❌ **Linux**: Requires OpenSSL cross-compilation setup  
❌ **macOS**: Requires Xcode command line tools  

## Windows Build (Native)

The Windows build works without any additional setup:

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

## Cross-Compilation Setup

### For Linux (x86_64-unknown-linux-gnu)

#### Option 1: Use Docker (Recommended)

```bash
# Create a Dockerfile for Linux builds
docker run --rm -v "$(pwd)":/workspace -w /workspace rust:latest cargo build --release --target x86_64-unknown-linux-gnu
```

#### Option 2: Manual Setup

1. Install WSL2 with Ubuntu
2. Install Rust and cross-compilation tools in WSL
3. Build from WSL environment

### For macOS (x86_64-apple-darwin / aarch64-apple-darwin)

#### Option 1: Use GitHub Actions (Recommended)

- Set up GitHub Actions workflow with macOS runners
- Build automatically on push/PR

#### Option 2: Manual Setup (macOS)

1. Install Xcode command line tools
2. Install cross-compilation toolchain
3. Set up proper environment variables

### For Windows GNU (x86_64-pc-windows-gnu)

1. Install MinGW-w64
2. Add to PATH
3. Set up cross-compilation environment

## Build Scripts

- `build-simple.bat`: Builds only Windows (guaranteed to work)
- `build-all.bat`: Attempts all platforms (may fail without proper setup)

## Dependencies

The minecraft-installer requires:

- **OpenSSL**: For HTTPS requests and crypto operations
- **SQLite**: For database operations (bundled with rusqlite)
- **System libraries**: Platform-specific system calls

## Alternative: GitHub Actions

For reliable cross-platform builds, consider using GitHub Actions:

```yaml
name: Build
on: [push, pull_request]
jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      - run: cargo build --release --target ${{ matrix.target }}
```

## Current Build Output

The Windows build produces:

- `minecraft-installer-windows-x86_64.exe` (10.4 MB)
- Supports all launcher types including the new "Other" custom path launcher
- Includes all features: mrpack installation, API downloads, automodpack setup
