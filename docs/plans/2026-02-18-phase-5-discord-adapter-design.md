# Phase 5 Discord Adapter Contracts (05-02) Design

## Scope
Implement Discord adapter contract baseline with:

1. slash-command routing contract
2. embed payload rendering contract
3. role-based permission check helper

## Requirement Coverage
- CHAT-04

## Options Considered

1. Build live Discord gateway transport first.
2. Build deterministic adapter contracts and CLI verification first (recommended).
3. Defer Discord until after Telegram.

## Selected Approach
Option 2. It mirrors the Slack rollout strategy: contract-first behavior with deterministic tests before live networking.

## Architecture

- Add `discord_adapter` module under `crates/opsclaw/src`.
- Expose `opsclaw discord ...` CLI subcommands for local contract checks.
- Include tests for:
  - slash-command routing
  - embed payload shape
  - role-authorization decisions

## Failure Modes

- invalid command payload accepted and routed
- embed payload missing required fields
- role check allows unauthorized action

## Verification Strategy

- RED: targeted discord adapter test before module exists
- GREEN: discord adapter tests pass for routing/embed/authorization cases
- full: workspace tests and clippy remain green
