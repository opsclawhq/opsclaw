# Slack Operator Handbook (Phase 3)

This handbook covers day-1 and day-2 operator workflows for the current Phase 3 Slack surface.

## 1. What Is Live in Phase 3

Current validated capabilities:

- `opsclaw slack install-url`: OAuth install URL generation contract
- `opsclaw slack route-event`: mention routing and thread-target selection contract
- `opsclaw slack retry-after`: rate-limit retry policy contract
- `opsclaw slack build-approval-card`: HITL approval payload generation
- `opsclaw slack parse-interaction`: approve/reject action parsing
- `opsclaw slack intro-message`: first-deploy bot intro message generation
- `opsclaw slack plan-discussion`: visible multi-agent discussion planning
- `opsclaw slack prepare-response`: long-response overflow to snippet payload + preview

## 2. First Deploy Runbook

1. Generate install URL and complete Slack app install:

```bash
opsclaw slack install-url \
  --client-id "$SLACK_CLIENT_ID" \
  --scope app_mentions:read,chat:write \
  --state opsclaw-phase3
```

2. Validate intro messages for each bot profile:

```bash
opsclaw slack intro-message --agent-json '{
  "name":"Remy",
  "role":"SRE",
  "specialty":"kubernetes",
  "personality":"calm and direct"
}'
```

3. Validate visible discussion plan for an incoming task:

```bash
opsclaw slack plan-discussion \
  --task "Investigate kubernetes crash loops" \
  --agents-json '[
    {"name":"Remy","role":"SRE","specialty":"kubernetes","personality":"calm and direct"},
    {"name":"Wren","role":"Cost","specialty":"cost","personality":"analytical"}
  ]'
```

## 3. Human-in-the-Loop Operations

Use approval cards for any non-read action.

1. Build approval payload:

```bash
opsclaw slack build-approval-card \
  --run-id run-123 \
  --command "kubectl delete pod api-1 -n prod"
```

2. Parse interactive decision payload:

```bash
opsclaw slack parse-interaction --payload-json "$SLACK_PAYLOAD_JSON"
```

Operational policy:

- do not execute mutating commands before an explicit approve decision
- reject decisions should be logged and surfaced back to the requester thread

## 4. Long Response Handling

When response content exceeds Slack message limits:

```bash
opsclaw slack prepare-response \
  --text "$LONG_TEXT" \
  --max-chars 3500 \
  --snippet-name "run-123-response.txt"
```

Expected behavior:

- short content returns inline payload
- long content returns snippet payload with preview and full content

## 5. Troubleshooting

Common checks:

- invalid Slack JSON payloads: confirm escaping and JSON schema
- missing intro fields: ensure `name`, `role`, and `personality` are present
- no specialist assignment: planner marks escalation when no specialty match is found
- overflow fallback not triggered: confirm `max-chars` is below content length

## 6. Live Relay Workflow (Phase 5)

Use `opsclaw slack live-event` to process a real Slack Events API payload and post a response back to the same thread.

```bash
export SLACK_BOT_TOKEN="xoxb-..."

opsclaw slack live-event \
  --bot-user-id U_BOT \
  --payload-json '{"type":"event_callback","event":{"type":"app_mention","channel":"C123","text":"<@U_BOT> squad","ts":"173.10","thread_ts":"173.01"}}' \
  --template sre-squad
```

Behavior:

- mention events route through shared squad responder logic and post via `chat.postMessage`
- replies preserve thread context through `thread_ts`
- non-mention events are ignored
- URL verification payloads return challenge data without posting
