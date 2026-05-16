#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

COMMIT=true

case "${1:-}" in
  '')
    ;;
  --no-commit)
    COMMIT=false
    ;;
  -h|--help)
    echo 'Usage: devops/release/2-tag-version.sh [--no-commit]'
    exit 0
    ;;
  *)
    echo 'Usage: devops/release/2-tag-version.sh [--no-commit]' >&2
    exit 1
    ;;
esac

TAG_NAME="$(package_version)"

if [ "$COMMIT" = true ]; then
  git add package.json package-lock.json Cargo.toml Cargo.lock

  if ! git diff --cached --quiet; then
    git commit -m "chore: release $TAG_NAME"
  fi
fi

git tag "$TAG_NAME"
git push --follow-tags
