# Phase 2 Content Series and Docs Track Design

## Scope
Create a repeatable content system that publishes an OpsClaw vs OpenClaw comparison update after every phase, alongside phase-scoped LinkedIn and X drafts. Establish user and developer docs tracks plus engineering blog scaffolding so content and docs are first-class DoD outputs.

## Options Considered
1. Keep ad-hoc social drafts only per slice.
2. Add a phase-structured comparison series with templates and per-phase draft files (recommended).
3. Defer comparison series until Phase 5 automation.

## Selected Approach
Option 2. Implement the series now with deterministic file paths and templates so each phase closure updates the same artifacts. Keep automation scope in Phase 5, but enforce manual cadence immediately.

## Data Flow
1. For each phase, update comparison source at `.content/series/opsclaw-vs-openclaw/phase-<n>-comparison.md`.
2. Produce per-phase social drafts in `.content/phase-<n>/` for LinkedIn and X.
3. Mirror stable public narrative in tracked docs under `docs/blog/` and docs index pages.
4. Record artifact paths in planning/state metadata and project tracking fields.

## Failure Modes
- Comparison claims drift from shipped features.
  - Mitigation: each phase file requires "What shipped in OpsClaw" bullets tied to merged PRs.
- Social drafts exist but no consistent structure.
  - Mitigation: template with required sections (`hook`, `proof`, `delta vs OpenClaw`, `CTA`).
- Docs become stale relative to roadmap gates.
  - Mitigation: roadmap/state entries include mandatory update checkpoints per phase.

## Test Strategy
- Verify all required series files exist for phases `0..5`.
- Verify each phase has both LinkedIn and X comparison drafts.
- Run workspace verification commands to ensure docs/content changes do not regress code health:
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
