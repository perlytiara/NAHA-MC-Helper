#!/bin/bash

# Minecraft Installer Release Script
# Usage: ./create-release.sh [version]
# Example: ./create-release.sh 0.1.0

set -e

# Get version from argument or prompt
if [ -z "$1" ]; then
    echo "Enter version number (e.g., 0.1.0):"
    read -r VERSION
else
    VERSION="$1"
fi

# Validate version format
if [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo "❌ Invalid version format. Use semantic versioning (e.g., 0.1.0, 1.0.0-beta.1)"
    exit 1
fi

echo "🚀 Creating release v$VERSION..."

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo "❌ You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Check if tag already exists
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    echo "❌ Tag v$VERSION already exists"
    exit 1
fi

# Create and push tag
echo "📝 Creating tag v$VERSION..."
git tag -a "v$VERSION" -m "Release v$VERSION"

echo "📤 Pushing tag to remote..."
git push origin "v$VERSION"

echo "✅ Release v$VERSION created successfully!"
echo ""
echo "🎉 GitHub Actions will now automatically:"
echo "   • Build for all platforms (Windows, Linux, macOS)"
echo "   • Run tests"
echo "   • Create a GitHub release with all binaries"
echo "   • Generate checksums for verification"
echo ""
echo "📋 Check the progress at:"
echo "   https://github.com/perlytiara/minecraft-installer/actions"
echo ""
echo "📦 Once complete, download from:"
echo "   https://github.com/perlytiara/minecraft-installer/releases/tag/v$VERSION"








