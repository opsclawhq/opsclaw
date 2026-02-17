# [Feature Name] Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** [0|1|2|3|4|5]  
**Goal:** [One sentence describing what this builds]  
**Architecture:** [2-3 sentences about approach]  
**Tech Stack:** [Key technologies/libraries]  
**Requirement IDs:** [IDs from `.planning/REQUIREMENTS.md`]

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-<n>-<topic>`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

## Task N: [Component Name]

**Files:**
- Create: `exact/path`
- Modify: `exact/path`
- Test: `exact/path`

**Step 1: Write the failing test (RED)**

```text
[Paste exact test code/command]
```

**Step 2: Run test to verify RED**

Run: `[exact command]`  
Expected: `[explicit failing output]`

**Step 3: Write minimal implementation (GREEN)**

```text
[Paste exact implementation change]
```

**Step 4: Run test to verify GREEN**

Run: `[exact command]`  
Expected: `[explicit passing output]`

**Step 5: Run broader verification**

Run: `[test/build/lint commands]`  
Expected: `[explicit pass criteria]`

**Step 6: Commit**

```bash
git add [files]
git commit -m "[message]"
```

Repeat task structure until plan scope is complete.

## Test Cases and Scenarios

1. Traceability: each merged PR maps to at least one requirement ID.
2. TDD evidence: each completed task includes RED and GREEN verification evidence.
3. Gate enforcement: no phase transition unless KPI snapshot exists and gate status is `passed`.
4. Docs/social completion: each completed plan includes docs and social artifact links.
5. Process audit: sample three tasks in this phase and verify spec compliance and verification logs.
6. Consistency: if `PROJECT.md` conflicts with roadmap scope, update roadmap or log exception before execution.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`

## Metadata Contract

Create and maintain a metadata JSON record using `.planning/contracts/planning-metadata.schema.json`.

Minimum fields:
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

- User docs update: `[path]`
- Technical changelog update: `[path]`
- LinkedIn draft: `.content/phase-<n>/linkedin-draft.md`
- X thread draft: `.content/phase-<n>/x-thread-draft.md`
