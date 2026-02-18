# Multi-Platform Routing Preview (`opsclaw channels`)

Phase 5 `05-04` introduces a platform-agnostic routing contract baseline for Slack, Discord, and Telegram.

## Available Command

Route a platform event into a unified contract:

```bash
cargo run -p opsclaw -- channels route-event --platform slack --payload-json '{"type":"event_callback","event":{"type":"app_mention","user":"U123","text":"<@UOPS> deploy status","channel":"C123","ts":"1000.1","thread_ts":"1000.1"}}' --identity UOPS
```

For Discord:

```bash
cargo run -p opsclaw -- channels route-event --platform discord --payload-json '{"type":2,"data":{"name":"status"},"member":{"roles":["ops"]}}'
```

For Telegram:

```bash
cargo run -p opsclaw -- channels route-event --platform telegram --payload-json '{"message":{"chat":{"id":42,"type":"group"},"text":"/status@opsclaw_bot"}}' --identity opsclaw_bot
```

## Current Scope

This slice normalizes routed events into one deterministic shape so downstream execution can stay platform-agnostic. Live runtime fan-out and production channel transports are completed in later slices.
