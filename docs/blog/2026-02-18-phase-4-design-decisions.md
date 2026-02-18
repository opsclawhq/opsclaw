# Phase 4 Design Decisions: Why We Built Mission Control in Slices

## Decision 1: API contracts before handlers

We wrote and verified OpenAPI + shared payload types before backend handlers. This forced explicit interface boundaries and prevented frontend guesswork.

## Decision 2: Deterministic reducers before live transport

Kanban and approval behavior was implemented as deterministic reducer logic with local tests. This reduced debugging scope and made event behavior auditable before any socket complexity.

## Decision 3: Economics and transcript visibility before optimization

We added spend, ROI, and transcript visibility early. Operators can now evaluate both effectiveness and cost, which is required before scale or automation claims are credible.

## What We Deferred

- live stream transport wiring
- persistence and historical query optimization
- phase-level docs/playbook automation (Phase 5)

The result is a controlled progression: clear contracts, tested behavior, then operator-facing narrative and runbooks.
