# Phase 5 Unified Runtime Core (05-08) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add one shared `opsclaw run` runtime core for multi-platform event handling and squad responses.  
**Architecture:** Introduce `squad_responder` and `squad_runtime` modules; route all platform events through `channels_router`; expose `run route-event` and `run stdio` commands.  
**Tech Stack:** Rust (`clap`, `serde`, `serde_json`)  
**Requirement IDs:** CHAT-08, BOT-07, CHAT-05

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-unified-runtime-core`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED test for unified runtime parity

**Files:**
- Create: `crates/opsclaw/src/squad_runtime.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - Slack/Discord/Telegram routed events produce responses via one function.
  - `/squad` command response parity across Telegram and Discord inputs.
  - non-routable events return `None`.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw squad_runtime::tests::parity_for_squad_command_across_platforms -- --exact
```

Expected: fail before implementation.

### Task 2: Implement shared responder and runtime core (GREEN)

**Files:**
- Create: `crates/opsclaw/src/squad_responder.rs`
- Create: `crates/opsclaw/src/squad_runtime.rs`
- Modify: `crates/opsclaw/src/telegram_adapter.rs`

**Step 1: Add minimal implementation**
- Extract command/agent reply builders to `squad_responder`.
- Implement `process_inbound_event` and stdio loop helpers in `squad_runtime`.
- Wire Telegram live runtime to reuse `squad_responder`.

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw squad_runtime::tests::parity_for_squad_command_across_platforms -- --exact
```

Expected: pass.

### Task 3: Wire CLI commands

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 1: Add `opsclaw run` subcommands**
- `run route-event` for single event processing.
- `run stdio` for NDJSON loop processing.

**Step 2: Verify integration**

Run:
```bash
cargo test -p opsclaw
```

Expected: pass.

### Task 4: Docs, changelog, evidence

**Files:**
- Modify: `docs/architecture.md`
- Modify: `docs/user-guide/multi-platform-routing-preview.md`
- Modify: `docs/user-guide/README.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-unified-runtime-core-slice.md`

**Step 1: Update operator-facing usage**
- Add `opsclaw run route-event` and `opsclaw run stdio` workflows.
- Explain relationship to existing Telegram live transport.

**Step 2: Add RED/GREEN command evidence.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw squad_runtime::tests::parity_for_squad_command_across_platforms -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected:
- all commands exit 0.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
