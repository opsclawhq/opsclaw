# Architecture

Phase 0 establishes a Rust workspace, type boundary generation, and a local infrastructure stack.

## Phase 1 Runtime Primitives (In Progress)

- `router`: parses `[@agent: payload]` tags for TinyClaw-style inter-agent routing.
- `executor`: per-agent FIFO queue manager with pending counter accounting.
- `budget`: hard conversation ceiling defaults (50 messages) for runaway-loop control.
- `events`: append-only JSONL journal for audit replay.
- `alert`: payload normalization for PagerDuty and Prometheus webhook formats.
- `heartbeat`: per-agent liveness tracking with interval and missed-beat tolerance.
- `memory_store`: JSON-backed per-agent key/value persistence across restarts.
- `cancellation`: run-level cancellation registry for stopping in-progress agent work.
- `simulation`: tagged ping-pong harness for phase-gate conversation drain/budget verification.
- `tool_boundary`: credential injection + risk/approval + leak-output filtering at execution boundary.

## Phase 2 Foundations (Kickoff)

- `oax-skills`: markdown frontmatter parser and policy validator (`required_bins`, trust, rollback rules).
