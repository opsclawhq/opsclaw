# Phase 4 Dashboard Foundation (Org + Profile + Activity) Design

## Scope
Implement roadmap plan `04-02` by introducing the first dashboard frontend package with three focused surfaces:

1. org hierarchy view
2. selected agent profile view
3. activity feed view

This slice is frontend-foundation only; backend API handlers and live WebSocket wiring remain for later Phase 4 slices.

## Requirement Coverage
- DASH-01
- DASH-03
- DASH-06

## Options Considered

1. Build backend handlers first, defer frontend scaffolding.
2. Create frontend scaffold now against mock/typed contracts (recommended).
3. Build full dashboard app with kanban and approvals in one slice.

## Selected Approach
Option 2. It keeps scope bounded while proving contract consumption and interaction patterns needed for upcoming real-time slices.

## Architecture

- Create `packages/dashboard` package with:
  - React/TanStack component scaffold
  - deterministic view-model helpers for org tree, profile selection, activity ordering
  - local mock dataset shaped to Phase 4 contract types
- Keep executable tests on pure JS view-model helpers (`node --test`) so this slice has runnable verification without requiring a full frontend toolchain install.

## Failure Modes

- hierarchy output unstable order -> noisy UI diffs
- profile lookup fails for unknown agent ID -> empty panel regressions
- feed ordering wrong -> operators see stale events as newest

## Verification Strategy

- RED: package tests fail before helper implementation
- GREEN: `npm --prefix packages/dashboard test` passes
- global checks: workspace Rust tests and clippy remain green
