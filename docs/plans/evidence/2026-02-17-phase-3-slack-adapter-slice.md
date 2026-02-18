# Phase 3 Slack Adapter Slice Evidence

Date: 2026-02-17  
Plan: `docs/plans/2026-02-17-phase-3-slack-adapter-slice.md`

## RED -> GREEN

1. RED: Added Slack adapter tests before implementing adapter symbols.
2. RED verification command:
   - `cargo test -p opsclaw slack_adapter::tests::routes_app_mention_to_thread -- --exact`
   - Result: compile failure for missing `SlackInstallConfig`, `build_install_url`, `route_for_bot`, and `retry_after_seconds`.
3. GREEN: Implemented `opsclaw::slack_adapter` plus CLI wiring in `opsclaw slack ...` commands.
4. GREEN verification command:
   - `cargo test -p opsclaw slack_adapter::tests::routes_app_mention_to_thread -- --exact`
   - Result: pass.

## Full Verification

- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`

## Coverage Notes

- OAuth install URL generation validated for required query fields.
- Mention routing validated for thread inheritance and `ts` fallback.
- URL verification challenge handling validated.
- Rate-limit extraction validated for 429 + `Retry-After` behavior.
