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
- `opsclaw::slack_approval` adds deterministic Slack approval-card logic for:
  - mutating command approval-card generation via `oax_tools::approval`
  - Slack Block Kit payload rendering with blast radius and rollback guidance
  - interaction action parsing into approve/reject decisions
- additional `opsclaw slack ...` commands expose approval-card generation and interaction parsing for local HITL contract verification.
- `opsclaw::slack_collaboration` adds deterministic collaboration logic for:
  - first-deploy bot intro message generation from typed agent profiles
  - visible multi-agent discussion planning with specialist assignment and escalation markers
  - long-response overflow handling to snippet payload + preview
- additional `opsclaw slack ...` commands expose intro, discussion, and overflow contracts for local Phase 3 viral-moment verification without live Slack networking.

## Phase 4 Foundations (Kickoff)

- Mission Control API is defined contract-first in `docs/api/mission-control-openapi.yaml`.
- Shared dashboard payload models live in `oax-core::types` and are exported to TypeScript through `typeshare`.
- Current contract coverage includes agent summaries, kanban board snapshots, activity feed items, approval requests, and WebSocket event envelopes.
- Dashboard frontend foundation now exists at `packages/dashboard` with:
  - org hierarchy panel
  - agent profile panel
  - activity feed panel
  - deterministic view-model helpers and runnable Node tests (`node:test`)
- Dashboard state reducer and event stream simulation now cover:
  - kanban stage transitions across `Inbox -> Assigned -> In Progress -> Review -> Done`
  - approval queue lifecycle (`requested`, `approved`, `rejected`)
  - activity-feed append behavior from dashboard stream events
- Dashboard economics and transcript surfaces now cover:
  - per-agent token/spend/incident/minutes-saved accounting rows
  - ROI summary math (spend vs estimated value saved)
  - conversation transcript viewer with tool-call command/output entries

## Phase 5 Foundations (Kickoff)

- `opsclaw init` command now exposes setup-wizard planning contracts:
  - template selection (`sre-squad`, `dev-ops-team`, `incident-response`)
  - deterministic setup-step plan generation
  - estimated-seconds output with `within_60_second_goal` gate flag
  - optional plan writeout to `.opsclaw/setup-wizard-plan.json`
- `opsclaw discord` command now exposes deterministic Discord adapter contracts:
  - slash-command payload routing (`route-event`)
  - embed payload rendering (`build-embed`)
  - role authorization checks (`authorize`)
- `opsclaw telegram` command now exposes deterministic Telegram adapter contracts:
  - message update routing for group/private chats (`route-event`)
  - inline keyboard payload rendering (`build-keyboard`)
  - group-chat support detection (`chat-support`)
- `opsclaw channels` command now exposes deterministic multi-platform routing contracts:
  - platform-agnostic route normalization for Slack/Discord/Telegram (`route-event`)
  - unified route payload fields (`platform`, `route_kind`, `target_ref`, `text`)
- `scripts/content/phase-delivery-pipeline.sh` now exposes deterministic build-in-public artifact generation:
  - per-slice LinkedIn/X drafts in `.content/phase-<n>/`
  - per-slice blog recap/design drafts and machine-readable manifests in `docs/blog/`
  - structured docs navigation index in `docs/agent-index.yaml`
