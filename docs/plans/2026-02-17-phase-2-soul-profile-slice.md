# Phase 2 SOUL Profile Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Add typed SOUL.md profile parsing and bundled preset personality files.  
**Architecture:** Implement `oax-core::soul` parser for markdown frontmatter + body, producing typed profile fields (name, role, personality, communication style, avatar). Add bundled preset SOUL files and discovery helper for runtime selection.  
**Tech Stack:** Rust (`serde`, `serde_yaml`) + markdown assets  
**Requirement IDs:** BOT-01, BOT-02, BOT-03

---

## Implemented Tasks

1. Added RED tests for SOUL parser/discovery APIs and verified failing state.
2. Implemented parser/types and bundled preset discovery:
- `parse_soul_markdown`
- `load_soul_file`
- `preset_soul_paths`
3. Added preset SOUL files for Remy, Ferris, and Wren under `souls/presets/`.
4. Ran verification:
- `cargo test -p oax-core`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
5. Add docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
