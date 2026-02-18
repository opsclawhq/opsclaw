# Phase 5 Setup Wizard Scaffold (05-01) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Introduce a tested `opsclaw init` setup wizard contract with template planning and sub-60-second gate output.

**Architecture:** Add `setup_wizard` domain module and wire a new CLI entrypoint returning deterministic setup-plan JSON.

**Tech Stack:** Rust (`clap`, `serde_json`) in existing `opsclaw` crate

**Requirement IDs:** BOT-04, BOT-05, BOT-07

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
cargo test -p opsclaw setup_wizard::tests::build_sre_template_plan_is_sub_60_seconds -- --exact
```

Expected: failure because setup wizard module/tests do not exist yet.

### Task 2: Add setup wizard module and tests

**Files:**
- Create: `crates/opsclaw/src/setup_wizard.rs`

### Task 3: Wire new `opsclaw init` command

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

### Task 4: Docs and changelog updates

**Files:**
- Create: `docs/user-guide/setup-wizard-preview.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-setup-wizard-slice.md`

### Task 5: Verification

Run:
- `cargo test -p opsclaw setup_wizard::tests::build_sre_template_plan_is_sub_60_seconds -- --exact`
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
