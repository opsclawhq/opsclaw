# Phase 1 Tool Boundary Safety Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Integrate credential injection, approval/risk planning, and output leak blocking into one runtime tool-boundary contract.  
**Architecture:** Add `oax-runtime::tool_boundary` to orchestrate existing primitives from `oax-security` and `oax-tools`: inject secrets into command templates only at execution boundary, classify risk and approval requirement, and block secret-like output before it can reach LLM context.  
**Tech Stack:** Rust (`oax-runtime`, `oax-security`, `oax-tools`)  
**Requirement IDs:** SAFE-04, SAFE-06, SAFE-07

---

## Implemented Tasks

1. Added `oax-runtime::tool_boundary` module and exported it via `oax-runtime/src/lib.rs`.
2. Added `oax-runtime` dependencies on `oax-security` and `oax-tools`.
3. Implemented:
- `ToolBoundaryDecision`
- `PreparedToolCall`
- `prepare_tool_call(command_template, secrets, rollback_template)`
- `filter_tool_output_for_llm(output, leak_patterns)`
4. Added tests for:
- secret placeholder injection at tool boundary
- missing-secret rejection
- mutating command approval requirement
- leak-pattern output blocking before LLM context
- clean output pass-through

## Verification

- `cargo test -p oax-runtime tool_boundary::tests::injects_credentials_at_tool_boundary -- --exact`
- `cargo test -p oax-runtime`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
