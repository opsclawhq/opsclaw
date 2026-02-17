# GitHub Project Setup: OpsClaw Delivery

Use this runbook to configure the primary tracking board for roadmap execution.

## 1. Authenticate with required scopes

```bash
gh auth refresh -h github.com -s read:project,project,read:org,read:discussion
```

## 2. Create project board

Use org owner (preferred) or user owner.

```bash
# Org-level (preferred if you have org permissions)
gh project create --owner opsclawhq --title "OpsClaw Delivery"

# User-level fallback
gh project create --owner @me --title "OpsClaw Delivery"
```

If the repository has no default branch yet, create it first:

```bash
git push origin codex/phase-0-workspace-foundations:main
```

## 3. Add required project fields

Create these fields on the board:

- `Phase` (number or single-select)
- `Requirement IDs` (text)
- `Plan Doc Path` (text)
- `Branch` (text)
- `PR` (text or URL)
- `TDD Evidence` (text or URL)
- `Docs Status` (single-select: `not_started`, `in_progress`, `done`)
- `Social Status` (single-select: `not_started`, `in_progress`, `done`)
- `Gate Status` (single-select: `not_ready`, `ready`, `passed`, `blocked`)
- `KPI Snapshot` (text or JSON snippet)

## 4. Recommended item flow

1. Add one item per implementation unit (plan file).
2. Link requirement IDs from `.planning/REQUIREMENTS.md`.
3. Populate branch and PR when execution starts.
4. Populate TDD evidence links per task/batch.
5. Populate docs/social artifact links before marking done.
6. Phase gate is only `passed` when all evidence fields are complete.

## 5. Contract alignment

Ensure item metadata follows:

- `.planning/contracts/planning-metadata.schema.json`
- `.planning/contracts/planning-metadata.example.json`

## 6. Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
