# Phase 5 Telegram Adapter Contracts (05-03) Design

## Scope
Implement Telegram adapter contract baseline with:

1. group chat and private chat routing contract
2. inline keyboard rendering contract
3. mention/command extraction helper for deterministic handling

## Requirement Coverage
- CHAT-05

## Options Considered

1. Build live Telegram transport with bot token and webhook polling first.
2. Build deterministic adapter contracts and CLI verification first (recommended).
3. Defer Telegram until after multi-platform routing.

## Selected Approach
Option 2. Keep rollout symmetric with Slack and Discord slices: implement deterministic behavior + tests first, then wire live transport in later slices.

## Architecture

- Add `telegram_adapter` module under `crates/opsclaw/src`.
- Expose `opsclaw telegram ...` CLI subcommands for local contract checks.
- Include tests for:
  - command/mention routing in private/group updates
  - inline keyboard payload shape
  - group support detection logic

## Failure Modes

- group update payload is misclassified and skipped
- inline keyboard payload misses required `inline_keyboard` nesting
- mention extraction routes unrelated messages

## Verification Strategy

- RED: targeted telegram adapter test before module exists
- GREEN: telegram adapter tests pass for routing/keyboard/group cases
- full: workspace tests and clippy remain green
