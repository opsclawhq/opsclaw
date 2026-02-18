# Phase 4 API Contracts-First Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Ship Mission Control API contracts (OpenAPI + shared typed models) before any endpoint implementation.

**Architecture:** Define canonical API paths and schemas in OpenAPI, mirror payload models in `oax-core` with `typeshare`, and regenerate SDK types for frontend/backend contract parity.

**Tech Stack:** OpenAPI 3.1 YAML, Rust (`serde`, `typeshare`), TypeScript type generation

**Requirement IDs:** DASH-01, DASH-02, DASH-03, DASH-08

---

### Task 1: RED checks for missing contract artifacts

Run (RED):

```bash
test -f docs/api/mission-control-openapi.yaml
cargo test -p oax-core types::tests::dashboard_stream_event_roundtrip_json -- --exact
```

Expected: OpenAPI file missing and test target fails before contract implementation.

### Task 2: Add OpenAPI contract draft

**Files:**
- Create: `docs/api/mission-control-openapi.yaml`

### Task 3: Add shared dashboard contract models

**Files:**
- Modify: `crates/oax-core/src/types.rs`
- Modify: `packages/sdk/src/generated/types.ts` (via generation)

### Task 4: Regenerate SDK types and docs links

**Files:**
- Modify: `docs/api-reference.md`
- Modify: `CHANGELOG.md`

Run:
- `bash scripts/generate-types.sh`

### Task 5: Verification

Run:
- `cargo test -p oax-core`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `bash scripts/generate-types.sh && git diff --exit-code packages/sdk/src/generated/types.ts`
