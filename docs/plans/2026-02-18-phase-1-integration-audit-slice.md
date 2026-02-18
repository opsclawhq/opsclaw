# Phase 1 Integration Audit (Gate Readiness) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Produce a single integrated verification slice for Phase 1 safety/runtime behavior and contributor-facing audit documentation.

**Architecture:** Consolidate previously delivered slice evidence through targeted reruns of safety/runtime tests and document gate-readiness outcomes.

**Tech Stack:** Rust tests, Markdown docs

**Requirement IDs:** SAFE-01, SAFE-02, SAFE-03, SAFE-04, SAFE-05, SAFE-06, SAFE-07, SAFE-08, SAFE-09, SAFE-10, COORD-01, COORD-02, COORD-03, COORD-04, COORD-05, INFRA-04, INFRA-05, INFRA-06, INFRA-07

---

### Task 1: RED check before implementation

Run (RED):

```bash
test -f docs/developer-guide/phase-1-safety-audit.md
```

Expected: failure because integrated audit doc does not exist.

### Task 2: Run targeted Phase 1 integration verification

Run:

- `cargo test -p oax-runtime simulation::tests::ping_pong_conversation_reaches_zero_pending -- --exact`
- `cargo test -p oax-runtime simulation::tests::simulation_stops_when_budget_exhausted -- --exact`
- `cargo test -p oax-runtime executor::tests::pending_counter_tracks_enqueued_and_processed_work -- --exact`
- `cargo test -p oax-runtime tool_boundary::tests::injects_credentials_at_tool_boundary -- --exact`
- `cargo test -p oax-runtime tool_boundary::tests::leak_output_is_blocked_before_llm_context -- --exact`
- `cargo test -p oax-runtime events::tests::appends_and_reads_events_in_order -- --exact`
- `cargo test -p oax-runtime events::tests::returns_empty_when_journal_file_missing -- --exact`
- `cargo test -p oax-security injector::tests::injects_secret_placeholders -- --exact`

### Task 3: Add audit docs + changelog

**Files:**
- Create: `docs/developer-guide/phase-1-safety-audit.md`
- Modify: `docs/developer-guide/README.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-1-integration-audit-slice.md`

### Task 4: Full verification

Run:
- `test -f docs/developer-guide/phase-1-safety-audit.md`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
