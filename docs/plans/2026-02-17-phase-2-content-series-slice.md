# Phase 2 Content Series and Docs Track Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Add a mandatory OpsClaw vs OpenClaw series workflow with per-phase social drafts and a durable docs/blog structure for users and contributors.  
**Architecture:** Keep volatile marketing drafts in `.content/` (gitignored), while committing stable narrative and contribution structure to `docs/`. Attach the new content outputs to phase-gate tracking and evidence artifacts.  
**Tech Stack:** Markdown docs, git workflow, GitHub PR traceability  
**Requirement IDs:** PUB-02, PUB-03, PUB-05, PUB-06, PUB-07

---

## Task 1: Add tracked docs structure for users, contributors, and engineering blog

**Files:**
- Modify: `docs/README.md`
- Create: `docs/user-guide/README.md`
- Create: `docs/developer-guide/README.md`
- Create: `docs/blog/README.md`
- Create: `docs/blog/2026-02-17-phase-2-content-system.md`

Steps:
1. Write docs index links that explicitly separate user and developer paths.
2. Add contributor-facing architecture/testing/process expectations.
3. Add engineering blog scaffolding and first post for the new content workflow.

## Task 2: Create OpsClaw vs OpenClaw phase-by-phase series artifacts

**Files:**
- Create: `.content/series/opsclaw-vs-openclaw/README.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-template.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-0-comparison.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-1-comparison.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-2-comparison.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-3-comparison.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-4-comparison.md`
- Create: `.content/series/opsclaw-vs-openclaw/phase-5-comparison.md`

Steps:
1. Define strict structure for measurable comparison by phase.
2. Fill phases 0-2 with current shipped deltas.
3. Leave phases 3-5 as forward templates with update checklist.

## Task 3: Add dedicated LinkedIn + X comparison drafts per phase

**Files:**
- Create: `.content/phase-0/2026-02-17-phase-0-opsclaw-vs-openclaw-linkedin-draft.md`
- Create: `.content/phase-0/2026-02-17-phase-0-opsclaw-vs-openclaw-x-thread.md`
- Create: `.content/phase-1/2026-02-17-phase-1-opsclaw-vs-openclaw-linkedin-draft.md`
- Create: `.content/phase-1/2026-02-17-phase-1-opsclaw-vs-openclaw-x-thread.md`
- Create: `.content/phase-2/2026-02-17-phase-2-opsclaw-vs-openclaw-linkedin-draft.md`
- Create: `.content/phase-2/2026-02-17-phase-2-opsclaw-vs-openclaw-x-thread.md`
- Create: `.content/phase-3/2026-02-17-phase-3-opsclaw-vs-openclaw-linkedin-draft.md`
- Create: `.content/phase-3/2026-02-17-phase-3-opsclaw-vs-openclaw-x-thread.md`
- Create: `.content/phase-4/2026-02-17-phase-4-opsclaw-vs-openclaw-linkedin-draft.md`
- Create: `.content/phase-4/2026-02-17-phase-4-opsclaw-vs-openclaw-x-thread.md`
- Create: `.content/phase-5/2026-02-17-phase-5-opsclaw-vs-openclaw-linkedin-draft.md`
- Create: `.content/phase-5/2026-02-17-phase-5-opsclaw-vs-openclaw-x-thread.md`

Steps:
1. Use one narrative format across all phases.
2. Populate phases 0-2 with shipped evidence.
3. Keep phases 3-5 prefilled with placeholders tied to roadmap outcomes.

## Task 4: Update changelog and evidence

**Files:**
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-17-phase-2-content-series-slice.md`

Steps:
1. Add phase-2 slice changelog entry for content/docs track.
2. Capture verification output and artifact existence checks.

## Task 5: Verification

Run:
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `test -f docs/user-guide/README.md`
- `test -f docs/developer-guide/README.md`
- `test -f docs/blog/2026-02-17-phase-2-content-system.md`
- `test -f .content/series/opsclaw-vs-openclaw/phase-5-comparison.md`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
