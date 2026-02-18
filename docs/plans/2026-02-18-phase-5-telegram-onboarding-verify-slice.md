# Phase 5 Telegram Onboarding Verify (05-15) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5
**Goal:** Provide a user-facing `opsclaw telegram verify` path for Telegram bot setup validation.
**Architecture:** Add `getMe`/verification helper in `telegram_adapter`, wire CLI verify command, add docs/evidence.
**Tech Stack:** Rust (`ureq`, `serde_json`, `clap`)
**Requirement IDs:** CHAT-05, BOT-05, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-telegram-onboarding-verify`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Verification is mandatory before any completion claim

### Task 1: RED tests for verification helper

**Files:**
- Modify: `crates/opsclaw/src/telegram_adapter.rs`

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::verify_bot_identity_rejects_username_mismatch -- --exact
```

Expected: fail before implementation.

### Task 2: Implement adapter verification helper + HTTP `getMe`

**Files:**
- Modify: `crates/opsclaw/src/telegram_adapter.rs`

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::verify_bot_identity_rejects_username_mismatch -- --exact
```

Expected: pass.

### Task 3: Wire `opsclaw telegram verify` CLI command

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

Run:
```bash
cargo test -p opsclaw
```

Expected: pass.

### Task 4: Docs/changelog/evidence

**Files:**
- Modify: `docs/user-guide/telegram-adapter-preview.md`
- Modify: `docs/user-guide/setup-wizard-preview.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-telegram-onboarding-verify-slice.md`

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw telegram_adapter::tests::verify_bot_identity_rejects_username_mismatch -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
