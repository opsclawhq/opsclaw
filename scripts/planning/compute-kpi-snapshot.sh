#!/usr/bin/env bash
set -euo pipefail

OWNER=""
REPO=""
PROJECT_NUMBER=""
WINDOW_DAYS="30"
LIMIT="300"
OUTPUT=""

usage() {
  echo "usage: $0 --owner <org-or-user> --repo <repo> --project <number> [--window-days <n>] [--limit <n>] [--output <path>]" >&2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --owner)
      OWNER="$2"
      shift 2
      ;;
    --repo)
      REPO="$2"
      shift 2
      ;;
    --project)
      PROJECT_NUMBER="$2"
      shift 2
      ;;
    --window-days)
      WINDOW_DAYS="$2"
      shift 2
      ;;
    --limit)
      LIMIT="$2"
      shift 2
      ;;
    --output)
      OUTPUT="$2"
      shift 2
      ;;
    *)
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$OWNER" || -z "$REPO" || -z "$PROJECT_NUMBER" ]]; then
  usage
  exit 1
fi

if ! [[ "$WINDOW_DAYS" =~ ^[0-9]+$ ]] || [[ "$WINDOW_DAYS" -eq 0 ]]; then
  echo "error: --window-days must be a positive integer" >&2
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

date_days_ago() {
  local days="$1"
  if date -u -v-"${days}"d +%Y-%m-%d >/dev/null 2>&1; then
    date -u -v-"${days}"d +%Y-%m-%d
    return
  fi

  if date -u -d "-${days} days" +%Y-%m-%d >/dev/null 2>&1; then
    date -u -d "-${days} days" +%Y-%m-%d
    return
  fi

  echo "error: unable to compute date ${days} days ago on this platform" >&2
  exit 1
}

SINCE_DATE="$(date_days_ago "$WINDOW_DAYS")"
NOW_EPOCH="$(date -u +%s)"
WINDOW_START_EPOCH=$((NOW_EPOCH - WINDOW_DAYS * 86400))
HALF_WINDOW_EPOCH=$((WINDOW_START_EPOCH + (WINDOW_DAYS * 86400 / 2)))

prs_json="$(gh pr list \
  --repo "${OWNER}/${REPO}" \
  --state merged \
  --search "merged:>=${SINCE_DATE}" \
  --limit "$LIMIT" \
  --json createdAt,mergedAt,author,title,body)"

project_json="$(gh project item-list "$PROJECT_NUMBER" --owner "$OWNER" --format json --limit "$LIMIT")"

snapshot_json="$(
  jq -cn \
    --argjson prs "$prs_json" \
    --argjson project "$project_json" \
    --argjson half_epoch "$HALF_WINDOW_EPOCH" '
    def round2: ((. * 100) | round) / 100;
    def median:
      sort as $a
      | length as $n
      | if $n == 0 then
          0
        elif ($n % 2) == 1 then
          $a[($n / 2 | floor)]
        else
          (($a[($n / 2) - 1] + $a[$n / 2]) / 2)
        end;

    $prs as $pr_rows
    | ($pr_rows | length) as $activation_count
    | ($pr_rows
      | map(((.mergedAt | fromdateiso8601) - (.createdAt | fromdateiso8601)) / 60)
      | map(select(. >= 0))
      | median) as $ttfv_minutes
    | ($project.items
      | map(."gate Status" // "")
      | map(select(. == "passed" or . == "ready" or . == "blocked"))) as $gates
    | ($gates | map(select(. == "passed")) | length) as $passed_count
    | ($gates | length) as $gate_total
    | (if $gate_total == 0 then 0 else (($passed_count / $gate_total) * 100) end) as $reliability
    | ($pr_rows
      | map(select((.mergedAt | fromdateiso8601) < $half_epoch))
      | map(.author.login // "")
      | map(select(length > 0))
      | unique) as $first_half_authors
    | ($pr_rows
      | map(select((.mergedAt | fromdateiso8601) >= $half_epoch))
      | map(.author.login // "")
      | map(select(length > 0))
      | unique) as $second_half_authors
    | ($first_half_authors | map(select(. as $a | $second_half_authors | index($a))) | length) as $retained
    | ($first_half_authors | length) as $first_half_count
    | (if $first_half_count == 0 then 0 else (($retained / $first_half_count) * 100) end) as $retention
    | ($pr_rows
      | map(
          ((.title // "") + " " + (.body // ""))
          | ascii_downcase
          | test("enterprise|pilot")
        )
      | map(select(. == true))
      | length) as $enterprise_pilots
    | {
        activation: ($activation_count | tonumber),
        ttfv_minutes: ($ttfv_minutes | round2),
        reliability_success_rate: ($reliability | round2),
        retention_d30: ($retention | round2),
        enterprise_pilots: ($enterprise_pilots | tonumber)
      }'
)"

if [[ -n "$OUTPUT" ]]; then
  mkdir -p "$(dirname "$OUTPUT")"
  printf '%s\n' "$snapshot_json" > "$OUTPUT"
fi

printf '%s\n' "$snapshot_json"
