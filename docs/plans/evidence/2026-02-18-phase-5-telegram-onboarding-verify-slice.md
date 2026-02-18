# Phase 5 Telegram Onboarding Verify (05-15) Evidence

## RED (expected failure on invalid credentials)

Command:

```bash
cargo run -p opsclaw -- telegram verify --bot-token invalid-token --expected-bot-username opsclaw_bot
```

Observed failure:

- command exits non-zero with explicit validation error:
  - `telegram verify failed: telegram getMe request failed ... status code 404`

## GREEN (targeted verification helper behavior)

Commands:

```bash
cargo test -p opsclaw telegram_adapter::tests::verify_bot_identity_rejects_username_mismatch -- --exact
cargo test -p opsclaw telegram_adapter::tests::verify_bot_identity_rejects_non_bot_identity -- --exact
```

Observed:

- both targeted tests pass.

## Integration Verification

Commands:

```bash
cargo test -p opsclaw
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/docs/validate-release-docs.sh
```

Observed:

- `opsclaw` tests pass (`74 passed`)
- workspace tests pass (`150 passed`)
- clippy clean (`No issues found`)
- docs validation pass (`release-doc validation passed`)
