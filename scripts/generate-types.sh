#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="$ROOT_DIR/crates/oax-core/src"
OUT_FILE="$ROOT_DIR/packages/sdk/src/generated/types.ts"

if ! command -v typeshare >/dev/null 2>&1; then
  echo "typeshare is not installed. Run: cargo install typeshare-cli --locked" >&2
  exit 1
fi

mkdir -p "$(dirname "$OUT_FILE")"
typeshare "$SRC_DIR" --lang=typescript --output-file="$OUT_FILE"
