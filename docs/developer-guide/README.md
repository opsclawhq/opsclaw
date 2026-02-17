# Developer Guide

This track is for contributors building OpsClaw internals.

## Scope

- Workspace architecture and crate boundaries
- Runtime contracts and safety invariants
- Testing discipline (RED -> GREEN evidence)
- Contribution flow (plan docs, review gates, verification)

## Required Updates Per Phase

1. Add one architecture deep-dive tied to merged code.
2. Add one test/verification note describing what changed.
3. Add traceability links: requirement -> plan -> PR.

## Content Maintenance Command

Use this helper at phase close to scaffold comparison + social artifacts:

`scripts/content/update-phase-content.sh <phase-number> [yyyy-mm-dd]`
