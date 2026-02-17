# Phase 2 MCP Integration Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Add a runtime MCP integration contract that exposes OpsClaw tools as MCP tool definitions and validates incoming MCP calls against safety policy.  
**Architecture:** Add `oax-runtime::mcp` module with typed MCP tool descriptors and call-validation entrypoints. Leverage existing risk classification and approval planning from `oax-tools` so MCP-originated calls follow the same safety gates as native runtime calls.  
**Tech Stack:** Rust (`oax-runtime`, `oax-tools`)  
**Requirement IDs:** INFRA-10

---

## Implemented Tasks

1. Added RED tests for MCP tool catalog and call-validation behavior.
2. Implemented `oax-runtime::mcp` module with:
- `McpToolDefinition`
- `McpCallDecision`
- `opsclaw_mcp_tools`
- `evaluate_mcp_call`
3. Wired module export in `crates/oax-runtime/src/lib.rs`.
4. Ran verification:
- `cargo test -p oax-runtime`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
5. Add docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
