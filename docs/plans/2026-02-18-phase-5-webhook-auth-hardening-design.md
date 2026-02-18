# Phase 5 Webhook Auth Hardening (05-14) Design

## Scope
Add request-level secret validation for native webhook ingress so `run serve-webhooks` can reject unauthorized inbound POSTs.

Deliverables:
1. Optional shared-secret guard in webhook runtime path.
2. Configurable header name for secret transmission.
3. Deterministic unauthorized error contract (`401` JSON error).
4. Docs/changelog/evidence updates.

## Requirement Coverage
- CHAT-08
- BOT-07

## Options Considered
1. No ingress auth in runtime (current behavior).
2. Optional shared-secret header validation for all webhook endpoints (recommended).
3. Full per-platform signature verification in one slice.

## Selected Approach
Option 2. It provides immediate ingress access control with low complexity and prepares the path for platform-specific signature verification in a later slice.

## Architecture
- Extend `webhook_runtime` module with auth helper:
  - compare required secret (if configured) against header-provided secret.
  - explicit errors for missing/invalid secrets.
- Extend `run serve-webhooks` args:
  - `--webhook-shared-secret` (optional)
  - `--webhook-secret-header` (default `X-OpsClaw-Webhook-Secret`)
- In request loop, enforce auth before dispatching to platform handlers.

## Failure Modes
1. Secret configured but header missing -> `401`.
2. Secret configured but header mismatch -> `401`.
3. No secret configured -> ingress remains open for local/dev use.

## Verification Strategy
- RED: auth helper test fails before implementation.
- GREEN: auth helper tests pass.
- Full: package/workspace tests + clippy + docs validator pass.
