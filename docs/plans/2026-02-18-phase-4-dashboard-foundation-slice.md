# Phase 4 Dashboard Foundation (Org + Profile + Activity) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build dashboard frontend foundation for org hierarchy, agent profile, and activity feed surfaces.

**Architecture:** Add a new `packages/dashboard` package with React/TanStack scaffold files plus deterministic view-model helpers and runnable Node tests for core data shaping behavior.

**Tech Stack:** React 19, TanStack (scaffold imports), Node test runner (`node:test`) for view-model checks

**Requirement IDs:** DASH-01, DASH-03, DASH-06

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
npm --prefix packages/dashboard test
```

Expected: failure because dashboard package/tests do not exist yet.

### Task 2: Create dashboard package scaffold

**Files:**
- Create: `packages/dashboard/package.json`
- Create: `packages/dashboard/README.md`
- Create: `packages/dashboard/src/App.tsx`
- Create: `packages/dashboard/src/components/OrgHierarchyPanel.tsx`
- Create: `packages/dashboard/src/components/AgentProfilePanel.tsx`
- Create: `packages/dashboard/src/components/ActivityFeedPanel.tsx`
- Create: `packages/dashboard/src/styles.css`

### Task 3: Add data/view-model layer and tests

**Files:**
- Create: `packages/dashboard/src/lib/mock-data.mjs`
- Create: `packages/dashboard/src/lib/view-models.mjs`
- Create: `packages/dashboard/tests/view-models.test.mjs`

### Task 4: Docs + changelog + evidence updates

**Files:**
- Modify: `docs/architecture.md`
- Modify: `docs/developer-guide/README.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-4-dashboard-foundation-slice.md`

### Task 5: Verification

Run:
- `npm --prefix packages/dashboard test`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
