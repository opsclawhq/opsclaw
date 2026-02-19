# Phase 5 Telegram Signature Hardening (05-19) Design

## Context

`opsclaw run serve-webhooks` already supports ingress shared-secret auth, rate limiting, Slack signature verification, and Discord interaction signature verification. Telegram webhook ingress currently lacks platform-native request authentication parity.

## Goal

Add Telegram webhook secret-token verification at ingress for `/telegram/webhook`.

## Requirement IDs

- CHAT-05
- CHAT-08
- BOT-07

## Interface

- CLI flag on `run serve-webhooks`:
  - `--telegram-webhook-secret-token <token>` (optional)
- Header validation target:
  - `X-Telegram-Bot-Api-Secret-Token`

## Behavior

- When `--telegram-webhook-secret-token` is not set: Telegram signature verification is disabled (existing behavior preserved).
- When set:
  - `/telegram/webhook` must include `X-Telegram-Bot-Api-Secret-Token`.
  - Missing token returns `401` with deterministic JSON error.
  - Mismatched token returns `401` with deterministic JSON error.
  - Matching token proceeds to normal routing.
- Slack and Discord verification behavior remains unchanged.

## Failure Modes

- Missing header -> `missing telegram webhook secret token`
- Wrong header value -> `invalid telegram webhook secret token`
- No config provided -> verification bypassed intentionally.

## Test Strategy

- Unit tests in `webhook_runtime` for Telegram token verifier:
  - rejects missing when configured
  - rejects mismatch
  - accepts match
  - allows when not configured
- Runtime path verification in `main.rs` exercised via existing package/workspace tests.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
