# Phase 5 Telegram Onboarding Verify (05-15) Design

## Scope
Add a Telegram onboarding verification command so users can validate bot credentials and connectivity before running the live transport loop.

Deliverables:
1. Telegram bot identity verification helper (`getMe` + optional username check).
2. CLI command `opsclaw telegram verify` with JSON output contract.
3. Optional connectivity ping to a target chat id.
4. User/docs/changelog/evidence updates.

## Requirement Coverage
- CHAT-05
- BOT-05
- BOT-07

## Options Considered
1. Keep onboarding manual in docs only.
2. Add deterministic verification command with API-backed checks (recommended).
3. Fully interactive wizard provisioning in this slice.

## Selected Approach
Option 2. It closes the largest end-user uncertainty (`is my bot token and route valid?`) with minimal complexity and immediate operational value.

## Architecture
- Extend `telegram_adapter` with bot identity model + verify helper:
  - resolve token -> call Telegram `getMe` -> parse identity
  - optional expected username match validation
  - optional ping send to `chat_id`
- Add `telegram verify` command in CLI:
  - inputs: bot token/env, optional expected username, optional ping chat id, template label for ping text
  - output: structured JSON status for scripts/UAT

## Failure Modes
1. Invalid token -> clear verification error.
2. Username mismatch -> explicit expected vs actual message.
3. Ping chat id invalid/not started chat -> explicit send failure.

## Verification Strategy
- RED: username mismatch test fails before helper implementation.
- GREEN: helper tests pass.
- Full gate: package/workspace tests + clippy + docs validator pass.
