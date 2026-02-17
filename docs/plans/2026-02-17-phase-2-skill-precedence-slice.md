# Phase 2 Skill Precedence Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Implement skill source precedence resolution (`bundled < global < workspace`) in `oax-skills`.  
**Architecture:** Add a path-resolution module that scans three roots for `*.md` skills, indexes by skill name, and resolves duplicates by precedence where workspace-local overrides global, and global overrides bundled.  
**Tech Stack:** Rust (`std::fs`, `std::path`, `tempfile`)  
**Requirement IDs:** SKILL-02, SKILL-10

---

## Implemented Tasks

1. Added precedence module tests first (RED):
- merge behavior across bundled/global/workspace roots
- same-name override behavior
- absent skill handling
2. Implemented precedence resolver APIs:
- `SkillSource`
- `ResolvedSkill`
- `resolve_skill_catalog(bundled, global, workspace)`
3. Ran verification:
- `cargo test -p oax-skills`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
4. Added docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
