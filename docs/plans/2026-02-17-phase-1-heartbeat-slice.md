# Phase 1 Heartbeat Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add heartbeat runtime primitives so agents can report alive status on schedule.  
**Architecture:** Introduce a `HeartbeatRegistry` in `oax-runtime` that tracks registered agents, records heartbeat timestamps, computes liveness with a configurable missed-beat tolerance, and reports agents currently due for their next beat.  
**Tech Stack:** Rust (`std::collections`)  
**Requirement IDs:** INFRA-04

---

## Implemented Tasks

1. Added `oax-runtime::heartbeat` module and exported it via `oax-runtime/src/lib.rs`.
2. Implemented `HeartbeatRegistry` with:
- `new(interval_ms, allowed_misses)`
- `register_agent(agent_id)`
- `record_heartbeat(agent_id, ts_ms) -> Result<(), String>`
- `is_alive(agent_id, now_ms) -> bool`
- `due_agents(now_ms) -> Vec<String>`
3. Added tests covering:
- recording heartbeat for unknown agent returns error
- liveness within and outside tolerated missed-beat window
- due-agent reporting for stale and never-seen agents

## Verification

- `cargo test -p oax-runtime heartbeat::tests::recording_unregistered_agent_returns_error -- --exact`
- `cargo test -p oax-runtime`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
