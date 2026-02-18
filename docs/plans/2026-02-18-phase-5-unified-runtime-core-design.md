# Phase 5 Unified Runtime Core (05-08) Design

## Scope
Deliver a single `opsclaw run` runtime core that processes Slack, Discord, and Telegram events through one shared squad response engine.

Deliverables:
1. New `opsclaw run route-event` command for platform-agnostic inbound event handling + squad response output.
2. New `opsclaw run stdio` mode that reads NDJSON inbound events and emits NDJSON squad responses.
3. Shared squad response logic reused by Telegram live runtime and unified runtime core.

## Requirement Coverage
- CHAT-08
- BOT-07
- CHAT-05

## Options Considered
1. Build full live Slack/Discord/Telegram transport wiring in one slice.
2. Build a unified runtime core first, with deterministic multi-platform routing + shared responder logic (recommended).
3. Keep per-platform command silos and defer unification.

## Selected Approach
Option 2. It creates an immediately testable single runtime behavior layer and removes per-platform response drift, while keeping future live transport wiring incremental.

## Architecture
- Add `squad_responder` module:
  - template-aware squad intros/help/member lists
  - deterministic message-to-agent assignment
- Add `squad_runtime` module:
  - parse unified inbound payload (`platform`, `payload_json`, `identity`)
  - call `channels_router::route_platform_event`
  - map routed events to squad responses
  - run bounded/unbounded stdin NDJSON loop
- Add CLI:
  - `opsclaw run route-event ...`
  - `opsclaw run stdio ...`
- Refactor Telegram live runtime to use `squad_responder` for consistent replies.

## Failure Modes
1. Invalid inbound NDJSON payloads in stdio mode.
2. Platform identity missing for Slack/Telegram route handling.
3. Response drift between Telegram live runtime and unified runtime command.
4. Non-routable events creating noisy outputs.

## Verification Strategy
- RED:
  - targeted runtime parity test fails before module implementation
- GREEN:
  - targeted parity and stdio processing tests pass
- Full:
  - `cargo test -p opsclaw`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - `bash scripts/docs/validate-release-docs.sh`
