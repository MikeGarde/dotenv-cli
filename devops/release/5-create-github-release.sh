#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

DIST_DIR="$REPO_ROOT/dist/release"
VERSION="${RELEASE_VERSION:-$(package_version)}"
TAG_NAME="$VERSION"
TARGET_COMMITISH="${RELEASE_TARGET:-$(current_branch)}"
NOTES_FILE="$DIST_DIR/release-notes.md"
TITLE_FILE="$DIST_DIR/release-notes-title.txt"
ASSET_PATTERNS=(
  "$DIST_DIR"/*.tar.gz
  "$DIST_DIR"/*.tar.xz
  "$DIST_DIR"/*.tar.zst
  "$DIST_DIR"/*.zip
)

assets=()
for asset in "${ASSET_PATTERNS[@]}"; do
  if [ -f "$asset" ]; then
    assets+=("$asset")
  fi
done

if [ "${#assets[@]}" -eq 0 ]; then
  echo 'No release assets found. Run task release:assets first.' >&2
  exit 1
fi

upload_args=(
  release upload "$TAG_NAME"
  "${assets[@]}"
)

if [ "${RELEASE_CLOBBER_ASSETS:-false}" = 'true' ]; then
  upload_args+=(--clobber)
fi

if gh release view "$TAG_NAME" >/dev/null 2>&1; then
  if [ "${RELEASE_CLOBBER_ASSETS:-false}" != 'true' ]; then
    mapfile -t existing_assets < <(gh release view "$TAG_NAME" --json assets --jq '.assets[].name')
    missing_assets=()

    for asset in "${assets[@]}"; do
      local_asset_name="$(basename "$asset")"
      already_uploaded=false

      for existing_asset in "${existing_assets[@]}"; do
        if [ "$local_asset_name" = "$existing_asset" ]; then
          already_uploaded=true
          break
        fi
      done

      if [ "$already_uploaded" = false ]; then
        missing_assets+=("$asset")
      fi
    done

    if [ "${#missing_assets[@]}" -eq 0 ]; then
      echo "Release $TAG_NAME already has all local assets."
      exit 0
    fi

    upload_args=(
      release upload "$TAG_NAME"
      "${missing_assets[@]}"
    )
  fi

  gh "${upload_args[@]}"
  exit 0
fi

if [ ! -f "$NOTES_FILE" ] || [ ! -f "$TITLE_FILE" ]; then
  echo 'Release notes are missing. Run devops/release/4-generate-release-notes.sh first.' >&2
  exit 1
fi

args=(
  release create "$TAG_NAME"
  "${assets[@]}"
  --title "$(cat "$TITLE_FILE")"
  --notes-file "$NOTES_FILE"
  --target "$TARGET_COMMITISH"
)

if [ "$TARGET_COMMITISH" = 'develop' ] || [ "${PRERELEASE:-false}" = 'true' ]; then
  args+=(--prerelease)
fi

if [ "${RELEASE_DRAFT:-true}" != 'false' ]; then
  args+=(--draft)
fi

gh "${args[@]}"
