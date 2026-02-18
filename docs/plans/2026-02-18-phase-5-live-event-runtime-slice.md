# Phase 5 Unified Live Event Runtime Bridge (05-11) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add `opsclaw run live-event` so one runtime command can relay Slack/Discord/Telegram live events through native APIs.  
**Architecture:** Add a Telegram one-shot live handler, then wire a runtime-level live dispatcher command that reuses platform adapter live handlers.  
**Tech Stack:** Rust (`serde`, `serde_json`, `ureq`, `clap`)  
**Requirement IDs:** CHAT-08, BOT-07, CHAT-01, CHAT-04, CHAT-05

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-live-event-router`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED tests for Telegram one-shot live handling

**Files:**
- Modify: `crates/opsclaw/src/telegram_adapter.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - routable group/private update posts one message and returns `replied` decision.
  - non-routable update returns `ignore` and does not post.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::live_event_posts_reply_for_group_mention -- --exact
```

Expected: fail before implementation.

### Task 2: Implement Telegram one-shot live handler

**Files:**
- Modify: `crates/opsclaw/src/telegram_adapter.rs`

**Step 1: Add minimal implementation**
- Add `TelegramLiveDecision` enum.
- Add `handle_live_event(...)` reusing existing reply builder and `TelegramApi::send_message`.
- Keep behavior deterministic (`replied|ignore`).

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::live_event_posts_reply_for_group_mention -- --exact
```

Expected: pass.

### Task 3: Wire `opsclaw run live-event`

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 1: Add command + dispatch**
- add `run live-event` subcommand and args (`platform`, `payload_json`, `identity`, template + token args).
- dispatch by platform to existing adapter live handlers.
- print stable JSON decision contract per platform.

**Step 2: Verify integration**

Run:
```bash
cargo test -p opsclaw
```

Expected: pass.

### Task 4: Docs/changelog/evidence

**Files:**
- Modify: `docs/user-guide/multi-platform-routing-preview.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-live-event-runtime-slice.md`

**Step 1: Document live runtime bridge usage and constraints.**
**Step 2: Record RED/GREEN evidence and smoke commands.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::live_event_posts_reply_for_group_mention -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
