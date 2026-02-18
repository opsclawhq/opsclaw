#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOW_PLACEHOLDER_KPI="${ALLOW_PLACEHOLDER_KPI:-0}"

resolve_contracts_dir() {
  if [[ -n "${OPSCLAW_METADATA_DIR:-}" ]]; then
    echo "$OPSCLAW_METADATA_DIR"
    return
  fi

  if [[ -d "$ROOT/.planning/contracts" ]]; then
    echo "$ROOT/.planning/contracts"
    return
  fi

  # Worktree fallback: scripts may run from ".worktrees/<branch>" paths.
  if [[ -d "$ROOT/../../.planning/contracts" ]]; then
    echo "$(cd "$ROOT/../../.planning/contracts" && pwd)"
    return
  fi

  echo "$ROOT/.planning/contracts"
}

CONTRACTS_DIR="$(resolve_contracts_dir)"

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required" >&2
  exit 1
fi

if [[ ! -d "$CONTRACTS_DIR" ]]; then
  echo "error: contracts directory not found: $CONTRACTS_DIR" >&2
  exit 1
fi

METADATA_FILES=()
while IFS= read -r file; do
  METADATA_FILES+=("$file")
done < <(find "$CONTRACTS_DIR" -maxdepth 1 -type f -name '*.metadata.json' | sort)
if [[ ${#METADATA_FILES[@]} -eq 0 ]]; then
  echo "error: no metadata files found in $CONTRACTS_DIR" >&2
  exit 1
fi

required_keys='["phase","plan_id","requirement_ids","branch","pr","tdd_evidence_uri","docs_artifacts","social_artifacts","gate_status","kpi_snapshot"]'
kpi_keys='["activation","ttfv_minutes","reliability_success_rate","retention_d30","enterprise_pilots"]'

total=0
errors=0
placeholder_count=0

for file in "${METADATA_FILES[@]}"; do
  total=$((total + 1))

  if ! jq -e 'type == "object"' "$file" >/dev/null; then
    echo "invalid: $file (not a JSON object)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e --argjson keys "$required_keys" '. as $obj | ($keys | all(. as $k | ($obj | has($k))))' "$file" >/dev/null; then
    echo "invalid: $file (missing required top-level keys)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e '.gate_status | IN("not_ready","ready","passed","blocked")' "$file" >/dev/null; then
    echo "invalid: $file (gate_status must be not_ready|ready|passed|blocked)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e '.requirement_ids | type == "array" and length > 0 and all(.[]; type == "string" and length > 0)' "$file" >/dev/null; then
    echo "invalid: $file (requirement_ids must be non-empty string array)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e '.docs_artifacts | type == "array" and all(.[]; type == "string")' "$file" >/dev/null; then
    echo "invalid: $file (docs_artifacts must be string array)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e '.social_artifacts | type == "array" and all(.[]; type == "string")' "$file" >/dev/null; then
    echo "invalid: $file (social_artifacts must be string array)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e --argjson keys "$kpi_keys" '
    .kpi_snapshot as $kpi |
    ($kpi | type == "object") and
    ($keys | all(. as $k | ($kpi | has($k)))) and
    ([$kpi.activation, $kpi.ttfv_minutes, $kpi.reliability_success_rate, $kpi.retention_d30, $kpi.enterprise_pilots] | all(type == "number"))
  ' "$file" >/dev/null; then
    echo "invalid: $file (kpi_snapshot fields missing or non-numeric)" >&2
    errors=$((errors + 1))
    continue
  fi

  if jq -e '[.kpi_snapshot.activation, .kpi_snapshot.ttfv_minutes, .kpi_snapshot.reliability_success_rate, .kpi_snapshot.retention_d30, .kpi_snapshot.enterprise_pilots] | all(. == 0)' "$file" >/dev/null; then
    placeholder_count=$((placeholder_count + 1))
    if [[ "$ALLOW_PLACEHOLDER_KPI" != "1" ]]; then
      echo "invalid: $file (placeholder KPI snapshot detected; set ALLOW_PLACEHOLDER_KPI=1 to permit temporarily)" >&2
      errors=$((errors + 1))
      continue
    fi
  fi

  echo "valid: $file"
done

echo "summary: files=$total valid=$((total - errors)) errors=$errors placeholders=$placeholder_count"

if [[ $errors -gt 0 ]]; then
  exit 1
fi
