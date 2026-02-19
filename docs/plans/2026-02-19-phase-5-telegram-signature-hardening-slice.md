# Phase 5 Telegram Signature Hardening (05-19) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5
**Goal:** Add Telegram webhook secret-token verification for `opsclaw run serve-webhooks`.
**Architecture:** `webhook_runtime` token verifier + serve-webhooks CLI arg + Telegram-path verification.
**Tech Stack:** Rust (`clap`, `tiny_http`)
**Requirement IDs:** CHAT-05, CHAT-08, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-telegram-signature-hardening`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Verification is mandatory before any completion claim

### Task 1: RED tests for Telegram token verification

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::telegram_signature_rejects_mismatch -- --exact
```

Expected: fail before implementation.

### Task 2: Implement helper + tests

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::telegram_signature_rejects_mismatch -- --exact
```

Expected: pass.

### Task 3: Wire `run serve-webhooks` verification path

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
- Create: `docs/plans/evidence/2026-02-19-phase-5-telegram-signature-hardening-slice.md`

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::telegram_signature_rejects_mismatch -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
