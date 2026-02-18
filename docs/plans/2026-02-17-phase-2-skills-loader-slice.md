# Phase 2 Skills Loader Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Introduce a Rust `oax-skills` crate that parses markdown skill files with YAML frontmatter and validates required binaries and risk policy constraints.  
**Architecture:** Add a new workspace crate with a small parser/validator API: parse frontmatter/body from markdown, deserialize frontmatter into typed structs, and enforce policy checks (`required_bins`, `risk`, trust + rollback requirements for destructive skills).  
**Tech Stack:** Rust (`serde`, `serde_yaml`, `which`)  
**Requirement IDs:** SKILL-01, SKILL-03, SKILL-08

---

## Implemented Tasks

1. Added workspace member `crates/oax-skills` with crate scaffolding.
2. Wrote failing tests for:
- valid skill markdown parsing
- missing frontmatter rejection
- required binary missing detection
- destructive skill policy rejection when `rollback_template` or `trust` is missing
3. Implemented parser and validation APIs:
- `parse_skill_markdown`
- `validate_required_bins`
- `validate_install_policy`
- `SkillFrontmatter` and `SkillRiskClass`
4. Ran verification:
- `cargo test -p oax-skills`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
5. Added docs/changelog/social artifacts and evidence record.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
