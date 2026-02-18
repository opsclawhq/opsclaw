# Phase 5 Setup Wizard Scaffold (05-01) Design

## Scope
Implement the first Phase 5 slice by adding a setup wizard scaffold through `opsclaw init` with:

1. squad template selection contract
2. setup-step plan generation
3. sub-60-second estimated setup gate output

## Requirement Coverage
- BOT-04
- BOT-05
- BOT-07

## Options Considered

1. Build full interactive ratatui UI in one slice.
2. Build command contract and deterministic wizard-plan engine first (recommended).
3. Keep setup as static docs only.

## Selected Approach
Option 2. It creates testable wizard behavior and timing gates immediately while leaving terminal-UI polish for later slices.

## Architecture

- Add `setup_wizard` module in `crates/opsclaw`.
- Expose `opsclaw init` CLI command with template and setup context inputs.
- Produce deterministic JSON plan output containing:
  - selected template
  - ordered setup steps
  - estimated seconds
  - `within_60_second_goal` flag
- Add tests for template plan shape and timing gate behavior.

## Failure Modes

- template mapping missing required steps
- setup estimate exceeds 60-second target without signaling gate miss
- CLI output shape drifts and breaks downstream integration

## Verification Strategy

- RED: run targeted setup-wizard test before module exists
- GREEN: setup-wizard tests pass with expected estimates and output contracts
- full: workspace tests and clippy remain green
