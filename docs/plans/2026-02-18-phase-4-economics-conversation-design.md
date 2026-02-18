# Phase 4 Economics + Conversation Viewer (04-04) Design

## Scope
Implement roadmap plan `04-04` by extending dashboard surfaces with:

1. per-agent economics tracking view
2. ROI summary view
3. conversation transcript viewer including tool call details

## Requirement Coverage
- DASH-04
- DASH-05
- DASH-07

## Options Considered

1. Wait for backend metrics and transcript APIs before frontend work.
2. Build deterministic frontend views now against contract-shaped mock payloads (recommended).
3. Implement full persistence and backend aggregation in this slice.

## Selected Approach
Option 2. It closes the last major product-surface gap in Phase 4 while keeping backend integration as a later wiring step.

## Architecture

- Add economics + conversation view-model helpers under `packages/dashboard/src/lib`.
- Add two new UI components:
  - `EconomicsPanel`
  - `ConversationViewerPanel`
- Extend dashboard mock data with economics snapshots and transcript payloads.
- Keep deterministic tests in `node:test` for ROI math and transcript formatting behavior.

## Failure Modes

- ROI math can become non-deterministic or divide by zero when spend is zero
- transcript rendering can lose tool-call output ordering
- per-agent economics row mapping can drift from dashboard contract fields

## Verification Strategy

- RED: run tests referencing new economics/conversation module before implementation
- GREEN: dashboard tests pass for ROI, cost aggregation, and transcript view-model behavior
- full: workspace `cargo test` and `cargo clippy` remain green
