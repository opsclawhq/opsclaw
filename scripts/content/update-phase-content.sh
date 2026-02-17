#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "usage: $0 <phase-number> [yyyy-mm-dd]" >&2
  exit 1
fi

PHASE="$1"
DATE="${2:-$(date +%F)}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SERIES_DIR="$ROOT/.content/series/opsclaw-vs-openclaw"
PHASE_DIR="$ROOT/.content/phase-$PHASE"

mkdir -p "$SERIES_DIR" "$PHASE_DIR"

SERIES_FILE="$SERIES_DIR/phase-$PHASE-comparison.md"
LI_FILE="$PHASE_DIR/${DATE}-phase-${PHASE}-opsclaw-vs-openclaw-linkedin-draft.md"
X_FILE="$PHASE_DIR/${DATE}-phase-${PHASE}-opsclaw-vs-openclaw-x-thread.md"

if [[ ! -f "$SERIES_FILE" ]]; then
  cat > "$SERIES_FILE" <<SERIES
# OpsClaw vs OpenClaw - Phase $PHASE

## Snapshot

- Phase: $PHASE
- Date: $DATE
- Gate status: not_ready

## What Shipped in OpsClaw

- Requirement IDs:
- Plans/PRs:
- User-visible behavior:

## OpenClaw Baseline (Validated)

- Reference link(s):
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

- LinkedIn draft path: .content/phase-$PHASE/${DATE}-phase-${PHASE}-opsclaw-vs-openclaw-linkedin-draft.md
- X thread draft path: .content/phase-$PHASE/${DATE}-phase-${PHASE}-opsclaw-vs-openclaw-x-thread.md
- Published links (if any):
SERIES
fi

if [[ ! -f "$LI_FILE" ]]; then
  cat > "$LI_FILE" <<LINKEDIN
# LinkedIn Draft - Phase $PHASE OpsClaw vs OpenClaw

Phase $PHASE update.

What shipped in OpsClaw:
- 

Comparison angle:
- 

Why it matters:
- 
LINKEDIN
fi

if [[ ! -f "$X_FILE" ]]; then
  cat > "$X_FILE" <<XTHREAD
OpsClaw vs OpenClaw - Phase $PHASE:

1) 
2) 
3) 
4) 
5) 
XTHREAD
fi

cat <<OUT
Created/verified phase content artifacts:
- $SERIES_FILE
- $LI_FILE
- $X_FILE

Next:
1. Fill shipped PR evidence and requirement IDs.
2. Link docs/changelog/tdd evidence.
3. Record published URLs after posting.
OUT
