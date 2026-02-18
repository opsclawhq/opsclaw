#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<USAGE
usage: $0 --phase <n> --date <yyyy-mm-dd> [--root <path>]
USAGE
}

PHASE=""
DATE=""
ROOT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --phase)
      PHASE="${2:-}"
      shift 2
      ;;
    --date)
      DATE="${2:-}"
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

if [[ -z "$PHASE" || -z "$DATE" ]]; then
  echo "missing required args" >&2
  usage >&2
  exit 1
fi

if [[ -z "$ROOT" ]]; then
  ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
fi

BLOG_DIR="$ROOT/docs/blog"
mkdir -p "$BLOG_DIR"

RECAP_FILE="$BLOG_DIR/${DATE}-phase-${PHASE}-engineering-recap.md"
DESIGN_FILE="$BLOG_DIR/${DATE}-phase-${PHASE}-engineering-design-decisions.md"
PROCESS_FILE="$BLOG_DIR/${DATE}-phase-${PHASE}-engineering-reliability-process.md"

if [[ ! -f "$RECAP_FILE" ]]; then
  cat > "$RECAP_FILE" <<RECAP
# Phase $PHASE Engineering Recap

## What Shipped

- summarize merged slices and user-visible outcomes.

## Verification and Quality Gates

- include RED/GREEN evidence links and CI/verification summary.

## Next Phase Focus

- list next execution priorities.
RECAP
fi

if [[ ! -f "$DESIGN_FILE" ]]; then
  cat > "$DESIGN_FILE" <<DESIGN
# Phase $PHASE Engineering Design Decisions

## Accepted Decisions

- record architectural choices that shaped this phase.

## Rejected Alternatives

1. alternative A
2. alternative B

## Tradeoffs

- summarize cost, speed, and reliability tradeoffs.
DESIGN
fi

if [[ ! -f "$PROCESS_FILE" ]]; then
  cat > "$PROCESS_FILE" <<PROCESS
# Phase $PHASE Reliability and Process Notes

## Process Signals

- plan adherence
- review gate outcomes
- verification coverage

## Reliability Signals

- test and lint status
- notable regressions prevented

## Workflow Improvements

- what changed in delivery process for next phase.
PROCESS
fi

cat <<OUT
Engineering blog drafts ready:
- ${RECAP_FILE#$ROOT/}
- ${DESIGN_FILE#$ROOT/}
- ${PROCESS_FILE#$ROOT/}
OUT
