# Phase 3 NDJSON Unix Socket IPC Design

## Scope
Implement the first Phase 3 IPC baseline with two Unix sockets (`runtime.sock`, `control.sock`) using NDJSON envelopes and a schema-version field, plus a TypeScript SDK client for writing/reading those envelopes.

## Options Considered

1. HTTP/WebSocket first.
2. Unix socket NDJSON first (recommended).
3. In-memory API only, defer transport.

## Selected Approach
Option 2. Unix sockets keep local integration simple, avoid HTTP infrastructure overhead, and align with the roadmap contract for `runtime.sock` and `control.sock`.

## Contract

- Envelope fields: `schema_version`, `message_type`, `run_id`, `payload_json`.
- Schema version constant: `opsclaw.ipc.v1alpha1`.
- Runtime socket behavior:
  - accepts NDJSON envelopes
  - returns NDJSON envelopes
  - supports `runtime.ping` and generic `runtime.forward`
- Control socket behavior:
  - supports `control.health`
  - supports `control.stop` (server-level stop signal)

## Data Flow

1. Client connects to Unix socket path.
2. Client writes one JSON line.
3. Server parses line into typed envelope.
4. Server routes by socket role and `message_type`.
5. Server writes one JSON response line.

## Failure Modes

- Malformed JSON -> error response envelope.
- Invalid schema version -> error response envelope.
- Unsupported message type -> error response envelope.
- Stale socket file at startup -> removed then rebound.

## Test Strategy

- Unit tests for envelope parse/serialize and schema validation.
- Runtime handler tests for ping/forward/unknown type.
- Control handler tests for health/stop.
- Unix socket integration test with `max_messages` shutdown guard.
- Type generation sync test via `scripts/generate-types.sh`.
