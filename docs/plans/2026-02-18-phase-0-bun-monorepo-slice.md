# Phase 0 Bun Monorepo Structure (00-02) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Deliver the Bun monorepo workspace and package structure for `@opsclaw/sdk`, `@opsclaw/channels`, and `@opsclaw/dashboard`.

**Architecture:** Add root workspace config, shared TS base config, and package-level manifests/tsconfigs including a new channels package.

**Tech Stack:** Bun workspaces, TypeScript

**Requirement IDs:** INFRA-02

---

### Task 1: RED check before implementation

Run (RED):

```bash
test -f packages/channels/package.json
```

Expected: failure because channels package does not exist yet.

### Task 2: Add root Bun workspace + TS base config

**Files:**
- Create: `package.json`
- Create: `bunfig.toml`
- Create: `tsconfig.base.json`

### Task 3: Normalize package manifests/configs

**Files:**
- Create: `packages/sdk/package.json`
- Create: `packages/sdk/tsconfig.json`
- Create: `packages/channels/package.json`
- Create: `packages/channels/tsconfig.json`
- Create: `packages/channels/src/index.ts`
- Create: `packages/channels/tests/contracts.test.ts`
- Create: `packages/channels/README.md`
- Create: `packages/dashboard/tsconfig.json`
- Modify: `packages/dashboard/package.json`

### Task 4: Docs + changelog + evidence

**Files:**
- Modify: `docs/developer-guide/README.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-0-bun-monorepo-slice.md`

### Task 5: Verification

Run:
- `test -f packages/channels/package.json`
- `bun install`
- `bun run --filter @opsclaw/channels test`
- `bun run --filter @opsclaw/channels typecheck`
- `bun run --filter @opsclaw/sdk typecheck`
- `bun run --filter @opsclaw/dashboard typecheck`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
