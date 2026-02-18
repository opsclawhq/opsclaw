# Phase 5 Live Telegram Squad Runtime (05-06) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Deliver a real `opsclaw telegram live` runtime so users can connect a Telegram bot and chat with a squad-style responder.  
**Architecture:** Introduce a long-poll Telegram transport behind a testable API trait, reuse existing routing logic, and wire a CLI command for live polling plus deterministic squad responses.  
**Tech Stack:** Rust (`clap`, `serde`, `serde_json`, `ureq`)  
**Requirement IDs:** BOT-04, BOT-05, CHAT-05

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-telegram-live-squad`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED tests for live runtime primitives

**Files:**
- Modify: `crates/opsclaw/src/telegram_adapter.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - token resolution precedence (`--bot-token` over env var).
  - live update processing emits `sendMessage` for routed update.
  - `/squad` response contains template squad identities.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::live_loop_replies_to_group_mention -- --exact
```

Expected: fail because live loop helpers do not exist yet.

### Task 2: Implement live Telegram runtime (GREEN)

**Files:**
- Modify: `crates/opsclaw/src/telegram_adapter.rs`
- Modify: `crates/opsclaw/Cargo.toml`

**Step 1: Add minimal implementation**
- Add token resolution helper.
- Add API trait + HTTP adapter.
- Add update processing function and long-poll loop.
- Add command response builders.

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::live_loop_replies_to_group_mention -- --exact
```

Expected: pass.

### Task 3: Wire CLI command

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 1: Add `telegram live` subcommand**
- Parse token args/env, template, and update limit.
- Call live loop entrypoint and print startup/summary logs.

**Step 2: Verify CLI integration**

Run:
```bash
cargo test -p opsclaw
```

Expected: pass with telegram module and CLI tests.

### Task 4: User docs + delivery docs

**Files:**
- Modify: `docs/user-guide/telegram-adapter-preview.md`
- Modify: `docs/user-guide/setup-wizard-preview.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-telegram-live-squad-slice.md`

**Step 1: Document real operator flow**
- BotFather setup steps.
- command to launch live loop.
- test scenarios: private chat, group mention, `/squad`.

**Step 2: Record RED/GREEN evidence commands and outputs.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::live_loop_replies_to_group_mention -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
```

Expected:
- all commands exit 0.
- no failing tests or clippy diagnostics.

## Test Cases and Scenarios

1. Live mention route: group `@bot` mention produces a reply.
2. Private chat route: plain text produces a reply.
3. Command route: `/squad` returns template member list.
4. Token resolution: missing token fails with explicit message.
5. Loop control: `--max-updates` stops loop predictably for test/smoke runs.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
