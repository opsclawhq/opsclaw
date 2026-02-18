# Phase 3 Slack Design Decisions

This post captures the design choices behind OpsClaw's Phase 3 Slack work.

## Decision 1: Contract-First, Transport-Later

Choice:

- build pure Rust contract modules (`slack_adapter`, `slack_approval`, `slack_collaboration`) before full `@slack/bolt` runtime wiring.

Why:

- deterministic testability
- smaller PR slices
- fewer integration-side regressions

Tradeoff:

- feature completeness arrives in layers instead of one end-to-end merge.

## Decision 2: HITL Is a Hard Policy Boundary

Choice:

- represent approvals as explicit payload contracts and typed decisions.

Why:

- policy is inspectable and testable
- no hidden mutating path bypassing approval flow

Tradeoff:

- additional payload parsing/validation surface to maintain.

## Decision 3: Visible Multi-Agent Planning Before Autonomous Execution

Choice:

- plan visible discussion turns and specialist assignment explicitly.

Why:

- supports the "JARVIS moment" requirement
- gives operators context before action

Tradeoff:

- extra coordination payloads can increase message volume.

## Decision 4: Overflow Safety by Snippet Fallback

Choice:

- convert over-limit responses into snippet payload + preview.

Why:

- avoid truncation-based data loss
- preserve complete response body

Tradeoff:

- requires file/snippet delivery wiring in full runtime integration layer.

## Rejected Alternative

Rejected:

- single monolithic Slack integration PR with transport, approvals, collaboration, and docs all at once.

Reason:

- weak traceability
- high regression risk
- difficult rollback boundaries
