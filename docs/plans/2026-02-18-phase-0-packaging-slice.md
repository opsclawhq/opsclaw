# Phase 0 Packaging + Install Path (00-03) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Deliver single-binary packaging and curl-style install flow with deterministic local verification for self-hosted deployments.

**Architecture:** Add a release packaging script that emits `opsclaw-<target>.tar.gz`, an install script that supports both release URL and local archive sources, and user/developer docs for compose + install operations.

**Tech Stack:** Bash, Cargo, Docker Compose, Markdown docs

**Requirement IDs:** INFRA-01, INFRA-03

---

### Task 1: RED check before implementation

Run (RED):

```bash
test -f scripts/install/install-opsclaw.sh
```

Expected: failure because installer script does not exist yet.

### Task 2: Add packaging + installer scripts

**Files:**
- Create: `scripts/release/package-opsclaw.sh`
- Create: `scripts/install/install-opsclaw.sh`
- Create: `scripts/install/verify-local-install.sh`

### Task 3: Add install docs for users and contributors

**Files:**
- Create: `docs/user-guide/install-and-self-host.md`
- Create: `docs/developer-guide/release-packaging.md`
- Modify: `docs/user-guide/README.md`
- Modify: `docs/developer-guide/README.md`
- Modify: `docs/getting-started.md`

### Task 4: Changelog + evidence

**Files:**
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-0-packaging-slice.md`

### Task 5: Verification

Run:
- `test -f scripts/install/install-opsclaw.sh`
- `bash scripts/release/package-opsclaw.sh`
- `bash scripts/install/verify-local-install.sh`
- `docker compose -f docker/docker-compose.yaml config`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
