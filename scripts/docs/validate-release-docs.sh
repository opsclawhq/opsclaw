#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

required_files=(
  "docs/README.md"
  "docs/agent-index.yaml"
  "docs/user-guide/README.md"
  "docs/developer-guide/README.md"
  "docs/blog/README.md"
  "docs/user-guide/build-in-public-pipeline.md"
  "docs/developer-guide/docs-release-hardening.md"
  "docs/blog/editorial-workflow.md"
)

missing=0
for rel in "${required_files[@]}"; do
  if [[ ! -f "$ROOT/$rel" ]]; then
    echo "missing file: $rel" >&2
    missing=$((missing + 1))
  fi
done

if [[ $missing -gt 0 ]]; then
  echo "release-doc validation failed: $missing required files missing" >&2
  exit 1
fi

if ! grep -q "Agent Docs Index" "$ROOT/docs/README.md"; then
  echo "docs/README.md missing Agent Docs Index link" >&2
  exit 1
fi

if ! grep -q "Build-in-Public Pipeline" "$ROOT/docs/user-guide/README.md"; then
  echo "docs/user-guide/README.md missing Build-in-Public Pipeline link" >&2
  exit 1
fi

if ! grep -q "Docs Release Hardening" "$ROOT/docs/developer-guide/README.md"; then
  echo "docs/developer-guide/README.md missing Docs Release Hardening link" >&2
  exit 1
fi

if ! grep -q "Editorial Workflow" "$ROOT/docs/blog/README.md"; then
  echo "docs/blog/README.md missing Editorial Workflow link" >&2
  exit 1
fi

echo "release-doc validation passed"
