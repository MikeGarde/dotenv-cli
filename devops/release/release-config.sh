#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

PACKAGE_NAME='@mikegarde/dotenv-cli'
BINARY_NAME='dotenv'
OS_ORDER=('macOS' 'Linux glibc' 'Linux static' 'Windows')
TARGET_ROWS=(
  'macOS|Intel|x86_64-apple-darwin|apple-darwin|x86_64|dotenv'
  'macOS|Arm|aarch64-apple-darwin|apple-darwin|aarch64|dotenv'
  'Linux glibc|Intel|x86_64-unknown-linux-gnu|unknown-linux-gnu|x86_64|dotenv'
  'Linux glibc|Arm|aarch64-unknown-linux-gnu|unknown-linux-gnu|aarch64|dotenv'
  'Linux static|Intel|x86_64-unknown-linux-musl|unknown-linux-musl|x86_64|dotenv'
  'Linux static|Arm|aarch64-unknown-linux-musl|unknown-linux-musl|aarch64|dotenv'
  'Windows|Intel|x86_64-pc-windows-gnu|pc-windows-gnu|x86_64|dotenv.exe'
)

require_bash_5() {
  if [ -z "${BASH_VERSION:-}" ] || [ "${BASH_VERSINFO[0]}" -lt 5 ]; then
    echo 'Release scripts require Bash 5.x or newer.' >&2
    echo 'On macOS, install modern Bash with Homebrew and run these scripts with that bash.' >&2
    exit 1
  fi
}

release_targets() {
  printf '%s\n' "${TARGET_ROWS[@]}"
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

selected_targets() {
  case "${1:-}" in
    --all)
      release_targets
      return
      ;;
    --macos)
      release_targets | awk -F'|' '$4 == "apple-darwin" { print }'
      return
      ;;
    --linux)
      release_targets | awk -F'|' '$4 ~ /linux/ { print }'
      return
      ;;
    --windows)
      release_targets | awk -F'|' '$4 ~ /windows/ { print }'
      return
      ;;
    --no-macos)
      release_targets | awk -F'|' '$4 != "apple-darwin" { print }'
      return
      ;;
  esac

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
    MINGW*|MSYS*|CYGWIN*) platform='pc-windows-gnu' ;;
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

asset_name() {
  local version="$1"
  local platform="$2"
  local arch="$3"

  printf 'dotenv-cli-%s-%s-%s.tar.gz\n' "$version" "$platform" "$arch"
}

package_version() {
  sed -n 's/.*"version": "\([^"]*\)".*/\1/p' package.json | head -n 1
}

current_branch() {
  git rev-parse --abbrev-ref HEAD
}

github_repo() {
  gh repo view --json nameWithOwner --jq '.nameWithOwner'
}

require_bash_5
