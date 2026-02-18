# Phase 0 Docusaurus Docs Site Foundation (00-04/00-05) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Ship a Docusaurus docs site that renders OpsClaw core docs plus explicit user and developer tracks.

**Architecture:** Add a Docusaurus package that reads the existing repository docs tree and exposes structured navigation for humans and agents.

**Tech Stack:** Docusaurus 3, TypeScript config, Markdown docs

**Requirement IDs:** PUB-01, PUB-04

---

### Task 1: RED check before implementation

Run (RED):

```bash
test -f packages/docs-site/docusaurus.config.ts
```

Expected: failure because the Docusaurus site package does not exist yet.

### Task 2: Scaffold Docusaurus site package

**Files:**
- Create: `packages/docs-site/package.json`
- Create: `packages/docs-site/tsconfig.json`
- Create: `packages/docs-site/docusaurus.config.ts`
- Create: `packages/docs-site/sidebars.ts`
- Create: `packages/docs-site/src/css/custom.css`
- Create: `packages/docs-site/src/pages/index.tsx`
- Create: `packages/docs-site/.gitignore`

### Task 3: Wire docs information architecture

**Files:**
- Create: `docs/docs-site-index.md`
- Modify: `docs/README.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/developer-guide/README.md`

### Task 4: Verification + evidence + changelog

**Files:**
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-0-docusaurus-site-slice.md`

Run:
- `test -f packages/docs-site/docusaurus.config.ts`
- `npm --prefix packages/docs-site install`
- `npm --prefix packages/docs-site run build`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
