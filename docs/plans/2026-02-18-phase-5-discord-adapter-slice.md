# Phase 5 Discord Adapter Contracts (05-02) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add deterministic Discord adapter contracts and CLI commands for slash command routing, embed output, and role checks.

**Architecture:** Implement `discord_adapter` module and wire new `opsclaw discord` CLI subcommands for local verification.

**Tech Stack:** Rust (`serde`, `serde_json`, `clap`) in `opsclaw` crate

**Requirement IDs:** CHAT-04

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
cargo test -p opsclaw discord_adapter::tests::routes_slash_command_payload -- --exact
```

Expected: failure because discord adapter module/tests do not exist yet.

### Task 2: Add discord adapter module and tests

**Files:**
- Create: `crates/opsclaw/src/discord_adapter.rs`

### Task 3: Wire `opsclaw discord` CLI subcommands

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

### Task 4: Docs and changelog updates

**Files:**
- Create: `docs/user-guide/discord-adapter-preview.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-discord-adapter-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw discord_adapter::tests::routes_slash_command_payload -- --exact`
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
