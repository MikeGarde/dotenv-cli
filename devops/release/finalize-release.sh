#!/usr/bin/env bash
# Publishes the GitHub draft release once all expected assets have been uploaded.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

command -v gh >/dev/null 2>&1 || { echo 'gh CLI is required.' >&2; exit 1; }

VERSION="${RELEASE_VERSION:-$(package_version)}"
TAG="$VERSION"
REPO="${RELEASE_REPO:-$(github_repo)}"
HOMEBREW_FORMULA="${HOMEBREW_FORMULA:-dotenv-cli}"
HOMEBREW_TAP_REPO="${HOMEBREW_TAP_REPO:-MikeGarde/homebrew-tap}"

# Record whether this run actually flipped the draft to published so callers
# (e.g. the release-assets workflow) can run follow-on publish steps once.
signal_published() {
  if [ -n "${GITHUB_OUTPUT:-}" ]; then
    echo 'published=true' >> "$GITHUB_OUTPUT"
  fi
}

trigger_homebrew_update() {
  # Stable releases only; prereleases must not bump the published formula.
  local is_prerelease
  is_prerelease="$(gh release view "$TAG" --repo "$REPO" --json isPrerelease --jq '.isPrerelease')"
  if [ "$is_prerelease" = 'true' ]; then
    echo "Release $TAG is a prerelease; skipping Homebrew tap update."
    return
  fi

  echo "Triggering Homebrew tap formula update..."
  if GH_TOKEN="${HOMEBREW_TAP_TOKEN:-${GH_TOKEN:-}}" gh workflow run update-formula.yml \
    --repo "$HOMEBREW_TAP_REPO" \
    -f formula="$HOMEBREW_FORMULA" \
    -f repo="$REPO" \
    -f tag="$TAG"; then
    echo "Homebrew tap update dispatched."
  else
    echo "Warning: could not dispatch Homebrew tap update. Trigger it manually with:" >&2
    echo "  gh workflow run update-formula.yml --repo $HOMEBREW_TAP_REPO -f formula=$HOMEBREW_FORMULA -f repo=$REPO -f tag=$TAG" >&2
  fi
}

if ! gh release view "$TAG" --repo "$REPO" >/dev/null 2>&1; then
  echo "Release $TAG does not exist yet; nothing to finalize."
  exit 0
fi

is_draft="$(gh release view "$TAG" --repo "$REPO" --json isDraft --jq '.isDraft')"
if [ "$is_draft" != 'true' ]; then
  echo "Release $TAG is already published."
  exit 0
fi

mapfile -t uploaded < <(
  gh release view "$TAG" --repo "$REPO" --json assets --jq '.assets[].name' 2>/dev/null || true
)

missing=()
for target_row in "${TARGET_ROWS[@]}"; do
  IFS='|' read -r _os _cpu _rust_target platform arch _exe <<< "$target_row"
  expected="$(asset_name "$VERSION" "$platform" "$arch")"
  if ! printf '%s\n' "${uploaded[@]}" | grep -qx "$expected"; then
    missing+=("$expected")
  fi
done

if [ "${#missing[@]}" -gt 0 ]; then
  echo "Release $TAG is still a draft; waiting on ${#missing[@]} asset(s):"
  printf '  - %s\n' "${missing[@]}"
  exit 0
fi

echo "All assets uploaded. Publishing release $TAG..."
gh release edit "$TAG" --repo "$REPO" --draft=false
echo "Release $TAG is now published."

signal_published
trigger_homebrew_update
