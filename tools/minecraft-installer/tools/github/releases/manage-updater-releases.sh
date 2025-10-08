#!/bin/bash

# GitHub Release Management Script for Minecraft Updater
# This script helps delete all tags and releases, then create a new one

set -e

REPO_OWNER="perlytiara"
REPO_NAME="minecraft-updater"
GITHUB_REPO="$REPO_OWNER/$REPO_NAME"

echo "🚀 GitHub Release Management for $GITHUB_REPO"
echo "=============================================="

# Function to delete all releases
delete_all_releases() {
  echo ""
  echo "🗑️  Fetching all releases..."

  # Get all release IDs
  RELEASE_IDS=$(gh release list --repo "$GITHUB_REPO" --limit 1000 --json id --jq '.[].id' 2>/dev/null || echo "")

  if [ -z "$RELEASE_IDS" ]; then
    echo "✅ No releases found to delete."
    return
  fi

  echo "Found releases. Deleting..."

  # Delete each release
  while IFS= read -r release_id; do
    if [ -n "$release_id" ]; then
      echo "  Deleting release ID: $release_id"
      gh api -X DELETE "/repos/$GITHUB_REPO/releases/$release_id" 2>/dev/null || echo "  Failed to delete release $release_id"
    fi
  done <<< "$RELEASE_IDS"

  echo "✅ All releases deleted!"
}

# Function to delete all tags
delete_all_tags() {
  echo ""
  echo "🗑️  Fetching all tags..."

  # Get all tags
  TAGS=$(git tag -l)

  if [ -z "$TAGS" ]; then
    echo "✅ No tags found to delete."
    return
  fi

  echo "Found tags. Deleting locally and from remote..."

  # Delete each tag locally and remotely
  while IFS= read -r tag; do
    if [ -n "$tag" ]; then
      echo "  Deleting tag: $tag"
      git tag -d "$tag" 2>/dev/null || echo "  Failed to delete local tag $tag"
      git push origin --delete "$tag" 2>/dev/null || echo "  Failed to delete remote tag $tag"
    fi
  done <<< "$TAGS"

  echo "✅ All tags deleted!"
}

# Function to create a new release
create_release() {
  local version="$1"
  local title="$2"
  local notes_file="$3"

  echo ""
  echo "📦 Creating new release v$version..."

  # Create and push tag
  echo "  Creating tag v$version..."
  git tag -a "v$version" -m "$title"
  git push origin "v$version"

  # Create release
  echo "  Creating GitHub release..."
  if [ -f "$notes_file" ]; then
    gh release create "v$version" \
      --title "$title" \
      --notes-file "$notes_file" \
      --repo "$GITHUB_REPO"
  else
    gh release create "v$version" \
      --title "$title" \
      --notes "Release v$version" \
      --repo "$GITHUB_REPO"
  fi

  echo "✅ Release v$version created successfully!"
}

# Main menu
case "${1:-}" in
  delete-releases)
    delete_all_releases
    ;;
  delete-tags)
    delete_all_tags
    ;;
  delete-all)
    delete_all_releases
    delete_all_tags
    ;;
  create)
    if [ $# -lt 2 ]; then
      echo "Usage: $0 create <version> [title] [notes-file]"
      exit 1
    fi
    VERSION="${2}"
    TITLE="${3:-Minecraft Updater v$VERSION}"
    NOTES_FILE="${4:-}"
    create_release "$VERSION" "$TITLE" "$NOTES_FILE"
    ;;
  reset-and-create)
    if [ $# -lt 2 ]; then
      echo "Usage: $0 reset-and-create <version> [title] [notes-file]"
      exit 1
    fi
    VERSION="${2}"
    TITLE="${3:-Minecraft Updater v$VERSION}"
    NOTES_FILE="${4:-}"

    echo "⚠️  WARNING: This will delete ALL existing releases and tags!"
    echo "Then create a new release v$VERSION"
    read -p "Continue? (yes/no): " confirm

    if [ "$confirm" = "yes" ]; then
      delete_all_releases
      delete_all_tags
      create_release "$VERSION" "$TITLE" "$NOTES_FILE"
    else
      echo "Cancelled."
      exit 0
    fi
    ;;
  *)
    echo "Usage: $0 {delete-releases|delete-tags|delete-all|create|reset-and-create} [args]"
    echo ""
    echo "Commands:"
    echo "  delete-releases              Delete all GitHub releases"
    echo "  delete-tags                  Delete all Git tags (local and remote)"
    echo "  delete-all                   Delete all releases and tags"
    echo "  create <version> [title] [notes-file]"
    echo "                              Create a new release"
    echo "  reset-and-create <version> [title] [notes-file]"
    echo "                              Delete everything and create new release"
    echo ""
    echo "Examples:"
    echo "  $0 delete-all"
    echo "  $0 create 1.0.0 'Minecraft Updater v1.0.0' notes/updater/RELEASE_NOTES_UPDATER_v1.0.0.md"
    echo "  $0 reset-and-create 1.0.0"
    exit 1
    ;;
esac

echo ""
echo "✅ Done!"

