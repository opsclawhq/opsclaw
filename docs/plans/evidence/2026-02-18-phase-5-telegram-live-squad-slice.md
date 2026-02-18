# Phase 5 Live Telegram Squad Runtime (05-06) Evidence

## RED (expected failure before live runtime implementation)

Command:

```bash
cargo test -p opsclaw telegram_adapter::tests::live_loop_replies_to_group_mention -- --exact
```

Observed failure (before implementation):

- compile failed with unresolved symbols:
  - `TelegramLiveConfig`
  - `resolve_bot_token`
  - `MockTelegramApi`
  - `run_live_session`
- log reference:
  - `~/Library/Application Support/rtk/tee/1771404725_cargo_test.log`

## GREEN (targeted live runtime behavior)

Command:

```bash
cargo test -p opsclaw telegram_adapter::tests::live_loop_replies_to_group_mention -- --exact
```

Observed:

- pass (`1 passed; 44 filtered out`)

## Integration Verification

Commands:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Observed:

- `opsclaw` crate tests pass (`48 passed`)
- workspace tests pass (`124 passed`)
- clippy clean (`No issues found`)
- docs validation passes (`release-doc validation passed`)

## Live CLI Smoke (bounded)

Command:

```bash
cargo run -p opsclaw -- telegram live --bot-username opsclaw_bot --bot-token test-token --template sre-squad --max-updates 0
```

Observed:

- command starts live session and exits cleanly with bounded loop:
  - `updates_processed = 0`
  - `replies_sent = 0`
  - `last_update_id = null`
