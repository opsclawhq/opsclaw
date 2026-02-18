# Setup Wizard Preview (`opsclaw init`)

Phase 5 introduces a setup wizard scaffold through `opsclaw init`.

## What It Does in 05-01

- selects a squad template (`sre-squad`, `dev-ops-team`, `incident-response`)
- builds an ordered setup plan
- reports estimated setup time and whether it fits the 60-second goal
- optionally writes the plan JSON to disk

## Usage

```bash
cargo run -p opsclaw -- init --template sre-squad --api-key "$OPENAI_API_KEY" --slack-workspace my-workspace
```

Write plan output to disk:

```bash
cargo run -p opsclaw -- init --template incident-response --write-plan --output .opsclaw/setup-wizard-plan.json
```

## Output Contract

The command prints JSON with:

- `template`
- `template_label`
- `estimated_seconds`
- `within_60_second_goal`
- `steps[]` with `id`, `title`, `estimated_seconds`, and `required`

## Hand-off to Telegram Live Runtime

After selecting a template in `opsclaw init`, verify Telegram credentials and then run live transport with the same template:

```bash
export TELEGRAM_BOT_TOKEN="<your-bot-token>"

cargo run -p opsclaw -- telegram verify \
  --expected-bot-username "<bot-username>" \
  --template sre-squad

cargo run -p opsclaw -- telegram live \
  --bot-username "<bot-username>" \
  --template sre-squad
```

## Current Limitation

`opsclaw init` remains a deterministic planning scaffold (no interactive TUI provisioning yet). Live Telegram transport is available via `opsclaw telegram live`, and setup validation is available via `opsclaw telegram verify`.
