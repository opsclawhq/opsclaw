# Phase 5 Live Stdio Orchestrator Bridge (05-12) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add `opsclaw run live-stdio` for continuous Slack/Discord/Telegram live event dispatch from one runtime process.  
**Architecture:** Introduce a generic NDJSON live loop primitive, then wire runtime dispatcher with lazy platform API clients and stable JSON output.  
**Tech Stack:** Rust (`serde`, `serde_json`, `ureq`, `clap`)  
**Requirement IDs:** CHAT-08, BOT-07, CHAT-01, CHAT-04, CHAT-05

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-live-stdio-orchestrator`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED tests for live stdio loop primitive

**Files:**
- Create: `crates/opsclaw/src/live_runtime.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - NDJSON loop dispatches events and emits decision lines.
  - empty lines are skipped.
  - invalid inbound JSON fails with clear error.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw live_runtime::tests::stdio_loop_emits_decisions -- --exact
```

Expected: fail before implementation.

### Task 2: Implement live stdio loop primitive

**Files:**
- Create: `crates/opsclaw/src/live_runtime.rs`

**Step 1: Add minimal implementation**
- add `LiveRuntimeLoopOutcome` and `run_live_stdio_loop(...)`.
- parse `RuntimeInboundEvent` lines, invoke callback dispatcher, emit JSON lines.

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw live_runtime::tests::stdio_loop_emits_decisions -- --exact
```

Expected: pass.

### Task 3: Wire `opsclaw run live-stdio`

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 1: Add command + runtime dispatcher**
- add `run live-stdio` subcommand and token/env args.
- lazily initialize Slack/Discord/Telegram HTTP API clients.
- dispatch each inbound event through existing live handlers and print JSON decisions.

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
- Create: `docs/plans/evidence/2026-02-18-phase-5-live-stdio-runtime-slice.md`

**Step 1: Document continuous runtime bridge usage.**
**Step 2: Record RED/GREEN evidence and smoke commands.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw live_runtime::tests::stdio_loop_emits_decisions -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
