# Phase 4 Economics + Conversation Viewer (04-04) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add Mission Control economics/ROI and full conversation transcript viewing surfaces.

**Architecture:** Introduce deterministic economics and conversation view-model helpers, then wire React dashboard panels against mock contract-shaped data.

**Tech Stack:** React 19, Node `node:test` runner, existing dashboard package scaffold

**Requirement IDs:** DASH-04, DASH-05, DASH-07

---

### Task 1: RED test execution before implementation

Run (RED):

```bash
npm --prefix packages/dashboard test
```

Expected: failure due missing economics/conversation view-model module used by new tests.

### Task 2: Add economics/conversation tests first

**Files:**
- Create: `packages/dashboard/tests/economics-conversation.test.mjs`

### Task 3: Implement view-model helpers and mock data

**Files:**
- Create: `packages/dashboard/src/lib/economics-conversation.mjs`
- Modify: `packages/dashboard/src/lib/mock-data.mjs`

### Task 4: Add economics + conversation UI surfaces

**Files:**
- Create: `packages/dashboard/src/components/EconomicsPanel.tsx`
- Create: `packages/dashboard/src/components/ConversationViewerPanel.tsx`
- Modify: `packages/dashboard/src/App.tsx`
- Modify: `packages/dashboard/src/styles.css`

### Task 5: Docs + changelog + evidence updates

**Files:**
- Modify: `docs/architecture.md`
- Modify: `docs/developer-guide/README.md`
- Create: `docs/developer-guide/dashboard-economics-conversations.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-4-economics-conversation-slice.md`

### Task 6: Verification

Run:
- `npm --prefix packages/dashboard test`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
