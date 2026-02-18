# Phase 5 Multi-Platform Routing Contracts (05-04) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add platform-agnostic routing contracts that normalize Slack, Discord, and Telegram events.

**Architecture:** Implement `channels_router` module and wire `opsclaw channels route-event` for local verification.

**Tech Stack:** Rust (`serde`, `serde_json`, `clap`) in `opsclaw` crate

**Requirement IDs:** CHAT-08

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
cargo test -p opsclaw channels_router::tests::routes_slack_mention_into_unified_event -- --exact
```

Expected: failure because multi-platform router module/tests do not exist yet.

### Task 2: Add channels router module and tests

**Files:**
- Create: `crates/opsclaw/src/channels_router.rs`

### Task 3: Wire `opsclaw channels route-event` command

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

### Task 4: Docs and changelog updates

**Files:**
- Create: `docs/user-guide/multi-platform-routing-preview.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-multi-platform-routing-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw channels_router::tests::routes_slack_mention_into_unified_event -- --exact`
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
