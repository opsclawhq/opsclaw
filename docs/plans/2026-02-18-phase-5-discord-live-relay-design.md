# Phase 5 Discord Live Relay Transport (05-10) Design

## Scope
Add a live Discord relay path that accepts slash-command interaction payloads and posts squad responses back to Discord channels via bot token auth.

Deliverables:
1. `opsclaw discord live-event` command for live interaction payload handling.
2. Discord bot token resolution (`--bot-token` or env var).
3. Discord API message posting transport bridge.
4. Shared response rendering parity with Slack/Telegram/runtime core.

## Requirement Coverage
- CHAT-04
- CHAT-08
- BOT-07

## Options Considered
1. Full Discord gateway/websocket bot runtime in one slice.
2. Event relay command that consumes interaction payload and posts channel message (recommended).
3. Keep Discord as contract-only while focusing exclusively on MCP.

## Selected Approach
Option 2. It provides immediate API-backed Discord response capability with low operational complexity and keeps parity with the Slack live relay approach.

## Architecture
- Extend `discord_adapter` with:
  - `DiscordApi` trait + `HttpDiscordApi` client
  - token resolver helper
  - live-event handler that:
    - parses slash command payload (including `channel_id`)
    - maps command text through shared `squad_responder`
    - posts reply to Discord channel endpoint
- Add CLI:
  - `opsclaw discord live-event --payload-json ... --bot-token|--bot-token-env ... --template ...`
- Preserve deterministic route/embed/auth helpers as-is.

## Failure Modes
1. Missing `channel_id` in interaction payload.
2. Missing/invalid Discord bot token.
3. Discord API rejects message post.
4. Non-slash events should be ignored cleanly.

## Verification Strategy
- RED:
  - targeted live relay test fails before implementation.
- GREEN:
  - targeted relay tests pass with mock API.
- Full:
  - `cargo test -p opsclaw`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - `bash scripts/docs/validate-release-docs.sh`
