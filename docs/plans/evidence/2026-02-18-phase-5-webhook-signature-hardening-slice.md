# Phase 5 Webhook Signature Hardening (05-17) Evidence

## RED (expected failure before implementation)

Command:

```bash
cargo test -p opsclaw webhook_runtime::tests::slack_signature_rejects_mismatch -- --exact
```

Observed failure (before implementation):

- compile failed because `verify_slack_request_signature` was not defined.
- log reference:
  - `~/Library/Application Support/rtk/tee/1771471737_cargo_test.log`

## GREEN (targeted signature verification behavior)

Command:

```bash
cargo test -p opsclaw webhook_runtime::tests::slack_signature_rejects_mismatch -- --exact
```

Observed:

- pass (`1 passed; 80 filtered out`).

## Integration Verification

Commands:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Observed:

- `opsclaw` tests pass (`81 passed`)
- workspace tests pass (`157 passed`)
- clippy clean (`No issues found`)
- docs validation pass (`release-doc validation passed`)

## Runtime Smoke (missing Slack signature headers)

Command sequence:

```bash
cargo run -p opsclaw -- run serve-webhooks --bind 127.0.0.1:8911 --max-requests 1 --slack-signing-secret signing-secret --slack-bot-user-id U_BOT --telegram-bot-username opsclaw_bot --slack-bot-token test-token --discord-bot-token test-token --telegram-bot-token test-token

curl -X POST http://127.0.0.1:8911/slack/events -H 'Content-Type: application/json' -d '{\"type\":\"url_verification\",\"challenge\":\"challenge-123\"}'
```

Observed response:

- `{"error":"missing slack request timestamp"}`
