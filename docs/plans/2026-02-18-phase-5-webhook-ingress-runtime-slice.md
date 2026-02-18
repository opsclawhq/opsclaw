# Phase 5 Native Webhook Ingress Runtime (05-13) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add `opsclaw run serve-webhooks` to receive Slack/Discord/Telegram webhook POSTs directly and relay through live handlers.  
**Architecture:** Create a webhook runtime module with deterministic endpoint dispatch and wire a tiny_http server command in `run`.  
**Tech Stack:** Rust (`tiny_http`, `serde`, `serde_json`, `ureq`, `clap`)  
**Requirement IDs:** CHAT-01, CHAT-04, CHAT-05, CHAT-08, BOT-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-webhook-ingress`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED tests for webhook path routing

**Files:**
- Create: `crates/opsclaw/src/webhook_runtime.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - supported endpoint path resolves to Slack/Discord/Telegram platform.
  - unsupported path returns explicit error.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::resolves_supported_paths -- --exact
```

Expected: fail before implementation.

### Task 2: Implement webhook runtime helpers

**Files:**
- Create: `crates/opsclaw/src/webhook_runtime.rs`

**Step 1: Add minimal implementation**
- add `WebhookPlatform` enum.
- add `platform_from_path(...)` resolver.

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::resolves_supported_paths -- --exact
```

Expected: pass.

### Task 3: Wire `opsclaw run serve-webhooks`

**Files:**
- Modify: `crates/opsclaw/Cargo.toml`
- Modify: `crates/opsclaw/src/main.rs`
- Modify: `crates/opsclaw/src/webhook_runtime.rs`

**Step 1: Add command + tiny_http server loop**
- add `serve-webhooks` run subcommand with bind/max/tokens/identity args.
- map endpoint routes to existing platform live handlers.
- return JSON response payloads and HTTP status codes.

**Step 2: Verify integration**

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
- Create: `docs/plans/evidence/2026-02-18-phase-5-webhook-ingress-runtime-slice.md`

**Step 1: Document webhook ingress command and endpoint usage.**
**Step 2: Record RED/GREEN evidence and local webhook smoke run.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw webhook_runtime::tests::resolves_supported_paths -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
