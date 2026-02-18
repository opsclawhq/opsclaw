# KPI Telemetry Hardening Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Introduce repeatable KPI quality checks so gate state cannot silently drift from evidence quality.

**Architecture:** Bash scripts validate local planning metadata and audit GitHub Project KPI fields, with strict mode to block placeholder KPI snapshots.

**Tech Stack:** Bash, jq, gh, Markdown docs

**Requirement IDs:** Roadmap gate policy (KPI snapshot + gate quality)

---

### Task 1: RED check before implementation

Run (RED):

```bash
test -f scripts/planning/validate-planning-metadata.sh
```

Expected: failure because script does not exist yet.

### Task 2: Add KPI hardening scripts

**Files:**
- Create: `scripts/planning/validate-planning-metadata.sh`
- Create: `scripts/planning/audit-project-kpi-snapshots.sh`

### Task 3: Add docs + changelog + evidence

**Files:**
- Create: `docs/developer-guide/kpi-telemetry-hardening.md`
- Modify: `docs/developer-guide/README.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-kpi-telemetry-hardening-slice.md`

### Task 4: Verification

Run:
- `test -f scripts/planning/validate-planning-metadata.sh`
- `ALLOW_PLACEHOLDER_KPI=1 bash scripts/planning/validate-planning-metadata.sh`
- `ALLOW_PLACEHOLDER_KPI=1 bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1`
- `bash scripts/docs/validate-release-docs.sh`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
