#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET="${OPSCLAW_TARGET:-$(rustc -vV | awk '/^host: / {print $2}')}"
DIST_DIR="${OPSCLAW_DIST_DIR:-$ROOT/dist}"
ARCHIVE_NAME="opsclaw-${TARGET}.tar.gz"
ARCHIVE_PATH="$DIST_DIR/$ARCHIVE_NAME"

WORK_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

echo "Building opsclaw for target $TARGET" >&2
cargo build --release -p opsclaw --target "$TARGET"

BIN_PATH="$ROOT/target/$TARGET/release/opsclaw"
if [[ ! -x "$BIN_PATH" ]]; then
  echo "packaging failed: missing built binary at $BIN_PATH" >&2
  exit 1
fi

mkdir -p "$WORK_DIR/pkg" "$DIST_DIR"
cp "$BIN_PATH" "$WORK_DIR/pkg/opsclaw"
chmod 0755 "$WORK_DIR/pkg/opsclaw"

LC_ALL=C tar -C "$WORK_DIR/pkg" -czf "$ARCHIVE_PATH" opsclaw

echo "$ARCHIVE_PATH"
