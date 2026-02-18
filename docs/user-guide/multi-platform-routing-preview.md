# Multi-Platform Runtime Guide (`opsclaw run`)

Phase 5 `05-08` + `05-11` + `05-12` + `05-13` + `05-14` adds a unified runtime core so Slack, Discord, and Telegram events can be processed by one squad response engine and relayed through live platform APIs.

## One-Shot Event Processing

Use `run route-event` to process a single raw platform payload and get the routed squad response:

```bash
cargo run -p opsclaw -- run route-event \
  --platform slack \
  --payload-json '{"type":"event_callback","event":{"type":"app_mention","user":"U123","text":"<@UOPS> squad","channel":"C123","ts":"1000.1","thread_ts":"1000.1"}}' \
  --identity UOPS \
  --template sre-squad
```

Discord:

```bash
cargo run -p opsclaw -- run route-event \
  --platform discord \
  --payload-json '{"type":2,"data":{"name":"squad"},"member":{"roles":["ops"]}}' \
  --template sre-squad
```

Telegram:

```bash
cargo run -p opsclaw -- run route-event \
  --platform telegram \
  --payload-json '{"message":{"chat":{"id":42,"type":"group"},"text":"/squad@opsclaw_bot"}}' \
  --identity opsclaw_bot \
  --template sre-squad
```

## Unified Live API Relay

Use `run live-event` when you want one runtime entrypoint for platform-native live relay behavior:

Slack (requires bot user id in `--identity`):

```bash
cargo run -p opsclaw -- run live-event \
  --platform slack \
  --payload-json '{"type":"url_verification","challenge":"challenge-123"}' \
  --identity U_BOT \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --template sre-squad
```

Discord:

```bash
cargo run -p opsclaw -- run live-event \
  --platform discord \
  --payload-json '{"type":1}' \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --template sre-squad
```

Telegram (requires bot username in `--identity`):

```bash
cargo run -p opsclaw -- run live-event \
  --platform telegram \
  --payload-json '{"message":{"chat":{"id":42,"type":"private"},"text":""}}' \
  --identity opsclaw_bot \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN" \
  --template sre-squad
```

## Stdio Runtime Loop (NDJSON)

`run stdio` reads newline-delimited JSON events from stdin and emits routed responses:

```bash
printf '%s\n' \
  '{"platform":"telegram","payload_json":"{\"message\":{\"chat\":{\"id\":7,\"type\":\"private\"},\"text\":\"/squad\"}}","identity":"opsclaw_bot"}' \
  | cargo run -p opsclaw -- run stdio --template sre-squad --max-events 1
```

## Continuous Live Relay Loop

`run live-stdio` reads newline-delimited inbound events and dispatches each to live platform handlers:

```bash
printf '%s\n' \
  '{"platform":"slack","payload_json":"{\"type\":\"url_verification\",\"challenge\":\"challenge-123\"}","identity":"U_BOT"}' \
  '{"platform":"discord","payload_json":"{\"type\":1}","identity":null}' \
  '{"platform":"telegram","payload_json":"{\"message\":{\"chat\":{\"id\":42,\"type\":\"private\"},\"text\":\"\"}}","identity":"opsclaw_bot"}' \
  | cargo run -p opsclaw -- run live-stdio \
      --template sre-squad \
      --slack-bot-token "$SLACK_BOT_TOKEN" \
      --discord-bot-token "$DISCORD_BOT_TOKEN" \
      --telegram-bot-token "$TELEGRAM_BOT_TOKEN" \
      --max-events 3
```

## Relationship to `opsclaw telegram live`

- `opsclaw telegram live` is the production Telegram transport loop.
- `opsclaw run route-event` is the shared platform-agnostic runtime contract for route/response parity testing.
- `opsclaw run live-event` is the runtime-level live relay bridge for Slack/Discord/Telegram API posting paths.
- `opsclaw run live-stdio` is the always-on NDJSON relay loop for mixed multi-platform live events in one process.

## Native Webhook Ingress

`run serve-webhooks` hosts built-in HTTP endpoints for Slack/Discord/Telegram webhook POSTs:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Endpoints:
- `POST /slack/events`
- `POST /discord/interactions`
- `POST /telegram/webhook`

### Optional Shared-Secret Guard

Enable a request-level shared-secret check to reject unauthorized ingress:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --webhook-shared-secret "$OPSCLAW_WEBHOOK_SECRET" \
  --webhook-secret-header X-OpsClaw-Webhook-Secret \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Behavior:
- when `--webhook-shared-secret` is set and the header is missing, ingress responds `401`
- when the header value mismatches, ingress responds `401`
- when no shared secret is configured, ingress remains open (local/dev mode)
