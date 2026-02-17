# Phase 0 Workspace Foundations Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 0  
**Goal:** Establish the non-retrofittable workspace foundations for OpsClaw and prove they are verifiable.  
**Architecture:** Build a minimal Rust + TypeScript workspace scaffold with typed boundary generation, local dev stack, and docs skeleton. Enforce quality through RED/GREEN test tasks and verification gates before any Phase 1 runtime work starts.  
**Tech Stack:** Rust workspace, Bun/TypeScript, Docker Compose, Docusaurus, GitHub Actions.  
**Requirement IDs:** INFRA-01, INFRA-02, INFRA-03, PUB-01

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-0-workspace-foundations`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

## Task 1: Workspace and CI Baseline

**Files:**
- Create: `Cargo.toml`
- Create: `.github/workflows/ci.yml`
- Create: `crates/oax-core/src/lib.rs`
- Create: `crates/oax-runtime/src/lib.rs`
- Create: `crates/oax-llm/src/lib.rs`
- Create: `crates/oax-security/src/lib.rs`
- Create: `crates/opsclaw/src/main.rs`
- Test: `.github/workflows/ci.yml`

**Step 1: Write the failing test (RED)**

Create CI checks expecting workspace compile and failing before scaffolding exists.

**Step 2: Run test to verify RED**

Run: `cargo check --workspace`  
Expected: FAIL (workspace/crates missing)

**Step 3: Write minimal implementation (GREEN)**

Create minimal workspace manifest and crate stubs sufficient for `cargo check --workspace`.

**Step 4: Run test to verify GREEN**

Run: `cargo check --workspace`  
Expected: PASS

**Step 5: Run broader verification**

Run: `cargo test --workspace`  
Expected: PASS with zero failing tests

**Step 6: Commit**

```bash
git add Cargo.toml crates .github/workflows/ci.yml
git commit -m "chore(phase-0): scaffold rust workspace and ci baseline"
```

## Task 2: Type Boundary Generation Pipeline

**Files:**
- Create: `crates/oax-core/src/types.rs`
- Create: `packages/sdk/src/generated/types.ts`
- Create: `scripts/generate-types.sh`
- Modify: `.github/workflows/ci.yml`
- Test: `scripts/generate-types.sh`

**Step 1: Write the failing test (RED)**

Add CI step that fails when generated TS types drift from Rust definitions.

**Step 2: Run test to verify RED**

Run: `bash scripts/generate-types.sh && git diff --exit-code packages/sdk/src/generated/types.ts`  
Expected: FAIL when generated output is stale

**Step 3: Write minimal implementation (GREEN)**

Implement types generation script and initial generated file.

**Step 4: Run test to verify GREEN**

Run: `bash scripts/generate-types.sh && git diff --exit-code packages/sdk/src/generated/types.ts`  
Expected: PASS (no drift)

**Step 5: Run broader verification**

Run: `cargo check --workspace` and `bun --version`  
Expected: PASS and toolchain available

**Step 6: Commit**

```bash
git add crates/oax-core/src/types.rs scripts/generate-types.sh packages/sdk/src/generated/types.ts .github/workflows/ci.yml
git commit -m "chore(phase-0): add rust-to-typescript type generation gate"
```

## Task 3: Local Dev Stack and Single-Binary Packaging Baseline

**Files:**
- Create: `docker/docker-compose.yaml`
- Create: `docker/Dockerfile`
- Modify: `crates/opsclaw/src/main.rs`
- Test: `docker/docker-compose.yaml`

**Step 1: Write the failing test (RED)**

Create a validation check expecting health checks for PostgreSQL, Redis, and NATS.

**Step 2: Run test to verify RED**

Run: `docker compose -f docker/docker-compose.yaml config`  
Expected: FAIL before compose file exists

**Step 3: Write minimal implementation (GREEN)**

Create compose stack and Dockerfile with healthcheck definitions.

**Step 4: Run test to verify GREEN**

Run: `docker compose -f docker/docker-compose.yaml config`  
Expected: PASS

**Step 5: Run broader verification**

Run: `docker compose -f docker/docker-compose.yaml up -d && docker compose -f docker/docker-compose.yaml ps`  
Expected: Services healthy

**Step 6: Commit**

```bash
git add docker crates/opsclaw/src/main.rs
git commit -m "chore(phase-0): add local dev stack and packaging baseline"
```

## Task 4: Docs Skeleton (Human + Agent Readable)

**Files:**
- Create: `docs/README.md`
- Create: `docs/getting-started.md`
- Create: `docs/concepts.md`
- Create: `docs/skills.md`
- Create: `docs/architecture.md`
- Create: `docs/api-reference.md`
- Test: `docs/README.md`

**Step 1: Write the failing test (RED)**

Add docs structure check ensuring all required skeleton pages exist.

**Step 2: Run test to verify RED**

Run: `test -f docs/getting-started.md && test -f docs/concepts.md && test -f docs/skills.md && test -f docs/architecture.md && test -f docs/api-reference.md`  
Expected: FAIL until files exist

**Step 3: Write minimal implementation (GREEN)**

Create docs skeleton files with headings and scope notes.

**Step 4: Run test to verify GREEN**

Run: `test -f docs/getting-started.md && test -f docs/concepts.md && test -f docs/skills.md && test -f docs/architecture.md && test -f docs/api-reference.md`  
Expected: PASS

**Step 5: Run broader verification**

Run: `rg -n \"# \" docs`  
Expected: PASS with section headers in all docs

