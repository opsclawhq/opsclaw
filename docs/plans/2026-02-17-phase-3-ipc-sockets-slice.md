# Phase 3 IPC Sockets Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add NDJSON Unix socket IPC baseline (`runtime.sock`, `control.sock`) with versioned envelopes and a TypeScript SDK helper client.

**Architecture:** Introduce a typed IPC envelope in `oax-core`, message parsing/validation helpers in `oax-runtime`, and a dual-socket server in `opsclaw`. Keep handlers simple and deterministic for Phase 3 foundation work. Add SDK helper functions in `packages/sdk` that can connect, send one request line, and parse one response line.

**Tech Stack:** Rust (`serde`, `std::os::unix::net`), TypeScript (Node `net`), `typeshare`

**Requirement IDs:** CHAT-01, CHAT-06

---

### Task 1: Add failing IPC contract tests in runtime and server layers

**Files:**
- Modify: `crates/oax-runtime/src/lib.rs`
- Create: `crates/oax-runtime/src/ipc.rs`
- Modify: `crates/opsclaw/src/main.rs`
- Create: `crates/opsclaw/src/ipc_socket.rs`

**Step 1: Write failing tests for IPC envelope validation**
- Add tests for parse/serialize, schema version rejection, and unsupported message types.

**Step 2: Run tests to verify RED**
Run: `cargo test -p oax-runtime ipc::tests::rejects_invalid_schema_version -- --exact`
Expected: FAIL due to missing `ipc` module/contracts.

### Task 2: Implement runtime IPC types and handlers

**Files:**
- Create: `crates/oax-runtime/src/ipc.rs`
- Modify: `crates/oax-runtime/src/lib.rs`

**Step 3: Implement minimal IPC parser + handlers**
- Add handler functions for runtime/control channels.
- Return typed success/error envelopes.

**Step 4: Run runtime tests to verify GREEN**
Run: `cargo test -p oax-runtime ipc::tests::rejects_invalid_schema_version -- --exact`
Expected: PASS.

### Task 3: Implement dual Unix socket server in `opsclaw`

**Files:**
- Create: `crates/opsclaw/src/ipc_socket.rs`
- Modify: `crates/opsclaw/src/main.rs`

**Step 5: Add failing server test for socket request/response**
Run: `cargo test -p opsclaw ipc_socket::tests::runtime_socket_handles_ping_request -- --exact`
Expected: FAIL before server implementation.

**Step 6: Implement server + CLI command**
- New command: `opsclaw ipc serve-sockets --dir <path>`
- Create `runtime.sock` and `control.sock`.
- Handle NDJSON one-line request/response with graceful test-mode shutdown.

**Step 7: Run server tests to verify GREEN**
Run: `cargo test -p opsclaw ipc_socket::tests::runtime_socket_handles_ping_request -- --exact`
Expected: PASS.

### Task 4: Add shared contract types and generate TS types

**Files:**
- Modify: `crates/oax-core/src/types.rs`
- Modify: `packages/sdk/src/generated/types.ts`

**Step 8: Add `IpcEnvelope` contract and regenerate**
Run: `bash scripts/generate-types.sh`
Expected: generated TypeScript includes new IPC contract.

### Task 5: Add TypeScript SDK IPC client helper

**Files:**
- Create: `packages/sdk/src/ipc-client.ts`
- Create: `packages/sdk/src/index.ts`

**Step 9: Implement simple request helper**
- `sendIpcRequest(socketPath, envelope)` sends one line and returns parsed response.

### Task 6: Update docs/evidence/changelog and verify

**Files:**
- Modify: `docs/architecture.md`
- Modify: `docs/api-reference.md`
- Modify: `CHANGELOG.md`
- Create: `docs/plans/evidence/2026-02-17-phase-3-ipc-sockets-slice.md`

**Step 10: Verification**
Run:
- `cargo test -p oax-runtime`
- `cargo test -p opsclaw`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `bash scripts/generate-types.sh && git diff --exit-code packages/sdk/src/generated/types.ts`

