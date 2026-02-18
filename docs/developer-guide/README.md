# Developer Guide

This track is for contributors building OpsClaw internals.

Docs site route: `/docs/developer-guide`

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

Use these release hardening helpers before phase gate closure:

- `scripts/docs/validate-release-docs.sh`
- `scripts/content/generate-engineering-blog.sh --phase <n> --date <yyyy-mm-dd>`

## Phase 0 Workspace Package Commands

- `bun install`
- `bun run --filter @opsclaw/channels test`
- `bun run --filter @opsclaw/channels typecheck`
- `bun run --filter @opsclaw/sdk typecheck`
- `bun run --filter @opsclaw/dashboard typecheck`
- `bash scripts/release/package-opsclaw.sh`
- `bash scripts/install/verify-local-install.sh`

## Phase 0 Guides

- [Release Packaging](release-packaging.md)

## Phase 3 Guides

- [Slack Integration Guide](slack-integration.md)

## Phase 4 Guides

- [Mission Control Frontend Foundation](dashboard-frontend-foundation.md)
- [Mission Control Kanban and Approval Stream](dashboard-kanban-approvals.md)
- [Mission Control Economics and Conversation Viewer](dashboard-economics-conversations.md)
- [Mission Control Architecture](mission-control-architecture.md)
- [Docs Release Hardening](docs-release-hardening.md)
