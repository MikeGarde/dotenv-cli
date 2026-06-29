#!/usr/bin/env bash

set -euo pipefail

TAG="${1:?usage: publish-if-complete.sh <tag>}"

EXPECTED=(
  "dotenv-cli-${TAG}-apple-darwin-aarch64.tar.gz"
  "dotenv-cli-${TAG}-apple-darwin-x86_64.tar.gz"
  "dotenv-cli-${TAG}-unknown-linux-gnu-aarch64.tar.gz"
  "dotenv-cli-${TAG}-unknown-linux-gnu-x86_64.tar.gz"
  "dotenv-cli-${TAG}-unknown-linux-musl-aarch64.tar.gz"
  "dotenv-cli-${TAG}-unknown-linux-musl-x86_64.tar.gz"
  "dotenv-cli-${TAG}-pc-windows-gnu-x86_64.tar.gz"
)

ACTUAL="$(gh release view "${TAG}" --json assets --jq '[.assets[].name]')"

for asset in "${EXPECTED[@]}"; do
  if ! printf '%s' "${ACTUAL}" | jq -e --arg n "${asset}" 'any(.[]; . == $n)' > /dev/null 2>&1; then
    echo "Release not yet complete (missing: ${asset}). Skipping publish."
    exit 0
  fi
done

IS_PRERELEASE="$(gh release view "${TAG}" --json isPrerelease --jq '.isPrerelease' 2>/dev/null || echo false)"

echo "All expected assets present. Publishing release ${TAG}..."
gh release edit "${TAG}" --draft=false
echo "Published."

# Prereleases (develop line) publish the GitHub release but skip the npm and
# Homebrew steps that target stable consumers.
if [[ "${IS_PRERELEASE}" == "true" ]]; then
  echo "Release ${TAG} is a prerelease; skipping npm publish and Homebrew tap update."
  exit 0
fi

# Signal callers (the release-assets workflow) that this run published the
# release so the npm / GitHub Packages publish steps run exactly once.
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "published=true" >> "${GITHUB_OUTPUT}"
fi

echo "Triggering Homebrew tap formula update..."
if GH_TOKEN="${HOMEBREW_TAP_TOKEN:-${GH_TOKEN:-}}" gh workflow run update-formula.yml \
  --repo MikeGarde/homebrew-tap \
  -f formula=dotenv-cli \
  -f repo=MikeGarde/dotenv-cli \
  -f tag="${TAG}"; then
  echo "Homebrew tap update dispatched."
else
  echo "Warning: could not dispatch Homebrew tap update. Trigger it manually with:"
  echo "  gh workflow run update-formula.yml --repo MikeGarde/homebrew-tap -f formula=dotenv-cli -f repo=MikeGarde/dotenv-cli -f tag=${TAG}"
fi
