# Phase 5 Build-in-Public Content Pipeline (05-05) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Automate per-slice build-in-public draft generation and add an agent-readable docs index.

**Architecture:** Add a deterministic pipeline script under `scripts/content/` plus structured docs metadata in `docs/`.

**Tech Stack:** Bash + Markdown + YAML/JSON artifacts

**Requirement IDs:** PUB-04, PUB-07

---

### Task 1: RED checks before implementation

Run (RED):

```bash
test -x scripts/content/phase-delivery-pipeline.sh
```

Expected: failure because the pipeline script does not exist yet.

### Task 2: Implement pipeline script

**Files:**
- Create: `scripts/content/phase-delivery-pipeline.sh`

### Task 3: Add agent-readable docs index + docs links

**Files:**
- Create: `docs/agent-index.yaml`
- Modify: `docs/README.md`
- Modify: `docs/blog/README.md`

### Task 4: Add usage docs and changelog

**Files:**
- Create: `docs/user-guide/build-in-public-pipeline.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-content-pipeline-slice.md`

### Task 5: Verification

Run:
- `test -x scripts/content/phase-delivery-pipeline.sh`
- `scripts/content/phase-delivery-pipeline.sh --phase 5 --slice 05-05 --date 2026-02-18 --requirements PUB-04,PUB-07 --pr-url https://github.com/opsclawhq/opsclaw/pull/36 --summary "Phase 5 content pipeline baseline" --root "$(mktemp -d)"`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
