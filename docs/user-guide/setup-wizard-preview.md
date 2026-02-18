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

## Current Limitation

This slice provides deterministic wizard planning logic. Full interactive TUI flow and platform provisioning steps are scheduled in later Phase 5 slices.
