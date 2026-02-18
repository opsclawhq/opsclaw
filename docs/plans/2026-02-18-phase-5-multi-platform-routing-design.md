# Phase 5 Multi-Platform Routing Contracts (05-04) Design

## Scope
Implement a platform-agnostic routing contract baseline that normalizes Slack, Discord, and Telegram events into a single route shape.

1. unified routing decision contract
2. per-platform adapter bridge (Slack, Discord, Telegram)
3. CLI verification surface for cross-platform event routing

## Requirement Coverage
- CHAT-08

## Options Considered

1. Build full runtime transport fan-out first.
2. Build deterministic cross-platform routing contracts first (recommended).
3. Defer multi-platform until content automation is complete.

## Selected Approach
Option 2. It keeps Phase 5 incremental: each platform contract is validated first, then multi-platform orchestration is validated with a deterministic router before live runtime wiring.

## Architecture

- Add `channels_router` module in `crates/opsclaw/src`.
- Reuse existing `slack_adapter`, `discord_adapter`, and `telegram_adapter` routing contracts.
- Normalize platform decisions into a unified route model:
  - `platform`
  - `route_kind`
  - `target_ref`
  - `text`
- Expose `opsclaw channels route-event` for local contract validation.

## Failure Modes

- platform-specific payload is routed without required identity context
- normalized routing loses target reference and cannot respond on source platform
- non-routable events are incorrectly forwarded

## Verification Strategy

- RED: targeted multi-platform router test before module exists
- GREEN: router tests pass for Slack, Discord, and Telegram inputs
- full: workspace tests and clippy remain green
