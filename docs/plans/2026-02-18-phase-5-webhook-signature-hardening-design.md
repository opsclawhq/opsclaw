# Phase 5 Webhook Signature Hardening (05-17) Design

## Scope
Add Slack request-signature verification for native webhook ingress so `run serve-webhooks` can authenticate Slack-originated events before dispatch.

Deliverables:
1. Slack signature verification helper (HMAC SHA256 over canonical base string).
2. CLI options for Slack signing secret and replay-window tolerance.
3. Deterministic unauthorized error contract (`401` JSON) for invalid/missing signatures.
4. Docs/changelog/evidence updates.

## Requirement Coverage
- CHAT-01
- CHAT-08
- BOT-07

## Options Considered
1. Keep shared-secret-only protection.
2. Add Slack-specific signature verification in runtime ingress (recommended).
3. Add full Slack+Discord signature verification in one slice.

## Selected Approach
Option 2. It is the highest-impact next hardening step for real Slack webhooks, with manageable scope and deterministic validation behavior.

## Architecture
- Extend `webhook_runtime` with Slack verification helper:
  - parse `X-Slack-Request-Timestamp`
  - build canonical base string `v0:{timestamp}:{body}`
  - compute `v0=` + hex(HMAC_SHA256(secret, base_string))
  - compare against `X-Slack-Signature`
  - reject stale timestamps outside tolerance window
- Extend `run serve-webhooks` args:
  - `--slack-signing-secret` (optional)
  - `--slack-signature-tolerance-seconds` (default `300`)
- Enforce check only for `/slack/events` requests when signing secret is configured.

## Failure Modes
1. Secret configured but Slack signature/timestamp header missing -> `401`.
2. Secret configured and signature mismatch -> `401`.
3. Secret configured and timestamp stale -> `401`.
4. No signing secret configured -> behavior unchanged (compatible with local testing).

## Verification Strategy
- RED: signature mismatch test fails before implementation.
- GREEN: helper tests pass.
- Full: package/workspace tests + clippy + docs validator pass.
