# Phase 2: Building a Repeatable Content and Docs System

Date: 2026-02-17

OpsClaw now treats docs and social artifacts as hard Definition-of-Done outputs, not optional afterthoughts.

## What We Added

- A phase-by-phase `OpsClaw vs OpenClaw` comparison series structure.
- Dedicated LinkedIn and X draft slots for every phase.
- Separate docs tracks for users and contributors.
- A predictable file contract for phase closure updates.

## Why This Design

We considered waiting until Phase 5 (when automation lands), but that would compress writing quality and lose context from early technical decisions. We chose to start now with manual, structured drafts and phase templates.

## Tradeoffs

- Pros: consistent narrative, easier traceability, lower content debt at release.
- Cons: adds overhead during implementation phases.

We accept the overhead because OpsClaw is explicitly build-in-public and phase gates already require docs/social evidence.

## Update Rule Going Forward

At each phase closure:

1. Update `.content/series/opsclaw-vs-openclaw/phase-<n>-comparison.md` with shipped deltas.
2. Publish or refresh LinkedIn and X drafts in `.content/phase-<n>/`.
3. Add one tracked engineering blog post in `docs/blog/`.

The helper command `scripts/content/update-phase-content.sh <phase-number> [yyyy-mm-dd]` scaffolds these files when missing.
