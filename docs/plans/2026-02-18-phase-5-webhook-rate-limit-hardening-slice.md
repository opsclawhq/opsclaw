# Phase 5 Webhook Rate Limit Hardening (05-16) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5
**Goal:** Add optional in-process request rate limiting for `opsclaw run serve-webhooks`.
**Architecture:** `webhook_runtime` rate-limit helper + new CLI args + request-loop enforcement with 429 responses.
**Tech Stack:** Rust (`tiny_http`, `std::collections::VecDeque`, `clap`)
**Requirement IDs:** CHAT-08, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-webhook-rate-limit-hardening`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Verification is mandatory before any completion claim

### Task 1: RED tests for rate-limit helper

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::rate_limit_rejects_when_window_is_full -- --exact
```

Expected: fail before implementation.

### Task 2: Implement helper + tests

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::rate_limit_rejects_when_window_is_full -- --exact
```

Expected: pass.

### Task 3: Wire runtime CLI options and 429 handling

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

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
- Create: `docs/plans/evidence/2026-02-18-phase-5-webhook-rate-limit-hardening-slice.md`

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::rate_limit_rejects_when_window_is_full -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
