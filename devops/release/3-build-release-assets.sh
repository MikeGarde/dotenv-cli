#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"
source "$SCRIPT_DIR/release-config.sh"

DIST_DIR="$REPO_ROOT/dist/release"
VERSION="${RELEASE_VERSION:-$(package_version)}"

mkdir -p "$DIST_DIR"

selected_targets() {
  if [ "${1:-}" = "--all" ]; then
    release_targets
    return
  fi

  if [ "${1:-}" = "--macos" ]; then
    release_targets | awk -F'|' '$4 == "apple-darwin" { print }'
    return
  fi

  if [ "${1:-}" = "--no-macos" ]; then
    release_targets | awk -F'|' '$4 != "apple-darwin" { print }'
    return
  fi

  if [ -n "${RELEASE_TARGETS:-}" ]; then
    IFS=',' read -r -a requested_targets <<< "$RELEASE_TARGETS"
    targets_matching "${requested_targets[@]}"
    return
  fi

  if [ "$#" -gt 0 ]; then
    targets_matching "$@"
    return
  fi

  local platform arch
  case "$(uname -s)" in
    Darwin) platform='apple-darwin' ;;
    Linux) platform='unknown-linux-gnu' ;;
    MINGW*|MSYS*|CYGWIN*) platform='pc-windows-msvc' ;;
    *) echo "Unsupported platform: $(uname -s)" >&2; exit 1 ;;
  esac

  case "$(uname -m)" in
    x86_64|amd64) arch='x86_64' ;;
    arm64|aarch64) arch='aarch64' ;;
    *) echo "Unsupported architecture: $(uname -m)" >&2; exit 1 ;;
  esac

  release_targets | awk -F'|' -v platform="$platform" -v arch="$arch" \
    '$4 == platform && $5 == arch { print }'
}

targets_matching() {
  local requested rust_target target

  for target in "${TARGET_ROWS[@]}"; do
    IFS='|' read -r _os _cpu rust_target _platform _arch _executable <<< "$target"

    for requested in "$@"; do
      if [[ "$requested" == "$rust_target" ]]; then
        printf '%s\n' "$target"
      fi
    done
  done
}

mapfile -t targets < <(selected_targets "$@")

if [ "${#targets[@]}" -eq 0 ]; then
  echo 'No matching release targets selected.' >&2
  exit 1
fi

for target in "${targets[@]}"; do
  IFS='|' read -r os cpu rust_target platform arch executable <<< "$target"

  echo "Building $rust_target"
  cargo build --release --target "$rust_target"

  binary_dir="$REPO_ROOT/target/$rust_target/release"
  archive_path="$DIST_DIR/$(asset_name "$VERSION" "$platform" "$arch")"

  tar -czf "$archive_path" -C "$binary_dir" "$executable"
  echo "Wrote $archive_path"
done
