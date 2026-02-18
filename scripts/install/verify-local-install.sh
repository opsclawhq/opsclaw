#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

ARCHIVE_PATH="$(bash "$ROOT/scripts/release/package-opsclaw.sh" | tail -n 1)"
if [[ ! -f "$ARCHIVE_PATH" ]]; then
  echo "verification failed: package archive missing at $ARCHIVE_PATH" >&2
  exit 1
fi

INSTALL_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$INSTALL_DIR"
}
trap cleanup EXIT

OPSCLAW_INSTALL_ARCHIVE="$ARCHIVE_PATH" \
OPSCLAW_INSTALL_DIR="$INSTALL_DIR" \
  bash "$ROOT/scripts/install/install-opsclaw.sh"

"$INSTALL_DIR/opsclaw" --version

echo "local install verification passed"
