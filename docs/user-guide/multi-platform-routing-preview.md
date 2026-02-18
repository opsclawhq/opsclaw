# Multi-Platform Runtime Guide (`opsclaw run`)

Phase 5 `05-08` adds a unified runtime core so Slack, Discord, and Telegram events can be processed by one squad response engine.

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

## Stdio Runtime Loop (NDJSON)

`run stdio` reads newline-delimited JSON events from stdin and emits routed responses:

```bash
printf '%s\n' \
  '{"platform":"telegram","payload_json":"{\"message\":{\"chat\":{\"id\":7,\"type\":\"private\"},\"text\":\"/squad\"}}","identity":"opsclaw_bot"}' \
  | cargo run -p opsclaw -- run stdio --template sre-squad --max-events 1
```

## Relationship to `opsclaw telegram live`

- `opsclaw telegram live` is the production Telegram transport loop.
- `opsclaw run` is the shared platform-agnostic runtime core used for unified event-to-response behavior and parity testing.
