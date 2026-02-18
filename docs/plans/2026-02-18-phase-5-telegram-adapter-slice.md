# Phase 5 Telegram Adapter Contracts (05-03) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add deterministic Telegram adapter contracts and CLI commands for group/private routing and inline keyboard output.

**Architecture:** Implement `telegram_adapter` module and wire new `opsclaw telegram` CLI subcommands for local verification.

**Tech Stack:** Rust (`serde`, `serde_json`, `clap`) in `opsclaw` crate

**Requirement IDs:** CHAT-05

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
cargo test -p opsclaw telegram_adapter::tests::routes_group_command_update -- --exact
```

Expected: failure because telegram adapter module/tests do not exist yet.

### Task 2: Add telegram adapter module and tests

**Files:**
- Create: `crates/opsclaw/src/telegram_adapter.rs`

### Task 3: Wire `opsclaw telegram` CLI subcommands

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

### Task 4: Docs and changelog updates

**Files:**
- Create: `docs/user-guide/telegram-adapter-preview.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-telegram-adapter-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw telegram_adapter::tests::routes_group_command_update -- --exact`
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
