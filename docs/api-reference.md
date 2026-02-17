# API Reference

## Alert Ingestion Payload Normalization (Phase 1 Preview)

OpsClaw runtime now includes a parser that normalizes webhook payloads from PagerDuty and Prometheus into a shared internal alert model.

### Supported Input Shapes

1. PagerDuty:
- path: `data.incident.id`
- optional: `data.incident.urgency` (defaults to `unknown`)
- optional: `data.incident.title` (defaults to `pagerduty incident`)

2. Prometheus:
- path: `alerts[0].labels.alertname`
- optional: `alerts[0].labels.severity` (falls back to top-level `status`, then `unknown`)
- optional: `alerts[0].annotations.summary` (defaults to `prometheus alert`)

### Runtime Contract

- parser entrypoint: `oax_runtime::alert::parse_alert_payload(&str)`
- return type: `Result<AlertPayload, String>`
- failure mode: unknown/unsupported shapes return an error string

This is the Phase 1 parser contract used by upcoming webhook endpoint wiring.

## HITL Approval Card Contract (Phase 1 Preview)

Mutating commands are now planned through approval-card generation before execution.

### Runtime Types

- `oax_tools::approval::ExecutionDecision`
  - `AllowReadOnly`
  - `RequireApproval(ApprovalCard)`
- `oax_tools::approval::ApprovalCard`
  - `command`
  - `expected_effect`
  - `blast_radius`
  - `rollback_steps`

### Planner Entrypoint

- `oax_tools::approval::plan_command_execution(command, rollback_template)`
- read-only commands bypass HITL (`AllowReadOnly`)
- mutating commands require approval with a populated card

## Risk Classification Contract (Phase 1 Preview)

Command-level risk metadata is now exposed via:

- `oax_tools::risk::RiskClass`
  - `Read`
  - `SafeWrite`
  - `Destructive`
  - `Forbidden`
- `oax_tools::risk::classify_command_risk(command)`

This baseline classification feeds approval and policy decisions for runtime execution.

## Tool Boundary Safety Contract (Phase 1 Preview)

Runtime now provides a tool-boundary contract that combines:
- credential injection for command templates
- risk/approval planning for command execution
- leak blocking for tool output before LLM context

Entrypoints:
- `oax_runtime::tool_boundary::prepare_tool_call(command_template, secrets, rollback_template)`
- `oax_runtime::tool_boundary::filter_tool_output_for_llm(output, leak_patterns)`

Result types:
- `PreparedToolCall { rendered_command, risk_class, decision }`
- `ToolBoundaryDecision::{AllowReadOnly, RequireApproval(ApprovalCard)}`

## NDJSON IPC Contract (Phase 3 Preview)

OpsClaw now includes a Unix socket IPC baseline with versioned NDJSON envelopes.

### Socket Paths

- runtime socket: `<dir>/runtime.sock`
- control socket: `<dir>/control.sock`

CLI entrypoint:

- `opsclaw ipc serve-sockets --dir <path>`

### Envelope Type

- `oax_core::types::IpcEnvelope`
  - `schema_version`
  - `message_type`
  - `run_id`
  - `payload_json`
  - `ok`
  - `error`

Current schema constant:

- `oax_runtime::ipc::IPC_SCHEMA_VERSION = "opsclaw.ipc.v1alpha1"`

### Runtime Message Types

- `runtime.ping` -> `runtime.pong`
- `runtime.forward` -> `runtime.forward.ack`

### Control Message Types

- `control.health` -> `control.health.ok`
- `control.stop` -> `control.stop.ack` (requests socket server shutdown)

Malformed input, invalid schema versions, and unsupported message types produce an `error` envelope.
