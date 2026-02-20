# Phase 5 KPI Measurement Automation (05-21) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5
**Goal:** Automate measured KPI snapshot computation and application for planning contracts + GitHub Project.
**Architecture:** Bash scripts (`gh` + `jq`) with strict validation integration.
**Tech Stack:** Bash, jq, gh
**Requirement IDs:** ROADMAP-GATE-KPI, PUB-07

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-kpi-measurement-automation`
- Verification is mandatory before any completion claim

### Task 1: RED precheck

Run (expect fail):
```bash
test -f scripts/planning/compute-kpi-snapshot.sh
```

### Task 2: Implement KPI compute/apply scripts

**Files:**
- Create: `scripts/planning/compute-kpi-snapshot.sh`
- Create: `scripts/planning/apply-kpi-snapshot.sh`

### Task 3: Docs/changelog/evidence updates

**Files:**
- Modify: `docs/developer-guide/kpi-telemetry-hardening.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-19-phase-5-kpi-measurement-automation-slice.md`

### Task 4: Apply measured snapshot

Run:
```bash
bash scripts/planning/compute-kpi-snapshot.sh --owner opsclawhq --repo opsclaw --project 1 --window-days 30 --output .planning/contracts/kpi-snapshot.latest.json
bash scripts/planning/apply-kpi-snapshot.sh --snapshot .planning/contracts/kpi-snapshot.latest.json --contracts-dir .planning/contracts --owner opsclawhq --project 1
```

### Task 5: Strict verification gate

Run:
```bash
bash scripts/planning/validate-planning-metadata.sh
bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1
bash scripts/docs/validate-release-docs.sh
cargo test --workspace
cargo clippy --workspace --all-targets
```

Expected: all pass with no placeholder override.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
