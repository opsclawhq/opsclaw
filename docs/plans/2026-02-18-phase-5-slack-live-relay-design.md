# Phase 5 Slack Live Relay Transport (05-09) Design

## Scope
Add a live Slack relay path that accepts real Slack Events API payloads and posts responses back to Slack via `chat.postMessage`.

Deliverables:
1. `opsclaw slack live-event` command for live payload handling.
2. Bot token resolution (`--bot-token` or env var) for Slack API calls.
3. Shared squad response rendering via `squad_responder`.
4. Docs updates for operator and contributor live-relay usage.

## Requirement Coverage
- CHAT-01
- CHAT-02
- CHAT-08

## Options Considered
1. Full embedded Slack web server and signature verification in one slice.
2. Event relay command that handles payload + response posting, with external webhook ingress (recommended).
3. Keep Slack as contract-only while prioritizing Discord transport.

## Selected Approach
Option 2. It unlocks real Slack API posting now with lower risk and cleanly composes with existing contract modules while deferring server/signature concerns to the next integration slice.

## Architecture
- Extend `slack_adapter` with:
  - `SlackApi` trait and `HttpSlackApi` implementation (`chat.postMessage`)
  - token resolver helper
  - live-event handler that:
    - accepts Slack payload
    - uses existing `route_for_bot`
    - maps routed mentions to shared squad responder output
    - posts reply to channel/thread
- Add CLI subcommand:
  - `opsclaw slack live-event --bot-user-id ... --payload-json ... --bot-token|--bot-token-env ... --template ...`
- Reuse shared `squad_responder` behavior for command parity with Telegram and `opsclaw run`.

## Failure Modes
1. Missing/empty Slack bot token.
2. Slack API rejects `chat.postMessage`.
3. Mention routing missing channel/thread fields.
4. URL verification payloads need challenge passthrough without message post.

## Verification Strategy
- RED:
  - targeted live relay test fails before implementation.
- GREEN:
  - targeted live relay tests pass with mock API.
- Full:
  - `cargo test -p opsclaw`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - `bash scripts/docs/validate-release-docs.sh`
