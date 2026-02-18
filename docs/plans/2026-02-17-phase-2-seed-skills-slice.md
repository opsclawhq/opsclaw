# Phase 2 Seed Skills Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Add five bundled seed skill markdown files and validate that each parses and satisfies install policy rules.  
**Architecture:** Create `skills/bundled/*.md` definitions for K8s Pod Debugger, Log Analyzer, Incident Responder, PR Reviewer, and Cost Optimizer. Add tests in `oax-skills` that parse each file and enforce policy checks (`trust`, rollback for destructive).  
**Tech Stack:** Markdown + Rust tests (`oax-skills`)  
**Requirement IDs:** SKILL-05, SKILL-06, SKILL-07, SKILL-08, SKILL-09

---

## Implemented Tasks

1. Added five seed skill markdown files in `skills/bundled/`:
- `k8s-pod-debugger.md`
- `log-analyzer.md`
- `incident-responder.md`
- `pr-reviewer.md`
- `cost-optimizer.md`
2. Added failing test for bundled skill discovery (`bundled_seed_skill_paths`).
3. Implemented `bundled_seed_skill_paths` API and ensured bundled skill files are discoverable.
4. Ran verification:
- `cargo test -p oax-skills`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
5. Added docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
