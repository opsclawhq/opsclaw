# Phase 3 Slack Collaboration Contracts Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add intro/discussion/overflow Slack collaboration contracts with CLI helpers for Phase 3 `03-04`.

**Architecture:** Implement `opsclaw::slack_collaboration` as a pure module with deterministic outputs and explicit validation. Wire three CLI helpers for local contract validation without external Slack dependencies.

**Tech Stack:** Rust (`serde`, `serde_json`), existing CLI (`clap`), contract-first module testing

**Requirement IDs:** BOT-06, CHAT-07, COORD-06, COORD-07

---

### Task 1: RED tests for collaboration contracts

**Files:**
- Create: `crates/opsclaw/src/slack_collaboration.rs`

Run (RED):
- `cargo test -p opsclaw slack_collaboration::tests::builds_intro_message_for_agent -- --exact`

Expected: compile failure (module symbols missing).

### Task 2: Implement collaboration contract module

**Files:**
- Create: `crates/opsclaw/src/slack_collaboration.rs`

Functions:
- `build_intro_message(profile)`
- `plan_visible_discussion(task, agents)`
- `prepare_response_for_slack(text, max_chars, snippet_name)`

### Task 3: Wire CLI helper commands

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

Commands:
- `opsclaw slack intro-message --agent-json ...`
- `opsclaw slack plan-discussion --task ... --agents-json ...`
- `opsclaw slack prepare-response --text ... [--max-chars ...] [--snippet-name ...]`

### Task 4: Docs/changelog/evidence

**Files:**
- Modify: `docs/api-reference.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-3-slack-collaboration-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
