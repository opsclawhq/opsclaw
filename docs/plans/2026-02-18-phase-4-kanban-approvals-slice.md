# Phase 4 Kanban + Approval Prompts (04-03) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add real-time-capable kanban and approval prompt surfaces to Mission Control dashboard foundation.

**Architecture:** Introduce deterministic dashboard event reducer helpers and wire UI panels that render task board + pending approvals from shared dashboard state.

**Tech Stack:** React 19, Node `node:test` runner, existing dashboard package scaffold

**Requirement IDs:** DASH-02, DASH-03, DASH-08

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
npm --prefix packages/dashboard test
```

Expected: failure due missing kanban/approval reducer module used by new tests.

### Task 2: Add reducer-level tests and event fixtures

**Files:**
- Create: `packages/dashboard/tests/kanban-approvals.test.mjs`
- Create: `packages/dashboard/src/lib/mock-stream-events.mjs`

### Task 3: Implement dashboard event reducer and state helpers

**Files:**
- Create: `packages/dashboard/src/lib/dashboard-state.mjs`
- Modify: `packages/dashboard/src/lib/mock-data.mjs`
- Modify: `packages/dashboard/src/lib/view-models.mjs`

### Task 4: Add kanban + approval UI surfaces

**Files:**
- Create: `packages/dashboard/src/components/KanbanBoardPanel.tsx`
- Create: `packages/dashboard/src/components/ApprovalQueuePanel.tsx`
- Modify: `packages/dashboard/src/App.tsx`
- Modify: `packages/dashboard/src/styles.css`

### Task 5: Docs + changelog + evidence updates

**Files:**
- Modify: `docs/architecture.md`
- Modify: `docs/developer-guide/README.md`
- Create: `docs/developer-guide/dashboard-kanban-approvals.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-4-kanban-approvals-slice.md`

### Task 6: Verification

Run:
- `npm --prefix packages/dashboard test`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
