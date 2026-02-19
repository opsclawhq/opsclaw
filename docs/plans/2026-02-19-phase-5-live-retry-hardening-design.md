# Phase 5 Live Retry Hardening (05-20) Design

## Context

Live relay paths (`run serve-webhooks`, live event dispatch handlers) currently fail on the first transient transport error. That is brittle for temporary network failures and rate-limit windows.

## Goal

Add bounded retry controls with deterministic backoff behavior for live relay execution.

## Requirement IDs

- CHAT-08
- BOT-07
- CHAT-01

## Interface

`opsclaw run serve-webhooks` gains:
- `--live-retry-max-attempts <n>` (default: `1`)
- `--live-retry-backoff-millis <ms>` (default: `250`)

## Behavior

- Retry loop wraps live dispatch execution for Slack/Discord/Telegram relay calls.
- Retries only happen for retryable errors (`429`, `rate_limited`, transport/request failures, timeout/connection failures).
- Retry count is bounded by `--live-retry-max-attempts`.
- Delay policy:
  - if error includes `retry_after_seconds=<n>`, wait `n * 1000` ms
  - else exponential backoff from base delay (`base * 2^(attempt-1)`).

## Validation

- Reject invalid retry config:
  - `--live-retry-max-attempts` must be > 0
  - `--live-retry-backoff-millis` must be > 0

## Test Strategy

- Add unit-tested retry policy module for:
  - retryable/non-retryable classification
  - retry-after parsing
  - computed delay schedule
- Add RED->GREEN test cycle for parser/classifier behavior.
- Verify package/workspace tests and docs validation gates.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
