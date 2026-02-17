# Phase 1 Approval Card Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add HITL approval-card primitives so mutating commands require explicit human approval context before execution.  
**Architecture:** Implement an approval planner in `oax-tools` that classifies commands using read-only policy and returns either direct allow (`AllowReadOnly`) or a structured `ApprovalCard` with expected effect, blast radius, and rollback steps.  
**Tech Stack:** Rust (`oax-tools`)  
**Requirement IDs:** SAFE-02, SAFE-03, SAFE-09

---

## Implemented Tasks

1. Added `oax-tools::approval` module and exported it via `oax-tools/src/lib.rs`.
2. Implemented:
- `ApprovalCard`
- `ExecutionDecision`
- `plan_command_execution(command, rollback_template)`
3. Added planner helpers for:
- expected-effect inference
- blast-radius inference
- default rollback guidance generation
4. Added tests for:
- read-only commands bypassing HITL
- mutating commands requiring approval cards
- explicit rollback template propagation

## Verification

- `cargo test -p oax-tools approval::tests::read_only_command_is_allowed_without_hitl -- --exact`
- `cargo test -p oax-tools`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
