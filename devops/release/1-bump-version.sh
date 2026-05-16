#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

STEP="${1:-}"

case "$STEP" in
  patch|minor|major|[0-9]*.[0-9]*.[0-9]*) ;;
  *)
    echo 'Usage: devops/release/1-bump-version.sh <patch|minor|major|x.y.z>' >&2
    exit 1
    ;;
esac

CURRENT_VERSION="$(package_version)"

if printf '%s\n' "$STEP" | grep -Eq '^[v]?[0-9]+\.[0-9]+\.[0-9]+'; then
  NEXT_VERSION="${STEP#v}"
else
  major="$(printf '%s\n' "$CURRENT_VERSION" | cut -d. -f1)"
  minor="$(printf '%s\n' "$CURRENT_VERSION" | cut -d. -f2)"
  patch="$(printf '%s\n' "$CURRENT_VERSION" | cut -d. -f3)"

  case "$STEP" in
    major) NEXT_VERSION="$((major + 1)).0.0" ;;
    minor) NEXT_VERSION="$major.$((minor + 1)).0" ;;
    patch) NEXT_VERSION="$major.$minor.$((patch + 1))" ;;
  esac
fi

awk -v version="$NEXT_VERSION" '
  !done && /"version": "/ {
    sub(/"version": "[^"]*"/, "\"version\": \"" version "\"")
    done = 1
  }
  { print }
' package.json > package.json.tmp
mv package.json.tmp package.json

awk -v version="$NEXT_VERSION" '
  !done && /^version = "/ {
    print "version = \"" version "\""
    done = 1
    next
  }
  { print }
' Cargo.toml > Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml

echo "$NEXT_VERSION"
