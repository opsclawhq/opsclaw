# Phase 5 Live Stdio Orchestrator Bridge (05-12) Design

## Scope
Add a continuous runtime bridge loop that reads NDJSON live events from stdin and dispatches them to Slack/Discord/Telegram live handlers in one process.

Deliverables:
1. `opsclaw run live-stdio` command for continuous multi-platform live dispatch.
2. Shared loop primitive for parsing NDJSON events and emitting JSON decisions.
3. Lazy platform API client wiring (Slack/Discord/Telegram) reused across loop iterations.
4. Docs/changelog/evidence updates for always-on runtime bridge usage.

## Requirement Coverage
- CHAT-08
- BOT-07
- CHAT-01
- CHAT-04
- CHAT-05

## Options Considered
1. Full internal webhook servers and Telegram poll loop in one runtime command.
2. NDJSON live-stdio bridge that accepts inbound events from external ingress adapters and dispatches continuously (recommended).
3. Keep single-event `run live-event` only.

## Selected Approach
Option 2. It provides an immediately useful always-on process model without requiring embedded HTTP servers in this slice.

## Architecture
- Add `live_runtime` module:
  - NDJSON loop that parses `RuntimeInboundEvent` lines.
  - callback-based dispatch for platform-specific handling.
  - structured loop outcome (`events_processed`, `decisions_emitted`).
- Extend CLI runtime surface:
  - `opsclaw run live-stdio --template ... [--max-events ...]` + token/env args.
  - closure dispatcher that lazily initializes Slack/Discord/Telegram API clients and dispatches each inbound event.
  - writes one JSON decision line per processed event.
- Keep existing one-shot command (`run live-event`) unchanged.

## Failure Modes
1. Invalid NDJSON line should fail fast with clear error.
2. Unsupported platform should fail with explicit message.
3. Missing identity for Slack/Telegram events should fail with explicit message.
4. Missing token/env for encountered platform should fail with explicit message.

## Verification Strategy
- RED:
  - targeted `live_runtime` loop test fails before module implementation.
- GREEN:
  - targeted loop test passes.
- Full:
  - `cargo test -p opsclaw`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - `bash scripts/docs/validate-release-docs.sh`
