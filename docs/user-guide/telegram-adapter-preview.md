# Telegram Adapter Preview (`opsclaw telegram`)

Phase 5 `05-03` introduces deterministic Telegram adapter contracts exposed through CLI helpers.

## Available Commands

Route a Telegram update:

```bash
cargo run -p opsclaw -- telegram route-event --payload-json '{"message":{"chat":{"id":42,"type":"group"},"text":"/status@opsclaw_bot"}}' --bot-username opsclaw_bot
```

Build an inline keyboard payload:

```bash
cargo run -p opsclaw -- telegram build-keyboard --buttons-json '[[{"text":"Approve","callback_data":"approve:run-1"}]]'
```

Check group chat support:

```bash
cargo run -p opsclaw -- telegram chat-support --chat-type supergroup
```

## Current Scope

This slice establishes routing, inline keyboard, and group-chat support contracts. Full live Telegram webhook transport and runtime bridge wiring are covered in later Phase 5 slices.
