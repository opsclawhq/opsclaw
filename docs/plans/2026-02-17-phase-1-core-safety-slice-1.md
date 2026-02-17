# Phase 1 Core Safety Slice 1 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Implement foundational Phase 1 runtime and contract primitives needed for safe agent execution loops.  
**Architecture:** Define reusable core interfaces in `oax-core` and implement runtime primitives in `oax-runtime` for message-tag routing, conversation budgets, and state-machine transitions. Keep the implementation minimal and test-first.  
**Tech Stack:** Rust (`async-trait`, `serde`, `typeshare`)  
**Requirement IDs:** SAFE-04, SAFE-06, SAFE-07, COORD-01, COORD-02, COORD-04, COORD-05

---

## Implemented Tasks

1. Added `oax-core` modules and contracts:
- `agent`
- `model`
- `tool`
- `memory`

2. Added `oax-runtime` modules:
- `router` with `[@agent: payload]` parsing
- `budget` with default 50-message hard limit behavior
- `state` with explicit transition guard logic

3. Added test coverage:
- Core contract object construction tests
- Runtime router parsing tests
- Runtime budget enforcement tests
- Runtime state transition tests

4. Regenerated TypeScript interfaces from Rust via `typeshare`.

5. Added `oax-security` primitives:
- `injector` for `${SECRET_NAME}` boundary injection with missing-secret detection
- `leak` detector based on Aho-Corasick pattern scanning

## Verification

- `cargo test -p oax-runtime`
- `cargo test -p oax-core`
- `cargo test --workspace`
- `bash scripts/generate-types.sh`
- `cargo test -p oax-security`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
