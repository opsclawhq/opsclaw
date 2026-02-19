# Multi-Platform Runtime Guide (`opsclaw run`)

Phase 5 `05-08` + `05-11` + `05-12` + `05-13` + `05-14` + `05-16` + `05-17` + `05-18` adds a unified runtime core so Slack, Discord, and Telegram events can be processed by one squad response engine and relayed through live platform APIs.

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

### Optional Rate-Limit Guard

Enable in-process ingress rate limiting:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --webhook-rate-limit-max-requests 120 \
  --webhook-rate-limit-window-seconds 60 \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Behavior:
- requests above the configured limit return `429` with JSON error details
- limit is global for this server process across all webhook endpoints
- omit `--webhook-rate-limit-max-requests` to disable local rate limiting

### Optional Slack Signature Verification

Enable Slack-native webhook signature verification:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --slack-signing-secret "$SLACK_SIGNING_SECRET" \
  --slack-signature-tolerance-seconds 300 \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Behavior:
- when `--slack-signing-secret` is set, Slack requests must provide valid `X-Slack-Signature` and `X-Slack-Request-Timestamp`
- invalid or missing signature/timestamp returns `401`
- stale timestamps outside `--slack-signature-tolerance-seconds` return `401`

### Optional Discord Signature Verification

Enable Discord-native interaction signature verification:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --discord-public-key "$DISCORD_PUBLIC_KEY" \
  --discord-signature-tolerance-seconds 300 \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Behavior:
- when `--discord-public-key` is set, `/discord/interactions` requests must provide valid `X-Signature-Ed25519` and `X-Signature-Timestamp`
- invalid or missing signature/timestamp returns `401`
- stale timestamps outside `--discord-signature-tolerance-seconds` return `401`

### Optional Telegram Webhook Secret Verification

Enable Telegram webhook secret-token verification:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --telegram-webhook-secret-token "$TELEGRAM_WEBHOOK_SECRET_TOKEN" \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Behavior:
- when `--telegram-webhook-secret-token` is set, `/telegram/webhook` requests must provide `X-Telegram-Bot-Api-Secret-Token`
- invalid or missing secret token returns `401`
- when secret token is not configured, Telegram token verification is disabled

### Optional Live Retry Backoff Controls

Enable bounded retries for transient live relay failures:

```bash
cargo run -p opsclaw -- run serve-webhooks \
  --bind 127.0.0.1:8787 \
  --live-retry-max-attempts 3 \
  --live-retry-backoff-millis 250 \
  --slack-bot-user-id U_BOT \
  --telegram-bot-username opsclaw_bot \
  --slack-bot-token "$SLACK_BOT_TOKEN" \
  --discord-bot-token "$DISCORD_BOT_TOKEN" \
  --telegram-bot-token "$TELEGRAM_BOT_TOKEN"
```

Behavior:
- retries are bounded by `--live-retry-max-attempts`
- retryable errors include transport/request failures and rate-limit responses
- if an error exposes `retry_after_seconds=<n>`, runtime backs off for `n * 1000` ms
- otherwise runtime uses exponential backoff from `--live-retry-backoff-millis`
