# Discord Adapter Preview (`opsclaw discord`)

Phase 5 `05-02` and `05-10` expose Discord routing contracts plus a live relay helper command.

## Available Commands

Route a Discord payload:

```bash
cargo run -p opsclaw -- discord route-event --payload-json '{"type":2,"data":{"name":"status"},"member":{"roles":["ops"]}}'
```

Build an embed payload:

```bash
cargo run -p opsclaw -- discord build-embed --title "OpsClaw" --description "Deployment healthy"
```

Check role authorization:

```bash
cargo run -p opsclaw -- discord authorize --required-role ops --roles-json '["viewer","ops"]'
```

Relay a slash-command payload to a real Discord channel (bot token required):

```bash
cargo run -p opsclaw -- discord live-event \
  --payload-json '{"type":2,"channel_id":"123456789012345678","data":{"name":"squad"},"member":{"roles":["ops"]}}' \
  --bot-token "$DISCORD_BOT_TOKEN" \
  --template sre-squad
```

## Current Scope

- `discord live-event` handles the slash payload contract and posts response text through Discord HTTP API (`channels/{id}/messages`).
- Payloads without slash type are ignored (`decision=ignore`).
- Payloads missing `channel_id` fail fast with an explicit error.
- Discord gateway/websocket ingestion and slash-command registration remain outside this slice.
