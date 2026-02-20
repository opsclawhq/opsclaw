#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

SNAPSHOT_PATH=""
CONTRACTS_DIR="$ROOT/.planning/contracts"
OWNER=""
PROJECT_NUMBER=""
LIMIT="300"

usage() {
  echo "usage: $0 --snapshot <path> [--contracts-dir <path>] [--owner <org-or-user> --project <number> [--limit <n>]]" >&2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --snapshot)
      SNAPSHOT_PATH="$2"
      shift 2
      ;;
    --contracts-dir)
      CONTRACTS_DIR="$2"
      shift 2
      ;;
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
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$SNAPSHOT_PATH" ]]; then
  usage
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required" >&2
  exit 1
fi

if [[ ! -f "$SNAPSHOT_PATH" ]]; then
  echo "error: snapshot file not found: $SNAPSHOT_PATH" >&2
  exit 1
fi

if [[ ! -d "$CONTRACTS_DIR" ]]; then
  echo "error: contracts directory not found: $CONTRACTS_DIR" >&2
  exit 1
fi

if ! jq -e '
  . as $kpi
  | (type == "object")
  and (["activation","ttfv_minutes","reliability_success_rate","retention_d30","enterprise_pilots"] | all(. as $k | ($kpi | has($k))))
  and ([$kpi.activation,$kpi.ttfv_minutes,$kpi.reliability_success_rate,$kpi.retention_d30,$kpi.enterprise_pilots] | all(type == "number"))
' "$SNAPSHOT_PATH" >/dev/null; then
  echo "error: snapshot must match KPI schema object" >&2
  exit 1
fi

snapshot_compact="$(jq -c '.' "$SNAPSHOT_PATH")"
updated_files=0

while IFS= read -r file; do
  tmp_file="$(mktemp)"
  jq --argjson kpi "$snapshot_compact" '.kpi_snapshot = $kpi' "$file" > "$tmp_file"
  mv "$tmp_file" "$file"
  updated_files=$((updated_files + 1))
done < <(find "$CONTRACTS_DIR" -maxdepth 1 -type f -name '*.metadata.json' | sort)

echo "updated metadata files: $updated_files"

if [[ -n "$OWNER" || -n "$PROJECT_NUMBER" ]]; then
  if [[ -z "$OWNER" || -z "$PROJECT_NUMBER" ]]; then
    echo "error: both --owner and --project are required for GitHub Project updates" >&2
    exit 1
  fi

  if ! command -v gh >/dev/null 2>&1; then
    echo "error: gh CLI is required for GitHub Project updates" >&2
    exit 1
  fi

  project_id="$(gh project view "$PROJECT_NUMBER" --owner "$OWNER" --format json | jq -r '.id')"
  if [[ -z "$project_id" || "$project_id" == "null" ]]; then
    echo "error: unable to resolve project id for owner=$OWNER project=$PROJECT_NUMBER" >&2
    exit 1
  fi

  kpi_field_id="$(
    gh project field-list "$PROJECT_NUMBER" --owner "$OWNER" --format json \
      | jq -r '.fields[] | select((.name == "KPI Snapshot") or (.name == "kPI Snapshot")) | .id' \
      | head -n1
  )"
  if [[ -z "$kpi_field_id" ]]; then
    echo "error: KPI Snapshot field id not found for project $PROJECT_NUMBER" >&2
    exit 1
  fi

  project_items="$(gh project item-list "$PROJECT_NUMBER" --owner "$OWNER" --format json --limit "$LIMIT")"
  updated_items=0
  while IFS= read -r item_id; do
    gh project item-edit \
      --id "$item_id" \
      --project-id "$project_id" \
      --field-id "$kpi_field_id" \
      --text "$snapshot_compact" >/dev/null
    updated_items=$((updated_items + 1))
  done < <(
    jq -r '.items[]
      | select((."gate Status" // "") == "passed" or (."gate Status" // "") == "ready")
      | .id' <<<"$project_items"
  )

  echo "updated project items: $updated_items"
fi
