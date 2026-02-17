# Phase 2 SOUL Injection Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Inject SOUL profiles into runtime system prompt assembly so agent identity directly shapes execution prompts.  
**Architecture:** Add a runtime prompt-composition module that consumes typed `SoulProfile` values and composes a deterministic system prompt containing identity metadata and SOUL system instructions. Expose a file-based helper that loads SOUL markdown and returns a fully composed prompt in one step.  
**Tech Stack:** Rust (`oax-core`, `oax-runtime`)  
**Requirement IDs:** BOT-01, BOT-02, BOT-03

---

## Implemented Tasks

1. Added RED tests for prompt composition and file-based SOUL injection APIs in `oax-runtime`.
2. Implemented `oax-runtime::prompt` module with:
- `compose_system_prompt`
- `compose_system_prompt_from_file`
3. Wired module export in `crates/oax-runtime/src/lib.rs`.
4. Ran verification:
- `cargo test -p oax-runtime`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
5. Add docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
