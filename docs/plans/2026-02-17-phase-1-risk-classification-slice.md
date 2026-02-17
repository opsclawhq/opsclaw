# Phase 1 Risk Classification Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add explicit risk classification for runtime command governance.  
**Architecture:** Implement `oax-tools::risk` with a `RiskClass` enum and a command classifier that maps shell/tool commands into `Read`, `SafeWrite`, `Destructive`, or `Forbidden` categories using deterministic prefix/marker rules.  
**Tech Stack:** Rust (`oax-tools`)  
**Requirement IDs:** SAFE-08

---

## Implemented Tasks

1. Added `oax-tools::risk` module and exported it via `oax-tools/src/lib.rs`.
2. Implemented:
- `RiskClass` enum
- `classify_command_risk(command) -> RiskClass`
3. Added rule helpers for forbidden and destructive markers.
4. Added tests for:
- read-only classification
- safe-write classification
- destructive classification
- forbidden classification

## Verification

- `cargo test -p oax-tools risk::tests::read_only_commands_are_classified_as_read -- --exact`
- `cargo test -p oax-tools`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
