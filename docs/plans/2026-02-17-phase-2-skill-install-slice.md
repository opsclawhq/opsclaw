# Phase 2 Skill Install Command Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 2  
**Goal:** Add `opsclaw skill install <path>` command that installs a skill markdown file after trust/risk and required-bin validation.  
**Architecture:** Extend CLI with `skill install` subcommand and add installation helper that parses skill markdown via `oax-skills`, enforces install policy and binary presence, and writes validated skill files into `~/.opsclaw/skills/`.  
**Tech Stack:** Rust (`clap`, `std::fs`, `oax-skills`)  
**Requirement IDs:** SKILL-04, SKILL-03

---

## Implemented Tasks

1. Added tests first for skill install helper:
- successful install path
- rejection for missing trust/rollback policy
- rejection for missing required binaries
2. Implemented install helper + CLI wiring:
- `install_skill_from_file`
- `install_skill_to_default_location`
- `opsclaw skill install <path>` command
3. Ran verification:
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
4. Added docs/changelog/evidence/social artifacts.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
