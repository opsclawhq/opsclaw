# Phase 5 Docs Release Hardening + Blog Automation (05-06) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Close Phase 5 with release-grade docs validation and recurring engineering blog scaffolding workflow.

**Architecture:** Add docs/content automation scripts and contributor workflow docs.

**Tech Stack:** Bash + Markdown docs workflow

**Requirement IDs:** PUB-05, PUB-06, PUB-07

---

### Task 1: RED checks before implementation

Run (RED):

```bash
test -x scripts/docs/validate-release-docs.sh
```

Expected: failure because release-doc validation script does not exist yet.

### Task 2: Implement docs and blog automation scripts

**Files:**
- Create: `scripts/docs/validate-release-docs.sh`
- Create: `scripts/content/generate-engineering-blog.sh`

### Task 3: Add docs hardening + editorial workflow guides

**Files:**
- Create: `docs/developer-guide/docs-release-hardening.md`
- Create: `docs/blog/editorial-workflow.md`
- Modify: `docs/developer-guide/README.md`
- Modify: `docs/blog/README.md`
- Modify: `docs/README.md`

### Task 4: Changelog + evidence

**Files:**
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-docs-hardening-slice.md`

### Task 5: Verification

Run:
- `test -x scripts/docs/validate-release-docs.sh`
- `scripts/docs/validate-release-docs.sh`
- `scripts/content/generate-engineering-blog.sh --phase 5 --date 2026-02-18 --root "$(mktemp -d)"`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
