# Phase 5 Slack Live Relay Transport (05-09) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 5  
**Goal:** Add a live Slack event relay that posts squad responses back to Slack threads/channels.  
**Architecture:** Extend `slack_adapter` with API trait + token resolver + live-event handler, then expose through `opsclaw slack live-event`.  
**Tech Stack:** Rust (`serde`, `serde_json`, `ureq`, `clap`)  
**Requirement IDs:** CHAT-01, CHAT-02, CHAT-08

---

## Execution Policy

- Canonical precedence: `ROADMAP > REQUIREMENTS > PROJECT`
- Branch name: `codex/phase-5-slack-live-relay`
- TDD is mandatory: RED -> GREEN -> REFACTOR for each task
- Code review is mandatory after each task or batch
- Verification is mandatory before any completion claim

### Task 1: RED tests for Slack live relay behavior

**Files:**
- Modify: `crates/opsclaw/src/slack_adapter.rs`

**Step 1: Write failing tests (RED)**
- Add tests for:
  - mention payload triggers `chat.postMessage` request through mock API.
  - URL verification payload returns challenge response without posting.
  - non-routable payloads are ignored without posting.

**Step 2: Run RED**

Run:
```bash
cargo test -p opsclaw slack_adapter::tests::live_event_routes_mention_and_posts_reply -- --exact
```

Expected: fail before live relay implementation exists.

### Task 2: Implement live relay in `slack_adapter`

**Files:**
- Modify: `crates/opsclaw/src/slack_adapter.rs`
- Modify: `crates/opsclaw/src/squad_responder.rs`

**Step 1: Add minimal implementation**
- Add `SlackApi` trait + `HttpSlackApi`.
- Add Slack token resolver.
- Add live-event handler with decision enum.
- Reuse `squad_responder` shared rendering for mention text.

**Step 2: Run targeted GREEN**

Run:
```bash
cargo test -p opsclaw slack_adapter::tests::live_event_routes_mention_and_posts_reply -- --exact
```

Expected: pass.

### Task 3: Wire `opsclaw slack live-event` command

**Files:**
- Modify: `crates/opsclaw/src/main.rs`

**Step 1: Add CLI command and output contract**
- Parse bot user id, token/env, payload JSON, template.
- Execute live-event handler and print result JSON.

**Step 2: Verify integration**

Run:
```bash
cargo test -p opsclaw
```

Expected: pass.

### Task 4: Docs, changelog, evidence

**Files:**
- Modify: `docs/user-guide/slack-operator-handbook.md`
- Modify: `docs/developer-guide/slack-integration.md`
- Modify: `docs/architecture.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-18-phase-5-slack-live-relay-slice.md`

**Step 1: Update operator/contributor runbooks**
- Add live relay invocation examples and expected outcomes.

**Step 2: Add RED/GREEN evidence commands/observations.**

### Task 5: Final verification gate

Run:
```bash
cargo test -p opsclaw slack_adapter::tests::live_event_routes_mention_and_posts_reply -- --exact
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Expected: all commands pass.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
