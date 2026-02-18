# Mission Control Frontend Foundation (Phase 4 Slice 04-02)

This guide covers the initial dashboard frontend package introduced in Phase 4.

## Package Location

- `packages/dashboard`

## Current Surfaces

- Org hierarchy panel
- Agent profile panel
- Activity feed panel

## Data and View-Model Layer

- `packages/dashboard/src/lib/mock-data.mjs`: contract-shaped mock payloads
- `packages/dashboard/src/lib/view-models.mjs`: deterministic data shaping for:
  - hierarchy grouping
  - profile selection + recent activity
  - newest-first activity feed ordering

## Verification

Run dashboard helper tests:

```bash
npm --prefix packages/dashboard test
```

Run workspace safety checks after dashboard changes:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Next Integration Step

Replace mock data usage with API contract clients derived from:

- `docs/api/mission-control-openapi.yaml`
- `packages/sdk/src/generated/types.ts`
