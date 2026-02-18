#!/usr/bin/env bash
set -euo pipefail

OWNER=""
PROJECT_NUMBER=""
LIMIT="200"
ALLOW_PLACEHOLDER_KPI="${ALLOW_PLACEHOLDER_KPI:-0}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --owner)
      OWNER="$2"
      shift 2
      ;;
    --project)
      PROJECT_NUMBER="$2"
      shift 2
      ;;
    --limit)
      LIMIT="$2"
      shift 2
      ;;
    *)
      echo "usage: $0 --owner <org-or-user> --project <number> [--limit <n>]" >&2
      exit 1
      ;;
  esac
done

if [[ -z "$OWNER" || -z "$PROJECT_NUMBER" ]]; then
  echo "usage: $0 --owner <org-or-user> --project <number> [--limit <n>]" >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "error: gh CLI is required" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required" >&2
  exit 1
fi

items_json="$(gh project item-list "$PROJECT_NUMBER" --owner "$OWNER" --format json --limit "$LIMIT")"

total_items=$(jq '.items | length' <<<"$items_json")
audited=0
errors=0
placeholder_count=0

while IFS= read -r item; do
  gate_status=$(jq -r '."gate Status" // ""' <<<"$item")
  if [[ "$gate_status" != "passed" && "$gate_status" != "ready" ]]; then
    continue
  fi

  audited=$((audited + 1))
  item_number=$(jq -r '.content.number // "n/a"' <<<"$item")
  phase=$(jq -r '.phase // "n/a"' <<<"$item")

  kpi_value=$(jq -c '
    def parse_kpi:
      if . == null then null
      elif (type == "string") then (fromjson? // null)
      elif (type == "object") then .
      else null
      end;
    (."kPI Snapshot" // ."KPI Snapshot" // null) | parse_kpi
  ' <<<"$item")

  if [[ "$kpi_value" == "null" ]]; then
    echo "invalid: item#$item_number phase=$phase gate=$gate_status (missing/invalid KPI Snapshot JSON)" >&2
    errors=$((errors + 1))
    continue
  fi

  if ! jq -e '
    . as $kpi |
    (type == "object") and
    (["activation","ttfv_minutes","reliability_success_rate","retention_d30","enterprise_pilots"] | all(. as $k | ($kpi | has($k)))) and
    ([$kpi.activation,$kpi.ttfv_minutes,$kpi.reliability_success_rate,$kpi.retention_d30,$kpi.enterprise_pilots] | all(type == "number"))
  ' <<<"$kpi_value" >/dev/null; then
    echo "invalid: item#$item_number phase=$phase gate=$gate_status (KPI fields missing or non-numeric)" >&2
    errors=$((errors + 1))
    continue
  fi

  if jq -e '[.activation,.ttfv_minutes,.reliability_success_rate,.retention_d30,.enterprise_pilots] | all(. == 0)' <<<"$kpi_value" >/dev/null; then
    placeholder_count=$((placeholder_count + 1))
    if [[ "$ALLOW_PLACEHOLDER_KPI" != "1" ]]; then
      echo "invalid: item#$item_number phase=$phase gate=$gate_status (placeholder KPI snapshot detected; set ALLOW_PLACEHOLDER_KPI=1 to permit temporarily)" >&2
      errors=$((errors + 1))
      continue
    fi
  fi

  echo "valid: item#$item_number phase=$phase gate=$gate_status"
done < <(jq -c '.items[]' <<<"$items_json")

echo "summary: total_items=$total_items audited=$audited errors=$errors placeholders=$placeholder_count"

if [[ $audited -eq 0 ]]; then
  echo "error: no items with gate status passed/ready were audited" >&2
  exit 1
fi

if [[ $errors -gt 0 ]]; then
  exit 1
fi
