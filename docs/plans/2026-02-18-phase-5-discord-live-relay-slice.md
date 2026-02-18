# Phase 5 Discord Live Relay Transport (05-10) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add a live Discord event relay that posts squad responses to channels from slash-command payloads.  
**Architecture:** Extend `discord_adapter` with API transport + token resolution + live-event handler; expose via `opsclaw discord live-event`.  
**Tech Stack:** Rust (`serde`, `serde_json`, `ureq`, `clap`)  
**Requirement IDs:** CHAT-04, CHAT-08, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-discord-live-relay`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED tests for live relay behavior

**Files:**
- Modify: `crates/opsclaw/src/discord_adapter.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - slash payload with `channel_id` posts message through mock API.
  - non-slash payload is ignored without posting.
  - missing `channel_id` fails with explicit error.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw discord_adapter::tests::live_event_posts_channel_message -- --exact
```

Expected: fail before implementation.

### Task 2: Implement Discord live relay internals

**Files:**
- Modify: `crates/opsclaw/src/discord_adapter.rs`
- Modify: `crates/opsclaw/src/squad_responder.rs`

**Step 1: Add minimal implementation**
- Add `channel_id` extraction to slash route contract.
- Add token resolver + API trait/client + live-event handler.
- Route command text through shared responder.

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw discord_adapter::tests::live_event_posts_channel_message -- --exact
```

Expected: pass.

### Task 3: Wire `opsclaw discord live-event`

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 1: Add command + JSON output contract**
- parse payload/token/template options
- execute live-event handler and print decision JSON

**Step 2: Verify integration**

Run:
```bash
cargo test -p opsclaw
```

Expected: pass.

### Task 4: Docs/changelog/evidence

**Files:**
- Modify: `docs/user-guide/discord-adapter-preview.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-discord-live-relay-slice.md`

**Step 1: Document live relay usage and constraints.**
**Step 2: Record RED/GREEN evidence.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw discord_adapter::tests::live_event_posts_channel_message -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
