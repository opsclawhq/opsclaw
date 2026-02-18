# Phase 3 Slack Approval Cards Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add Slack approval card generation and interaction parsing contracts for HITL decisions.

**Architecture:** Implement `opsclaw::slack_approval` as a pure module that bridges `oax_tools::approval` with Slack Block Kit payload generation and action parsing. Add CLI helpers for local verification without external Slack dependencies.

**Tech Stack:** Rust (`serde`, `serde_json`), existing `oax-tools` approval planner, CLI (`clap`)

**Requirement IDs:** CHAT-03, SAFE-02, SAFE-03, SAFE-09

---

### Task 1: RED tests for approval card and interaction parsing

**Files:**
- Create: `crates/opsclaw/src/slack_approval.rs`

Run (RED):
- `cargo test -p opsclaw slack_approval::tests::builds_approval_card_for_mutating_command -- --exact`

Expected: compile failure (module symbols missing).

### Task 2: Implement approval contract module

**Files:**
- Create: `crates/opsclaw/src/slack_approval.rs`
- Modify: `crates/opsclaw/Cargo.toml`

Functions:
- `build_approval_card(run_id, command, rollback_template)`
- `card_to_block_kit_json(card)`
- `parse_interaction_decision(payload_json)`

### Task 3: Wire CLI helper commands

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

Commands:
- `opsclaw slack build-approval-card --run-id ... --command ... [--rollback-template ...]`
- `opsclaw slack parse-interaction --payload-json ...`

### Task 4: Docs/changelog/evidence

**Files:**
- Modify: `docs/api-reference.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-3-slack-approval-cards-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
