# Mission Control Architecture (Phase 4)

This guide summarizes the dashboard architecture after Phase 4 slices `04-01` through `04-04`.

## Contract Foundation

- OpenAPI source: `docs/api/mission-control-openapi.yaml`
- Shared contract types: `crates/oax-core/src/types.rs`
- Generated TypeScript types: `packages/sdk/src/generated/types.ts`

## Frontend Package Structure

- package root: `packages/dashboard`
- entry: `packages/dashboard/src/App.tsx`
- major modules:
  - `lib/view-models.mjs` (hierarchy/profile/feed/kanban view shaping)
  - `lib/dashboard-state.mjs` (task + approval event reducer)
  - `lib/economics-conversation.mjs` (ROI math + transcript retrieval)

## UI Component Map

- `OrgHierarchyPanel`
- `AgentProfilePanel`
- `ActivityFeedPanel`
- `KanbanBoardPanel`
- `ApprovalQueuePanel`
- `EconomicsPanel`
- `ConversationViewerPanel`

## Data Sources in Phase 4

- `mock-data.mjs`: contract-shaped fixture payloads
- `mock-stream-events.mjs`: simulated stream events for reducer behavior

## Verification Discipline

- RED/GREEN evidence docs live under `docs/plans/evidence/`
- dashboard tests run via Node test runner:

```bash
npm --prefix packages/dashboard test
```

- workspace safety verification:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Integration Note

Phase 4 currently simulates stream updates locally. Phase 5 should replace interval-based mock application with live channel transport integration and API-backed persistence.
