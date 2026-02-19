# Phase 5 Live Retry Hardening (05-20) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5
**Goal:** Add bounded retry + backoff controls for live relay runtime paths.
**Architecture:** new retry policy helper module + `run serve-webhooks` retry config flags + live dispatch retry wrapper.
**Tech Stack:** Rust (`std::thread::sleep`, `std::time::Duration`, `clap`)
**Requirement IDs:** CHAT-08, BOT-07, CHAT-01

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-live-retry-hardening`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Verification is mandatory before any completion claim

### Task 1: RED tests for retry policy parsing/classification

**Files:**
- Create: `crates/opsclaw/src/retry_runtime.rs`

Run:
```bash
cargo test -p opsclaw retry_runtime::tests::parses_retry_after_seconds_from_error -- --exact
```

Expected: fail before implementation.

### Task 2: Implement retry policy helper + tests

**Files:**
- Create: `crates/opsclaw/src/retry_runtime.rs`

Run:
```bash
cargo test -p opsclaw retry_runtime::tests::parses_retry_after_seconds_from_error -- --exact
```

Expected: pass.

### Task 3: Wire runtime retry controls

**Files:**
- Modify: `crates/opsclaw/src/main.rs`
- Modify: `crates/opsclaw/src/slack_adapter.rs`

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
- Create: `docs/plans/evidence/2026-02-19-phase-5-live-retry-hardening-slice.md`

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw retry_runtime::tests::parses_retry_after_seconds_from_error -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
