# Discord Adapter Preview (`opsclaw discord`)

Phase 5 `05-02` introduces deterministic Discord adapter contracts exposed through CLI helpers.

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

## Current Scope

This slice establishes routing, embed, and role-check contracts. Full live Discord gateway wiring and slash-command registration are covered in later Phase 5 slices.
