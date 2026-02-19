# Phase 5 Discord Signature Hardening (05-18) Evidence

## RED (expected failure before implementation)

Command:

```bash
cargo test -p opsclaw webhook_runtime::tests::discord_signature_rejects_mismatch -- --exact
```

Observed failure (before implementation):

- compile failed because `verify_discord_request_signature` was not defined.
- log reference:
  - `~/Library/Application Support/rtk/tee/1771472264_cargo_test.log`

## GREEN (targeted signature verification behavior)

Command:

```bash
cargo test -p opsclaw webhook_runtime::tests::discord_signature_rejects_mismatch -- --exact
```

Observed:

- pass (`1 passed; 84 filtered out`).

## Integration Verification

Commands:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Observed:

- `opsclaw` tests pass (`85 passed`)
- workspace tests pass (`161 passed`)
- clippy clean (`No issues found`)
- docs validation pass (`release-doc validation passed`)

## Runtime Smoke (missing Discord signature headers)

Command sequence:

```bash
cargo run -p opsclaw -- run serve-webhooks --bind 127.0.0.1:8920 --max-requests 1 --discord-public-key 0000000000000000000000000000000000000000000000000000000000000000 --slack-bot-user-id U_BOT --telegram-bot-username opsclaw_bot --slack-bot-token test-token --discord-bot-token test-token --telegram-bot-token test-token

curl -X POST http://127.0.0.1:8920/discord/interactions -H 'Content-Type: application/json' -d '{"type":1}'
```

Observed response:

- `{"error":"missing discord request timestamp"}`
