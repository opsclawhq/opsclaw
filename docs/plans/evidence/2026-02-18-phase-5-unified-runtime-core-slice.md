# Phase 5 Unified Runtime Core (05-08) Evidence

## RED (expected failure before implementation)

Command:

```bash
cargo test -p opsclaw squad_runtime::tests::parity_for_squad_command_across_platforms -- --exact
```

Observed failure (before implementation):

- compile failed with unresolved symbol:
  - `process_inbound_event` not found in scope
- log reference:
  - `~/Library/Application Support/rtk/tee/1771418759_cargo_test.log`

## GREEN (targeted parity)

Command:

```bash
cargo test -p opsclaw squad_runtime::tests::parity_for_squad_command_across_platforms -- --exact
```

Observed:

- pass (`1 passed; 52 filtered out`)

## Integration Verification

Commands:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Observed:

- `opsclaw` tests pass (`53 passed`)
- workspace tests pass (`129 passed`)
- clippy clean (`No issues found`)
- docs validation pass (`release-doc validation passed`)

## Runtime Smoke

Commands:

```bash
cargo run -p opsclaw -- run route-event --platform discord --payload-json '{"type":2,"data":{"name":"squad"},"member":{"roles":["ops"]}}' --template sre-squad
printf '%s\n' '{"platform":"telegram","payload_json":"{\"message\":{\"chat\":{\"id\":7,\"type\":\"private\"},\"text\":\"/squad\"}}","identity":"opsclaw_bot"}' | cargo run -p opsclaw -- run stdio --template sre-squad --max-events 1
```

Observed:

- `run route-event` emitted routed squad response for Discord slash command.
- `run stdio` processed one telegram event and emitted one routed response.
