# Phase 4 Kanban + Approval Prompts (04-03) Design

## Scope
Implement roadmap plan `04-03` by extending the dashboard foundation with:

1. Kanban board surface (Inbox -> Assigned -> In Progress -> Review -> Done)
2. Approval prompt surface with pending approve/reject actions
3. Event-driven update path that applies stream events to dashboard state

## Requirement Coverage
- DASH-02
- DASH-03
- DASH-08

## Options Considered

1. Wait for backend handlers and implement only after REST/WebSocket are live.
2. Build event-driven frontend state transitions now with contract-shaped mock stream events (recommended).
3. Implement full production websocket transport in this slice.

## Selected Approach
Option 2. This keeps the slice bounded and testable while validating real-time state transition behavior before transport integration.

## Architecture

- Add deterministic reducer helpers under `packages/dashboard/src/lib` for:
  - task stage transitions
  - approval queue updates
  - activity feed append behavior
- Add two UI components:
  - `KanbanBoardPanel`
  - `ApprovalQueuePanel`
- Extend `App.tsx` to apply mock stream events over time (simulation of real-time flow).
- Keep verification on pure JS reducers (`node:test`) for deterministic RED/GREEN proof.

## Failure Modes

- invalid stage transition can drop tasks or duplicate cards
- approval events can drift from task state (approved card not moved forward)
- activity stream order can regress when merging event updates

## Verification Strategy

- RED: run tests referencing new reducer module before implementation
- GREEN: dashboard tests pass for transition, approval queue, and activity append behavior
- full: workspace `cargo test` and `cargo clippy` remain green