**Step 6: Commit**

```bash
git add docs
git commit -m "docs(phase-0): add docs skeleton for user and agent audiences"
```

## Task 5: Phase 0 Traceability and Content Deliverables

**Files:**
- Modify: `.planning/STATE.md`
- Create: `.planning/contracts/phase-0-workspace-foundations.metadata.json`
- Create: `.content/phase-0/linkedin-draft.md`
- Create: `.content/phase-0/x-thread-draft.md`
- Test: `.planning/STATE.md`

**Step 1: Write the failing test (RED)**

Define expected traceability fields and check missing entries.

**Step 2: Run test to verify RED**

Run: `rg -n \"INFRA-01|INFRA-02|INFRA-03|PUB-01\" .planning/STATE.md`  
Expected: FAIL if rows missing or incomplete

**Step 3: Write minimal implementation (GREEN)**

Add requirement-to-plan mapping entries and metadata contract instance.

**Step 4: Run test to verify GREEN**

Run: `rg -n \"INFRA-01|INFRA-02|INFRA-03|PUB-01\" .planning/STATE.md`  
Expected: PASS with populated rows

**Step 5: Run broader verification**

Run: `test -f .content/phase-0/linkedin-draft.md && test -f .content/phase-0/x-thread-draft.md`  
Expected: PASS

**Step 6: Commit**

```bash
git add .planning/STATE.md .planning/contracts/phase-0-workspace-foundations.metadata.json .content/phase-0
git commit -m "chore(phase-0): add traceability and content deliverables"
```

## Test Cases and Scenarios

1. Traceability test: each merged PR must map to at least one requirement ID.
2. TDD evidence test: each completed task must show RED and GREEN verification evidence.
3. Gate enforcement test: no phase transition unless KPI snapshot is recorded and gate status is `Passed`.
4. Docs/social completion test: each completed plan must include docs artifact and social artifact links.
5. Process audit test: sample three tasks per phase and verify spec compliance plus verification logs.
6. Consistency test: if `PROJECT.md` scope conflicts with roadmap, roadmap must be updated or explicit exception logged before execution.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`

## Metadata Contract

Record and maintain metadata using `.planning/contracts/planning-metadata.schema.json`.

Required fields:
- `phase`
- `plan_id`
- `requirement_ids`
- `branch`
- `pr`
- `tdd_evidence_uri`
- `docs_artifacts`
- `social_artifacts`
- `gate_status`
- `kpi_snapshot`

## Artifacts

- User docs update: `docs/getting-started.md` (or equivalent phase output)
- Technical changelog update: `CHANGELOG.md`
- LinkedIn draft: `.content/phase-0/linkedin-draft.md`
- X thread draft: `.content/phase-0/x-thread-draft.md`
