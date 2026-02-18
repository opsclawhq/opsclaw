# Phase 5 Webhook Auth Hardening (05-14) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add shared-secret request validation for `opsclaw run serve-webhooks`.  
**Architecture:** Add webhook auth validation helper + CLI args, enforce in request loop before payload dispatch.  
**Tech Stack:** Rust (`tiny_http`, `serde_json`, `clap`)  
**Requirement IDs:** CHAT-08, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-webhook-auth-hardening`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Verification is mandatory before any completion claim

### Task 1: RED tests for webhook auth validation

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::rejects_missing_secret_when_required -- --exact
```

Expected: fail before implementation.

### Task 2: Implement auth validation helper

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::rejects_missing_secret_when_required -- --exact
```

Expected: pass.

### Task 3: Wire auth into `run serve-webhooks`

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
- Create: `docs/plans/evidence/2026-02-18-phase-5-webhook-auth-hardening-slice.md`

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::rejects_missing_secret_when_required -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
