# Phase 5 Discord Signature Hardening (05-18) Design

## Scope
Add Discord interaction signature verification for native webhook ingress so `run serve-webhooks` can authenticate `/discord/interactions` requests before dispatch.

Deliverables:
1. Discord Ed25519 signature verification helper.
2. CLI options for Discord public key and replay-window tolerance.
3. Deterministic unauthorized error contract (`401` JSON) for invalid/missing signatures.
4. Docs/changelog/evidence updates.

## Requirement Coverage
- CHAT-04
- CHAT-08
- BOT-07

## Options Considered
1. Keep shared-secret + rate-limit + Slack-signature only.
2. Add Discord-specific signature verification in runtime ingress (recommended).
3. Implement Discord + Telegram signature verification in one slice.

## Selected Approach
Option 2. It extends current ingress hardening with platform-native verification on the next highest-priority endpoint while keeping implementation risk bounded.

## Architecture
- Extend `webhook_runtime` with Discord verification helper:
  - parse `X-Signature-Ed25519` and `X-Signature-Timestamp`
  - verify signature over `timestamp + body` using provided Discord public key
  - reject stale timestamps outside tolerance window
- Extend `run serve-webhooks` args:
  - `--discord-public-key` (optional)
  - `--discord-signature-tolerance-seconds` (default `300`)
- Enforce check only for `/discord/interactions` requests when public key is configured.

## Failure Modes
1. Public key configured but headers missing -> `401`.
2. Signature mismatch or malformed key/signature -> `401`.
3. Timestamp stale -> `401`.
4. No public key configured -> behavior unchanged (local/dev mode).

## Verification Strategy
- RED: signature mismatch test fails before implementation.
- GREEN: helper tests pass.
- Full: package/workspace tests + clippy + docs validator pass.
