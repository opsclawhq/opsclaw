# Phase 5 Live Telegram Squad Runtime (05-06) Design

## Scope
Implement the first end-user live Telegram path so an operator can connect a real bot token, run a live polling loop, and chat with a squad-style responder in private or group chats.

Deliverables:
1. `opsclaw telegram live` command that long-polls Telegram updates and sends replies.
2. Bot token resolution via CLI flag or environment variable.
3. Squad-oriented command responses (`/start`, `/help`, `/squad`) plus free-text/mention handling.
4. User docs for launch, connect, and test flow.

## Requirement Coverage
- BOT-04
- BOT-05
- CHAT-05

## Options Considered
1. Webhook-first runtime with embedded HTTP server.
2. Long-polling runtime via Telegram `getUpdates` API (recommended).
3. Defer live runtime and keep contract-only adapter behavior.

## Selected Approach
Option 2. Long polling has the fastest path to working end-user behavior in local and self-hosted environments, avoids webhook TLS/exposure complexity, and can be covered with deterministic unit tests by isolating API calls behind a transport trait.

## Architecture
- Add live runtime support to `telegram_adapter`:
  - `TelegramApi` trait for `getUpdates` and `sendMessage` operations.
  - `HttpTelegramApi` implementation using `ureq` for blocking HTTP.
  - `run_live_session` loop that tracks update offsets and dispatches replies.
- Reuse existing `route_telegram_update` routing contract for mention/command detection.
- Add deterministic squad response helpers:
  - `build_start_message(template)`
  - `build_help_message()`
  - `build_squad_message(template)`
  - `build_agent_reply(template, text)`
- Wire CLI:
  - `opsclaw telegram live --bot-username <name> [--bot-token ...|--bot-token-env ...] [--template ...] [--max-updates ...]`
- Keep first live slice single-process and synchronous; no daemon/service manager required yet.

## Failure Modes
1. Missing token: command exits with a clear action message.
2. Telegram API transient errors: loop reports error and exits non-zero.
3. Group routing noise: non-mention group chatter remains ignored through existing router behavior.
4. Empty messages or unsupported updates: ignored without crash.

## Verification Strategy
- RED:
  - targeted unit test for token resolution and reply generation before implementation.
  - targeted unit test proving live loop emits replies for routed updates with mock API.
- GREEN:
  - targeted tests pass.
  - `cargo test -p opsclaw` passes.
- Full:
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - manual smoke command documented for live Telegram bot.
