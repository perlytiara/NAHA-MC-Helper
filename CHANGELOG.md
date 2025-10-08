# Changelog

All notable changes to NAHA MC Helper will be documented in this file.

## [1.0.2] - 2025-10-08

### Added

- **Cross-platform binary integration** for Minecraft installer and updater
  - Auto-detects platform (Windows/macOS/Linux) and architecture
  - Supports Windows x64 (MSVC + GNU), macOS (Intel + Apple Silicon), Linux x64
  - Binaries automatically bundled in production builds
- **Binary download script** (`tools/download-minecraft-binaries.cjs`)
  - Downloads all 10 binaries from GitHub releases
  - Smart caching to avoid re-downloading unchanged files
- **Binary utility module** (`src/shared/utils/minecraft-binaries.js`)
  - Unified API for binary execution across all platforms
  - Handles development and production paths automatically
- **Comprehensive integration documentation** (`docs/BINARY_INTEGRATION.md`)

### Changed

- **Completely redesigned Install Modpack page** to match onboarding flow
  - Simplified from multi-step process to 2 centered buttons
  - Clean, centered layout matching onboarding design guidelines
  - Fits entire interface in one window
  - Beautiful gradient backgrounds and animations
  - Removed complex launcher installation flows
  - Focus on core functionality: NeoForge or Fabric modpack installation
- **Updated `main.js`** to use platform-agnostic binary paths
  - Removed hardcoded Windows-specific paths
  - Now works on all platforms without modification
- **Updated `forge.config.cjs`** to bundle all binaries in production
  - Maps `tools/minecraft-installer-releases/` → `resources/binaries/`
  - All platforms included in single build

### Improved

- Better user experience on Install page
- Cleaner, more intuitive UI matching app design system
- Cross-platform compatibility for binary execution
- Development workflow (binaries update without rebuild)

## [1.0.1] - Initial Release - 2025-10-07

### Added (Initial Release)

- Initial release
- Onboarding flow
- Launcher installation
- Modpack download functionality
- Update flow for existing instances
- Auto-updater service

### Features

- Multi-launcher support (AstralRinth, ModrinthApp, XMCL, PrismLauncher)
- GitHub API integration for downloads
- Instance scanning and updating
- Modern UI with Svelte + TailwindCSS + DaisyUI

---

## Version Format

Versions follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality in a backward compatible manner
- **PATCH** version for backward compatible bug fixes
