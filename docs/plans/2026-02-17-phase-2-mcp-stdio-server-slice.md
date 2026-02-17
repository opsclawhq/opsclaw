# Phase 2 MCP Stdio Server Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Add `opsclaw mcp serve-stdio` so MCP clients can connect over stdio and use the runtime MCP contract.  
**Architecture:** Add a lightweight JSON-line stdio loop in `opsclaw` that supports `tools/list` and `tools/call` requests. Use `oax_runtime::mcp` for catalog and decisioning so transport logic stays thin and policy logic stays centralized.  
**Tech Stack:** Rust (`opsclaw`, `oax-runtime`, `serde_json`)  
**Requirement IDs:** INFRA-10

---

## Implemented Tasks

1. Added RED tests for stdio request handling (tools list, call decision, bad request).
2. Implemented `opsclaw` MCP stdio server module with:
- request/response envelope structs
- `handle_mcp_request_line`
- `serve_stdio`
3. Added CLI command `opsclaw mcp serve-stdio` in `main.rs`.
4. Ran verification:
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
5. Add docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
