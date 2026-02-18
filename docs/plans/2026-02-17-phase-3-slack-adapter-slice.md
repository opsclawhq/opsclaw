# Phase 3 Slack Adapter Foundation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add tested Slack adapter foundation logic for OAuth URL generation, mention routing, thread behavior, and rate-limit policy.

**Architecture:** Create a standalone `opsclaw::slack_adapter` module with pure functions and typed enums. Keep external integration out of scope for this slice; expose logic through small CLI helpers for deterministic local verification.

**Tech Stack:** Rust (`serde`, `serde_json`), CLI (`clap`), unit tests

**Requirement IDs:** CHAT-01, CHAT-02, CHAT-06

---

### Task 1: RED tests for Slack adapter behavior

**Files:**
- Create: `crates/opsclaw/src/slack_adapter.rs`

**Step 1:** Add tests for:
- OAuth URL fields
- mention routing + thread behavior
- non-mention ignore behavior
- rate-limit policy behavior

**Step 2 (RED):**
Run: `cargo test -p opsclaw slack_adapter::tests::routes_app_mention_to_thread -- --exact`
Expected: compile failure for missing adapter symbols.

### Task 2: Implement Slack adapter module

**Files:**
- Create: `crates/opsclaw/src/slack_adapter.rs`
- Modify: `crates/opsclaw/Cargo.toml`

**Step 3:** Implement minimal functions:
- `build_install_url`
- `parse_event`
- `route_for_bot`
- `retry_after_seconds`

**Step 4 (GREEN):**
Run: `cargo test -p opsclaw slack_adapter::tests::routes_app_mention_to_thread -- --exact`
Expected: pass.

### Task 3: Wire CLI helper commands

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 5:** Add `opsclaw slack install-url ...` and `opsclaw slack route-event ...` commands.

### Task 4: Docs, evidence, and changelog updates

**Files:**
- Modify: `docs/api-reference.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-17-phase-3-slack-adapter-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`

