# Mission Control Kanban and Approval Stream (Phase 4 Slice 04-03)

This guide describes the event-driven dashboard state path introduced for kanban and approval surfaces.

## Package Scope

- `packages/dashboard`

## New Surfaces

- `KanbanBoardPanel`: five-stage task flow board
- `ApprovalQueuePanel`: pending approval list with approve/reject actions

## Event State Layer

- `packages/dashboard/src/lib/dashboard-state.mjs`
- `packages/dashboard/src/lib/mock-stream-events.mjs`

Supported events in this slice:

- `task.moved`
- `approval.requested`
- `approval.decided`

## Deterministic Tests

- `packages/dashboard/tests/kanban-approvals.test.mjs`

Coverage in the test file:

- task stage transition updates
- pending approval creation
- approval decision resolution
- unknown task-event no-op behavior

## Verification

```bash
npm --prefix packages/dashboard test
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Next Integration Step

Replace interval-based mock stream application in `App.tsx` with live WebSocket events from the Mission Control API stream endpoint.
