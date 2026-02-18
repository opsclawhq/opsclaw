#!/usr/bin/env bash
set -euo pipefail

detect_target() {
  local os
  local arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os/$arch" in
    Darwin/x86_64)
      echo "x86_64-apple-darwin"
      ;;
    Darwin/arm64)
      echo "aarch64-apple-darwin"
      ;;
    Linux/x86_64)
      echo "x86_64-unknown-linux-gnu"
      ;;
    Linux/aarch64|Linux/arm64)
      echo "aarch64-unknown-linux-gnu"
      ;;
    *)
      echo "install failed: unsupported platform $os/$arch" >&2
      exit 1
      ;;
  esac
}

version_to_tag() {
  local version
  version="$1"
  if [[ "$version" == v* ]]; then
    echo "$version"
  else
    echo "v$version"
  fi
}

build_default_release_base_url() {
  local version
  version="$1"

  if [[ "$version" == "latest" ]]; then
    echo "https://github.com/opsclawhq/opsclaw/releases/latest/download"
    return
  fi

  local tag
  tag="$(version_to_tag "$version")"
  echo "https://github.com/opsclawhq/opsclaw/releases/download/$tag"
}

VERSION="${OPSCLAW_VERSION:-latest}"
TARGET="${OPSCLAW_TARGET:-$(detect_target)}"
INSTALL_DIR="${OPSCLAW_INSTALL_DIR:-$HOME/.local/bin}"
ARCHIVE_NAME="opsclaw-${TARGET}.tar.gz"
ARCHIVE_SOURCE="${OPSCLAW_INSTALL_ARCHIVE:-}"
RELEASE_BASE_URL="${OPSCLAW_RELEASE_BASE_URL:-$(build_default_release_base_url "$VERSION")}"

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

ARCHIVE_PATH="$TMP_DIR/$ARCHIVE_NAME"
if [[ -n "$ARCHIVE_SOURCE" ]]; then
  if [[ ! -f "$ARCHIVE_SOURCE" ]]; then
    echo "install failed: archive not found at $ARCHIVE_SOURCE" >&2
    exit 1
  fi
  cp "$ARCHIVE_SOURCE" "$ARCHIVE_PATH"
else
  ARCHIVE_URL="${RELEASE_BASE_URL%/}/$ARCHIVE_NAME"
  echo "Downloading $ARCHIVE_URL" >&2
  curl -fsSL "$ARCHIVE_URL" -o "$ARCHIVE_PATH"
fi

LC_ALL=C tar -xzf "$ARCHIVE_PATH" -C "$TMP_DIR"
if [[ ! -x "$TMP_DIR/opsclaw" ]]; then
  echo "install failed: archive missing executable opsclaw" >&2
  exit 1
fi

mkdir -p "$INSTALL_DIR"
install -m 0755 "$TMP_DIR/opsclaw" "$INSTALL_DIR/opsclaw"

"$INSTALL_DIR/opsclaw" --version >/dev/null

echo "Installed opsclaw to $INSTALL_DIR/opsclaw"
echo "Run: $INSTALL_DIR/opsclaw --version"
