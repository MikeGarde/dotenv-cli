#!/usr/bin/env bash
# Idempotent release asset build, package, and upload.
# Usage: release.sh [--macos|--linux|--windows|--no-macos|--all|<rust-target>...]
#
# Environment variables:
#   RELEASE_VERSION    Override the version (defaults to package.json)
#   RELEASE_TARGET     Git commit/branch for the release target (defaults to current branch)
#   RELEASE_REPO       GitHub repo (defaults to current repo)
#   PRERELEASE         Set to 'true' to mark the release as a prerelease
#   CARGO_BUILD_CMD    Override the cargo build command (e.g. 'cargo zigbuild')
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

command -v gh >/dev/null 2>&1 || { echo 'gh CLI is required.' >&2; exit 1; }

DIST_DIR="$REPO_ROOT/dist/release"
VERSION="${RELEASE_VERSION:-$(package_version)}"
TAG="$VERSION"
TARGET_COMMITISH="${RELEASE_TARGET:-$(current_branch)}"
REPO="${RELEASE_REPO:-$(github_repo)}"
read -r -a BUILD_CMD <<< "${CARGO_BUILD_CMD:-cargo build}"

mkdir -p "$DIST_DIR"

# --- Step 1: Ensure draft release exists ---
if ! gh release view "$TAG" --repo "$REPO" >/dev/null 2>&1; then
  echo "Creating draft release $TAG..."

  RELEASE_VERSION="$VERSION" \
  RELEASE_TARGET="$TARGET_COMMITISH" \
  RELEASE_REPO="$REPO" \
    "$SCRIPT_DIR/3-generate-release-notes.sh"

  create_args=(
    release create "$TAG"
    --draft
    --title "$(cat "$DIST_DIR/release-notes-title.txt")"
    --notes-file "$DIST_DIR/release-notes.md"
    --target "$TARGET_COMMITISH"
    --repo "$REPO"
  )

  if [ "$TARGET_COMMITISH" = 'develop' ] || [ "${PRERELEASE:-false}" = 'true' ]; then
    create_args+=(--prerelease)
  fi

  if ! gh "${create_args[@]}"; then
    # Handle race: another runner may have created the release first
    if gh release view "$TAG" --repo "$REPO" >/dev/null 2>&1; then
      echo "Draft release created by a concurrent process, continuing..."
    else
      echo "Failed to create draft release $TAG." >&2
      exit 1
    fi
  else
    echo "Draft release $TAG created."
  fi
else
  echo "Release $TAG already exists, skipping creation."
fi

# --- Step 2: Build, package, and upload each selected target ---

get_uploaded() {
  gh release view "$TAG" --repo "$REPO" --json assets --jq '.assets[].name' 2>/dev/null || true
}

any_failure=false

while IFS='|' read -r _os _cpu rust_target platform arch executable; do
  asset_base="$(asset_name "$VERSION" "$platform" "$arch")"
  asset_path="$DIST_DIR/$asset_base"

  if get_uploaded | grep -qx "$asset_base"; then
    echo "Skipping $asset_base (already uploaded)"
    continue
  fi

  echo "Building $rust_target..."
  if ! "${BUILD_CMD[@]}" --release --target "$rust_target"; then
    echo "WARNING: Build failed for $rust_target, skipping." >&2
    any_failure=true
    continue
  fi

  binary_dir="$REPO_ROOT/target/$rust_target/release"
  tar -czf "$asset_path" -C "$binary_dir" "$executable"
  echo "Packaged $asset_base"

  if ! gh release upload "$TAG" "$asset_path" --repo "$REPO"; then
    echo "WARNING: Upload failed for $asset_base." >&2
    any_failure=true
    continue
  fi

  echo "Uploaded $asset_base"
done < <(selected_targets "$@")

# --- Step 2b: Dispatch GitHub Actions for the non-macOS targets ---
# macOS binaries are built locally; Linux and Windows assets are produced by the
# release-assets workflow. Only dispatch from the macOS-only local release path.
if [ "${1:-}" = '--macos' ] || [ "${DISPATCH_RELEASE_ASSETS:-false}" = 'true' ]; then
  echo "Dispatching release-assets workflow for Linux and Windows builds..."
  if gh workflow run release-assets.yml \
    --repo "$REPO" \
    --ref "$TARGET_COMMITISH" \
    -f tag="$TAG" \
    -f source_run_url="local:$(hostname)"; then
    echo "Dispatched. Linux and Windows assets will be uploaded by GitHub Actions,"
    echo "which will publish the release and update the Homebrew tap once complete."
  else
    echo "WARNING: could not dispatch release-assets workflow. Trigger it manually with:" >&2
    echo "  gh workflow run release-assets.yml --ref $TARGET_COMMITISH -f tag=$TAG" >&2
    any_failure=true
  fi
fi

# --- Step 3: Publish the release if all expected assets are present ---
RELEASE_VERSION="$VERSION" RELEASE_REPO="$REPO" "$SCRIPT_DIR/finalize-release.sh"

if [ "$any_failure" = true ]; then
  echo "WARNING: Some builds or uploads failed. Re-run this script to retry missing assets." >&2
  exit 1
fi
