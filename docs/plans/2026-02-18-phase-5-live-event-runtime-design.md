# Phase 5 Unified Live Event Runtime Bridge (05-11) Design

## Scope
Add one runtime entrypoint that can process a single live event from Slack, Discord, or Telegram and relay the squad response through each platform's real API transport.

Deliverables:
1. `opsclaw run live-event` command for cross-platform live dispatch.
2. One-shot Telegram live event handler (parity with Slack/Discord live handlers).
3. Shared JSON output contract for platform-specific live decisions.
4. User docs/changelog/evidence updates for the unified live bridge workflow.

## Requirement Coverage
- CHAT-08
- BOT-07
- CHAT-01
- CHAT-04
- CHAT-05

## Options Considered
1. Full always-on multi-threaded daemon that hosts all platform loops in one process.
2. Single-event runtime bridge command (`run live-event`) that handles one payload and posts through the matching platform API (recommended).
3. Keep platform-specific `slack|discord|telegram` commands only and skip runtime-level live dispatch.

## Selected Approach
Option 2. It provides an immediate, testable bridge for webhook/process integrations with minimal operational risk and reuses existing platform adapters.

## Architecture
- Extend `telegram_adapter` with:
  - `TelegramLiveDecision` output model for one-shot event handling.
  - `handle_live_event(...)` that:
    - parses one Telegram update payload,
    - builds response via existing routing + responder logic,
    - posts via `TelegramApi::send_message` when needed,
    - returns deterministic `replied|ignore` decisions.
- Extend CLI runtime surface:
  - `opsclaw run live-event --platform <slack|discord|telegram> --payload-json ... --identity ... --template ...`
  - platform-specific token/env args for API auth.
  - route to `slack_adapter::handle_live_event`, `discord_adapter::handle_live_event`, or `telegram_adapter::handle_live_event`.
- Keep existing platform-native commands unchanged (`slack live-event`, `discord live-event`, `telegram live`).

## Failure Modes
1. Missing platform identity where required (`bot_user_id` for Slack, `bot_username` for Telegram).
2. Missing/invalid bot tokens.
3. Platform API rejection during post.
4. Non-routable payloads should return `decision=ignore` cleanly.

## Verification Strategy
- RED:
  - targeted Telegram one-shot live handler test fails before implementation.
- GREEN:
  - targeted Telegram one-shot live handler test passes.
- Full:
  - `cargo test -p opsclaw`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - `bash scripts/docs/validate-release-docs.sh`
