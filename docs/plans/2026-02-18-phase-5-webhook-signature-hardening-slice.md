# Phase 5 Webhook Signature Hardening (05-17) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5
**Goal:** Add Slack request-signature verification for `opsclaw run serve-webhooks`.
**Architecture:** `webhook_runtime` helper + serve-webhooks CLI args + Slack-path verification in request loop.
**Tech Stack:** Rust (`hmac`, `sha2`, `hex`, `clap`, `tiny_http`)
**Requirement IDs:** CHAT-01, CHAT-08, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-webhook-signature-hardening`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Verification is mandatory before any completion claim

### Task 1: RED tests for Slack signature verification

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::slack_signature_rejects_mismatch -- --exact
```

Expected: fail before implementation.

### Task 2: Implement verification helper + tests

**Files:**
- Modify: `crates/opsclaw/src/webhook_runtime.rs`
- Modify: `crates/opsclaw/Cargo.toml` (if new deps are needed)

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::slack_signature_rejects_mismatch -- --exact
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
- Create: `docs/plans/evidence/2026-02-18-phase-5-webhook-signature-hardening-slice.md`

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::slack_signature_rejects_mismatch -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
