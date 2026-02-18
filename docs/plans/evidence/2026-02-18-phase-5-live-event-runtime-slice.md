# Phase 5 Unified Live Event Runtime Bridge (05-11) Evidence

## RED (expected failure before implementation)

Command:

```bash
cargo test -p opsclaw telegram_adapter::tests::live_event_posts_reply_for_group_mention -- --exact
```

Observed failure (before implementation):

- compile failed with missing symbols for one-shot Telegram live handling:
  - `TelegramLiveDecision`
  - `handle_live_event`
- log reference:
  - `~/Library/Application Support/rtk/tee/1771420861_cargo_test.log`

## GREEN (targeted Telegram live bridge behavior)

Command:

```bash
cargo test -p opsclaw telegram_adapter::tests::live_event_posts_reply_for_group_mention -- --exact
```

Observed:

- pass (`1 passed; 62 filtered out`)

## Integration Verification

Commands:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Observed:

- `opsclaw` tests pass (`63 passed`)
- workspace tests pass (`139 passed`)
- clippy clean (`No issues found`)
- docs validation pass (`release-doc validation passed`)

## Live CLI Smoke

Commands:

```bash
cargo run -p opsclaw -- run live-event --platform slack --payload-json '{"type":"url_verification","challenge":"challenge-123"}' --identity U_BOT --slack-bot-token test-token --template sre-squad
cargo run -p opsclaw -- run live-event --platform discord --payload-json '{"type":1}' --discord-bot-token test-token --template sre-squad
cargo run -p opsclaw -- run live-event --platform telegram --payload-json '{"message":{"chat":{"id":42,"type":"private"},"text":""}}' --identity opsclaw_bot --telegram-bot-token test-token --template sre-squad
```

Observed:

- Slack URL verification is returned directly (`decision=url_verification`, `challenge=challenge-123`).
- Discord non-slash payload is ignored (`decision=ignore`).
- Telegram non-routable payload is ignored (`decision=ignore`).
