#!/usr/bin/env bash

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
  'Windows|Intel|x86_64-pc-windows-msvc|pc-windows-msvc|x86_64|dotenv.exe'
  'Windows|Arm|aarch64-pc-windows-msvc|pc-windows-msvc|aarch64|dotenv.exe'
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
