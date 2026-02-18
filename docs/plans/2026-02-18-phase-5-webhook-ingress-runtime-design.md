# Phase 5 Native Webhook Ingress Runtime (05-13) Design

## Scope
Add a built-in webhook ingress server so OpsClaw can accept Slack/Discord/Telegram webhook POSTs directly and dispatch them through existing live handlers.

Deliverables:
1. `opsclaw run serve-webhooks` command exposing local HTTP ingress.
2. Path routing for `/slack/events`, `/discord/interactions`, `/telegram/webhook`.
3. Reuse of existing live handlers and API clients for relay behavior.
4. Docs/changelog/evidence updates for end-user webhook operation.

## Requirement Coverage
- CHAT-01
- CHAT-04
- CHAT-05
- CHAT-08
- BOT-07

## Options Considered
1. Keep external ingress only (stdin/one-shot commands).
2. Add built-in HTTP ingress server with deterministic route dispatch (recommended).
3. Build full managed SaaS ingress with auth/session store.

## Selected Approach
Option 2. It closes the main remaining usability gap for live squads while preserving current adapter contracts and keeping implementation contained.

## Architecture
- Add `tiny_http` server runtime in `opsclaw`:
  - bind address configurable via `--bind`.
  - optional `--max-requests` for deterministic local test/smoke runs.
- Introduce `webhook_runtime` module:
  - path-to-platform resolver
  - request dispatch function mapping endpoint+payload to existing handlers:
    - Slack -> `slack_adapter::handle_live_event`
    - Discord -> `discord_adapter::handle_live_event`
    - Telegram -> `telegram_adapter::handle_live_event`
- Use lazy API client initialization and shared template selection.
- JSON response body returns handler decision payload.

## Failure Modes
1. Unsupported path -> clear `400` JSON error.
2. Missing identity for Slack/Telegram route handling -> `400` JSON error.
3. Missing bot token for encountered platform -> `400` JSON error.
4. Invalid JSON payload -> `400` JSON error from adapter parsing.

## Verification Strategy
- RED:
  - targeted path resolver/dispatch test fails before implementation.
- GREEN:
  - targeted test passes.
- Full:
  - `cargo test -p opsclaw`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - `bash scripts/docs/validate-release-docs.sh`
