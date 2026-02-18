#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<USAGE
usage: $0 --phase <n> --slice <id> --date <yyyy-mm-dd> --requirements <csv> --pr-url <url> --summary <text> [--root <path>]

Example:
  $0 --phase 5 --slice 05-05 --date 2026-02-18 --requirements PUB-04,PUB-07 --pr-url https://github.com/opsclawhq/opsclaw/pull/36 --summary "Phase 5 content pipeline baseline"
USAGE
}

PHASE=""
SLICE=""
DATE=""
REQUIREMENTS_CSV=""
PR_URL=""
SUMMARY=""
ROOT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --phase)
      PHASE="${2:-}"
      shift 2
      ;;
    --slice)
      SLICE="${2:-}"
      shift 2
      ;;
    --date)
      DATE="${2:-}"
      shift 2
      ;;
    --requirements)
      REQUIREMENTS_CSV="${2:-}"
      shift 2
      ;;
    --pr-url)
      PR_URL="${2:-}"
      shift 2
      ;;
    --summary)
      SUMMARY="${2:-}"
      shift 2
      ;;
    --root)
      ROOT="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ -z "$PHASE" || -z "$SLICE" || -z "$DATE" || -z "$REQUIREMENTS_CSV" || -z "$PR_URL" || -z "$SUMMARY" ]]; then
  echo "missing required arguments" >&2
  usage >&2
  exit 1
fi

if [[ -z "$ROOT" ]]; then
  ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
fi

SERIES_DIR="$ROOT/.content/series/opsclaw-vs-openclaw"
PHASE_DIR="$ROOT/.content/phase-$PHASE"
BLOG_DIR="$ROOT/docs/blog"
MANIFEST_DIR="$BLOG_DIR/manifests"

mkdir -p "$SERIES_DIR" "$PHASE_DIR" "$BLOG_DIR" "$MANIFEST_DIR"

PREFIX="${DATE}-phase-${PHASE}-${SLICE}"
SERIES_FILE="$SERIES_DIR/phase-$PHASE-comparison.md"
LINKEDIN_FILE="$PHASE_DIR/${PREFIX}-linkedin-draft.md"
X_FILE="$PHASE_DIR/${PREFIX}-x-thread.md"
RECAP_FILE="$BLOG_DIR/${PREFIX}-recap.md"
DESIGN_FILE="$BLOG_DIR/${PREFIX}-design-decisions.md"
MANIFEST_FILE="$MANIFEST_DIR/${PREFIX}-manifest.json"

json_array_from_csv() {
  local csv="$1"
  local out="["
  local first=1
  IFS=',' read -r -a reqs <<< "$csv"
  for raw in "${reqs[@]}"; do
    local req
    req="$(echo "$raw" | xargs)"
    if [[ -z "$req" ]]; then
      continue
    fi
    if [[ $first -eq 0 ]]; then
      out+=" ,"
    fi
    out+="\"$req\""
    first=0
  done
  out+="]"
  echo "$out"
}

REQ_JSON="$(json_array_from_csv "$REQUIREMENTS_CSV")"

if [[ ! -f "$SERIES_FILE" ]]; then
  cat > "$SERIES_FILE" <<SERIES
# OpsClaw vs OpenClaw - Phase $PHASE

## Snapshot

- Phase: $PHASE
- Date: $DATE
- Gate status: in_progress

## What Shipped in OpsClaw

- Requirement IDs touched:
- Plans/PRs:
- User-visible behavior:

## OpenClaw Baseline (Validated)

- Relevant capabilities used for comparison:

## Delta This Phase

- OpsClaw strengths:
- OpsClaw current gaps:
- Explicitly out of scope this phase:

## Evidence

- Docs:
- Changelog:
- TDD/review evidence:

## Social Outputs

- LinkedIn draft path: .content/phase-$PHASE/${PREFIX}-linkedin-draft.md
- X thread draft path: .content/phase-$PHASE/${PREFIX}-x-thread.md
- Published links (if any):
SERIES
fi

if [[ ! -f "$LINKEDIN_FILE" ]]; then
  cat > "$LINKEDIN_FILE" <<LINKEDIN
Phase $PHASE slice $SLICE update for OpsClaw.

What shipped:
- $SUMMARY

Requirements:
- $REQUIREMENTS_CSV

PR:
- $PR_URL

Why it matters:
- deterministic, phase-scoped delivery artifacts now exist for build-in-public cadence.
LINKEDIN
fi

if [[ ! -f "$X_FILE" ]]; then
  cat > "$X_FILE" <<XTHREAD
OpsClaw Phase $PHASE slice $SLICE shipped.

1) Summary: $SUMMARY
2) Requirements: $REQUIREMENTS_CSV
3) PR: $PR_URL
4) LinkedIn draft: .content/phase-$PHASE/${PREFIX}-linkedin-draft.md
5) Blog recap: docs/blog/${PREFIX}-recap.md
XTHREAD
fi

if [[ ! -f "$RECAP_FILE" ]]; then
  cat > "$RECAP_FILE" <<RECAP
# Phase $PHASE Slice $SLICE Recap

## What Shipped

$SUMMARY

## Requirement Coverage

- $REQUIREMENTS_CSV

## Pull Request

- $PR_URL

## Validation

- tests, lint, and review evidence are captured in the slice evidence doc.
RECAP
fi

if [[ ! -f "$DESIGN_FILE" ]]; then
  cat > "$DESIGN_FILE" <<DESIGN
# Phase $PHASE Slice $SLICE Design Decisions

## Decision Summary

- selected deterministic pipeline-first content generation.

## Alternatives Rejected

1. manual drafting only
2. ad-hoc social posts without traceability manifest

## Tradeoffs

- higher process rigor, lower phase-to-phase drift.
DESIGN
fi

cat > "$MANIFEST_FILE" <<MANIFEST
{
  "phase": $PHASE,
  "slice": "$SLICE",
  "date": "$DATE",
  "requirements": $REQ_JSON,
  "pr_url": "$PR_URL",
  "summary": "$SUMMARY",
  "artifacts": {
    "series": "${SERIES_FILE#$ROOT/}",
    "linkedin": "${LINKEDIN_FILE#$ROOT/}",
    "x_thread": "${X_FILE#$ROOT/}",
    "blog_recap": "${RECAP_FILE#$ROOT/}",
    "blog_design": "${DESIGN_FILE#$ROOT/}"
  }
}
MANIFEST

cat <<OUT
Phase delivery pipeline outputs:
- ${SERIES_FILE#$ROOT/}
- ${LINKEDIN_FILE#$ROOT/}
- ${X_FILE#$ROOT/}
- ${RECAP_FILE#$ROOT/}
- ${DESIGN_FILE#$ROOT/}
- ${MANIFEST_FILE#$ROOT/}
OUT
