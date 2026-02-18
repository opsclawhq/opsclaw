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

## Slack Adapter Contract (Phase 3 Preview)

OpsClaw now includes a pure Slack adapter contract in `opsclaw::slack_adapter`.

### OAuth Install URL

Type:

- `SlackInstallConfig`
  - `client_id`
  - `scopes`
  - `user_scopes`
  - `redirect_uri`
  - `state`

Function:

- `build_install_url(&SlackInstallConfig) -> Result<String, String>`

CLI helper:

- `opsclaw slack install-url --client-id ... --scope ... --state ...`

### Event Routing

Function:

- `route_for_bot(payload_json, bot_user_id) -> Result<SlackRouteDecision, String>`

Decisions:

- `UrlVerification { challenge }`
- `Mention(SlackMentionRoute { channel, thread_ts, cleaned_text, user_id })`
- `Ignore`

Threading rule:

- if `event.thread_ts` exists, reply to it
- otherwise reply to `event.ts`

### Rate Limit Policy

Function:

- `retry_after_seconds(status_code, retry_after_header) -> Option<u64>`

Behavior:

- returns parsed seconds only for HTTP `429` with valid `Retry-After` value
- all other cases return `None`

## Slack Approval Card Contract (Phase 3 Preview)

OpsClaw now includes a pure Slack approval-card contract in `opsclaw::slack_approval`.

### Approval Card Generation

Function:

- `build_approval_card(run_id, command, rollback_template) -> Result<SlackApprovalCard, String>`

Behavior:

- uses `oax_tools::approval::plan_command_execution` to derive:
  - `expected_effect`
  - `blast_radius`
  - `rollback_steps`
- rejects read-only commands (`AllowReadOnly`) for card generation
- emits stable action IDs:
  - `opsclaw:approval:<run_id>:approve`
  - `opsclaw:approval:<run_id>:reject`

Slack payload renderer:

- `card_to_block_kit_json(&SlackApprovalCard) -> serde_json::Value`

### Interaction Decision Parsing

Function:

- `parse_interaction_decision(payload_json) -> Result<SlackInteractionDecision, String>`

Decision enum:

- `ApprovalDecision::Approve`
- `ApprovalDecision::Reject`

Action ID format expected:

- `opsclaw:approval:<run_id>:approve|reject`

### CLI Helpers

- `opsclaw slack build-approval-card --run-id ... --command ... [--rollback-template ...]`
- `opsclaw slack parse-interaction --payload-json ...`

## Slack Collaboration Contract (Phase 3 Preview)

OpsClaw now includes deterministic Slack collaboration helpers in `opsclaw::slack_collaboration`.

### Intro Message Generation

Function:

- `build_intro_message(profile) -> Result<String, String>`

Profile type:

- `AgentProfile`
  - `name`
  - `role`
  - `specialty`
  - `personality`

Behavior:

- emits a human-readable first-deploy intro message
- validates required profile fields (`name`, `role`, `personality`)

### Visible Discussion Planning

Function:

- `plan_visible_discussion(task, agents) -> Result<DiscussionPlan, String>`

Plan type:

- `DiscussionPlan`
  - `assignee`
  - `escalation_required`
  - `turns: Vec<DiscussionTurn>`

Behavior:

- requires at least two agents
- selects a specialist assignee based on task/specialty keyword overlap
- marks escalation when no specialty match exists
- emits visible in-channel turns for multi-agent discussion

### Long Response Overflow Handling

Function:

- `prepare_response_for_slack(text, max_chars, snippet_name) -> Result<SlackResponsePayload, String>`

Payload enum:

- `SlackResponsePayload::Inline { text }`
- `SlackResponsePayload::Snippet { preview, file_name, content }`

Behavior:

- under-limit response stays inline
- over-limit response moves to snippet payload with bounded preview and full content preserved

### CLI Helpers

- `opsclaw slack intro-message --agent-json ...`
- `opsclaw slack plan-discussion --task ... --agents-json ...`
- `opsclaw slack prepare-response --text ... [--max-chars ...] [--snippet-name ...]`
