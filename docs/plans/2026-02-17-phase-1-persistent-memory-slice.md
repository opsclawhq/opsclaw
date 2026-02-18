# Phase 1 Persistent Memory Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add persistent per-agent memory that survives runtime restarts.  
**Architecture:** Implement a JSON-file-backed memory store in `oax-runtime` keyed by `agent_id -> key -> value`. The store loads existing state on startup, allows in-memory updates via `put`, and persists via explicit `save`.  
**Tech Stack:** Rust (`serde`, `serde_json`, `std::fs`)  
**Requirement IDs:** INFRA-06

---

## Implemented Tasks

1. Added `oax-runtime::memory_store` module and exported it via `oax-runtime/src/lib.rs`.
2. Implemented `JsonFileMemoryStore` with:
- `new(path)` for load-or-empty initialization
- `put(agent_id, key, value)` for updates
- `get(agent_id, key)` for lookups
- `save()` for disk persistence
3. Added tests for:
- missing-file bootstrap behavior
- persisted values after reload from same path
- overwrite behavior for existing key/value entries

## Verification

- `cargo test -p oax-runtime memory_store::tests::loads_empty_store_when_file_missing -- --exact`
- `cargo test -p oax-runtime`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
