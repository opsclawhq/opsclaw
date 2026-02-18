# Mission Control Economics and Conversation Viewer (Phase 4 Slice 04-04)

This guide covers the economics/ROI and transcript-viewer surfaces added to the dashboard package.

## Package Scope

- `packages/dashboard`

## New Surfaces

- `EconomicsPanel`: per-agent cost/utilization rows and top-line ROI summary
- `ConversationViewerPanel`: selectable transcript with tool-call command/output visibility

## View-Model Layer

- `packages/dashboard/src/lib/economics-conversation.mjs`

Exports:

- `buildEconomicsRows`
- `buildRoiSummary`
- `buildConversationTranscript`

## Mock Contract Data

- `packages/dashboard/src/lib/mock-data.mjs`
  - `mockEconomicsSnapshot`
  - `mockConversations`

## Deterministic Tests

- `packages/dashboard/tests/economics-conversation.test.mjs`

Coverage in this test file:

- spend/usage row mapping
- ROI ratio and rounded totals
- transcript sorting and missing-conversation handling

## Verification

```bash
npm --prefix packages/dashboard test
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Next Integration Step

Replace mock economics/conversation payloads with backend REST endpoints and live conversation event ingestion from Mission Control stream transport.
