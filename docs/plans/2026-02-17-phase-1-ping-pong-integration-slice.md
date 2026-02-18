# Phase 1 Ping-Pong Integration Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add integration-level simulation evidence for tagged inter-agent ping-pong and pending-counter drain behavior.  
**Architecture:** Implement a lightweight simulation harness in `oax-runtime` that composes existing `router`, `executor`, and `budget` primitives to replay scripted tagged turns and report processed count, remaining pending work, and budget exhaustion state.  
**Tech Stack:** Rust (`oax-runtime`)  
**Requirement IDs:** COORD-01, COORD-03, COORD-05

---

## Implemented Tasks

1. Added `oax-runtime::simulation` module and exported it via `oax-runtime/src/lib.rs`.
2. Implemented:
- `SimulationResult`
- `simulate_tagged_conversation(scripted_turns, budget_limit)`
3. Added tests for:
- two-agent ping-pong path draining to `pending_remaining == 0`
- conversation stopping when budget limit is exhausted

## Verification

- `cargo test -p oax-runtime simulation::tests::ping_pong_conversation_reaches_zero_pending -- --exact`
- `cargo test -p oax-runtime`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
