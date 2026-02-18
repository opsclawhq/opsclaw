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
- precedence resolver implemented for `bundled < global < workspace` skill source ordering.
- CLI install path wired: `opsclaw skill install <path>` validates then installs into `~/.opsclaw/skills/`.
- bundled seed skills added under `skills/bundled/` for core ops workflows.
- `oax-core::soul` provides typed SOUL profile parsing/loading plus bundled preset discovery from `souls/presets/`.
- `oax-runtime::prompt` composes runtime system prompts by injecting SOUL identity metadata and SOUL instructions.
- `oax-runtime::isolation` defines per-agent container spec validation and conversion to `bollard` create-container config with `network_mode=none` and scoped mounts.
- `oax-runtime::mcp` exposes baseline MCP tool definitions and evaluates MCP-originated calls through risk + approval policy gates.
- `opsclaw mcp serve-stdio` provides a stdio MCP transport endpoint for tool listing and policy-evaluated tool calls.

## Phase 3 Foundations (Kickoff)

- `oax-core::types::IpcEnvelope` defines the versioned NDJSON contract shared between Rust and TypeScript.
- `oax-runtime::ipc` provides parse/serialize helpers and deterministic runtime/control message handlers.
- `opsclaw ipc serve-sockets --dir <path>` starts dual Unix sockets:
  - `runtime.sock` for runtime-plane request forwarding.
  - `control.sock` for health and stop control messages.
- `packages/sdk/src/ipc-client.ts` provides a Node SDK helper for one-request/one-response socket calls.
- `opsclaw::slack_adapter` adds deterministic Slack adapter logic for:
  - OAuth install URL generation
  - event payload mention routing
  - thread reply target selection (`thread_ts` fallback to `ts`)
  - 429 retry policy extraction from `Retry-After`
- `opsclaw slack ...` CLI commands expose adapter behavior for local contract verification before live Slack transport wiring.
