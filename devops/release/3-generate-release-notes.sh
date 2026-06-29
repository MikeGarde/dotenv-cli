#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

command -v jq >/dev/null 2>&1 || {
  echo 'jq is required to parse GitHub release-note JSON.' >&2
  exit 1
}

DIST_DIR="$REPO_ROOT/dist/release"
TEMPLATE_FILE="$REPO_ROOT/devops/templates/release-notes.md"
VERSION="${RELEASE_VERSION:-$(package_version)}"
TARGET_COMMITISH="${RELEASE_TARGET:-$(current_branch)}"
REPO="${RELEASE_REPO:-$(github_repo)}"
RESPONSE_FILE="$DIST_DIR/release-notes.response.json"
BODY_FILE="$DIST_DIR/release-notes-body.md"
LINK_MATRIX_FILE="$DIST_DIR/release-notes-link-matrix.md"

if [ ! -f "$TEMPLATE_FILE" ]; then
  echo "Release notes template not found: $TEMPLATE_FILE" >&2
  exit 1
fi

mkdir -p "$DIST_DIR"

args=(
  "repos/$REPO/releases/generate-notes"
  --method POST
  -f "tag_name=$VERSION"
  -f "target_commitish=$TARGET_COMMITISH"
)

if [ -n "${PREVIOUS_TAG:-}" ]; then
  args+=(-f "previous_tag_name=$PREVIOUS_TAG")
fi

gh api "${args[@]}" > "$RESPONSE_FILE"

jq -r '.name' "$RESPONSE_FILE" > "$DIST_DIR/release-notes-title.txt"
jq -r '.body' "$RESPONSE_FILE" > "$BODY_FILE"

{
  declare -A intel_links=()
  declare -A arm_links=()

  for target in "${TARGET_ROWS[@]}"; do
    IFS='|' read -r os cpu rust_target platform arch executable <<< "$target"
    name="$(asset_name "$VERSION" "$platform" "$arch")"
    url="https://github.com/$REPO/releases/download/$VERSION/$name"
    link="[$arch]($url)"

    if [ "$cpu" = 'Intel' ]; then
      intel_links["$os"]="$link"
    else
      arm_links["$os"]="$link"
    fi
  done

  for os_name in "${OS_ORDER[@]}"; do
    printf '| %s | %s | %s |\n' \
      "$os_name" \
      "${intel_links[$os_name]:--}" \
      "${arm_links[$os_name]:--}"
  done
} > "$LINK_MATRIX_FILE"

awk \
  -v notes_file="$BODY_FILE" \
  -v link_matrix_file="$LINK_MATRIX_FILE" \
  '
    $0 == "{NOTES}" {
      while ((getline line < notes_file) > 0) print line
      close(notes_file)
      next
    }

    $0 == "{LINK_MATRIX}" {
      while ((getline line < link_matrix_file) > 0) print line
      close(link_matrix_file)
      next
    }

    { print }
  ' "$TEMPLATE_FILE" > "$DIST_DIR/release-notes.md"

perl -pi -e "s/\{VERSION\}/${VERSION}/g" "$DIST_DIR/release-notes.md"

echo "Wrote $DIST_DIR/release-notes.md"
echo "Wrote $DIST_DIR/release-notes-title.txt"
