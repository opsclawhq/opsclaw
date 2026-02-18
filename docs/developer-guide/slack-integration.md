# Slack Integration Guide (Contributors)

This guide describes the current Slack architecture and extension points for contributors.

## 1. Module Map

Phase 3 Slack behavior is split into pure contract modules under `crates/opsclaw/src/`:

- `slack_adapter.rs`
  - OAuth install URL contract
  - event routing contract
  - retry policy helper
- `slack_approval.rs`
  - approval-card generation from risk planner output
  - interaction action parsing (`approve` / `reject`)
- `slack_collaboration.rs`
  - intro-message generation
  - visible multi-agent discussion planning
  - long-response overflow formatting

`main.rs` exposes these contracts through `opsclaw slack ...` subcommands for deterministic local verification.

## 2. Verification Matrix

Use these commands when changing Slack behavior:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
```

Targeted contract checks:

```bash
cargo test -p opsclaw slack_adapter::tests::routes_app_mention_to_thread -- --exact
cargo test -p opsclaw slack_approval::tests::builds_approval_card_for_mutating_command -- --exact
cargo test -p opsclaw slack_collaboration::tests::plans_visible_discussion_with_specialist_assignment -- --exact
```

## 3. Extending to Live Slack Runtime

Current live relay bridge:

- `opsclaw slack live-event` accepts a raw Events API payload and uses:
  - `route_for_bot` for deterministic mention routing
  - `squad_responder` for shared response rendering
  - Slack `chat.postMessage` API via `HttpSlackApi`

When wiring full embedded server/signature verification next:

1. keep pure contracts in existing modules unchanged where possible
2. add transport and signature verification as separate integration layer
3. route inbound events to existing contract functions first
4. preserve explicit HITL gate behavior (`build-approval-card` + decision parsing)
5. preserve overflow behavior (`prepare-response`) before message post
6. keep `slack live-event` as an integration-testable relay path

## 4. Traceability

Current Phase 3 contract coverage:

- `03-02`: adapter foundations (`CHAT-01`, `CHAT-02`, `CHAT-06`) - PR #24
- `03-03`: approval contracts (`CHAT-03`) - PR #25
- `03-04`: collaboration contracts (`BOT-06`, `CHAT-07`, `COORD-06`, `COORD-07`) - PR #26

Plan and evidence references:

- `docs/plans/2026-02-17-phase-3-slack-adapter-slice.md`
- `docs/plans/2026-02-18-phase-3-slack-approval-cards-slice.md`
- `docs/plans/2026-02-18-phase-3-slack-collaboration-slice.md`
