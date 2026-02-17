# Phase 1 Event Journal Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add append-only JSONL runtime event journaling with replay support for auditability.  
**Architecture:** Implement a `JsonlEventJournal` in `oax-runtime::events` with append and read-all operations over newline-delimited JSON events, plus tests for ordered replay and missing-file behavior.  
**Tech Stack:** Rust (`serde`, `serde_json`, `tempfile`)  
**Requirement IDs:** SAFE-05, INFRA-07

---

## Implemented Tasks

1. Added `oax-runtime::events` module with:
- `RuntimeEvent` record type (`schema_version`, `event_type`, `run_id`, `payload_json`)
- `JsonlEventJournal::append`
- `JsonlEventJournal::read_all`
2. Added tests:
- ordered append/replay test
- missing journal file returns empty event list
3. Updated runtime dependencies for serialization and tempfile-backed tests.

## Verification

- `cargo test -p oax-runtime`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
