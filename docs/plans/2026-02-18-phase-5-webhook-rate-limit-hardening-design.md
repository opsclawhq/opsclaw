# Phase 5 Webhook Rate Limit Hardening (05-16) Design

## Scope
Add request-rate limiting for native webhook ingress so `run serve-webhooks` can bound inbound request volume and fail fast with explicit `429` responses.

Deliverables:
1. Rate-limit helper for webhook runtime request accounting.
2. CLI options for max requests per window.
3. Deterministic `429` JSON error on limit exceed.
4. Docs/changelog/evidence updates.

## Requirement Coverage
- CHAT-08
- BOT-07

## Options Considered
1. No runtime rate limiting.
2. Optional in-process global webhook rate limiting (recommended).
3. External reverse-proxy-only rate limiting.

## Selected Approach
Option 2. It gives immediate protection for default self-hosted runs while still allowing external proxy controls in production.

## Architecture
- Add helper in `webhook_runtime`:
  - prune request timestamps outside rolling window
  - reject when request count reaches configured max
- Extend `run serve-webhooks` args:
  - `--webhook-rate-limit-max-requests` (optional)
  - `--webhook-rate-limit-window-seconds` (default `60`)
- Enforce rate limit before payload processing and return `429` JSON.

## Failure Modes
1. Limit exceeded -> `429` and request is not dispatched.
2. Invalid limit args (`0` values) -> startup fails with clear error.
3. No limit configured -> ingress behavior unchanged.

## Verification Strategy
- RED: helper rate-limit test fails before implementation.
- GREEN: helper tests pass.
- Full: package/workspace tests + clippy + docs validator pass.
