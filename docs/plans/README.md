# OpsClaw Plan Documents

This directory stores implementation plans that are ready to execute.

## Naming

Use this filename format for every plan:

`YYYY-MM-DD-phase-<n>-<topic>.md`

Example:

`2026-02-17-phase-0-workspace-foundations.md`

## Required Process

Each plan must follow this Superpowers chain:

1. `brainstorming`
2. `writing-plans`
3. `using-git-worktrees`
4. `subagent-driven-development`
5. `requesting-code-review`
6. `verification-before-completion`
7. `finishing-a-development-branch`

## Required Traceability Fields

Every plan must record:

- Phase number
- Requirement IDs from `.planning/REQUIREMENTS.md`
- Branch name (`codex/phase-<n>-<topic>`)
- PR link
- TDD RED/GREEN evidence link
- Docs artifacts
- Social artifacts
- Gate status and KPI snapshot

Use `.planning/contracts/planning-metadata.schema.json` as the contract.

## Definition of Done Contract

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`

No plan is complete until this contract is satisfied.
