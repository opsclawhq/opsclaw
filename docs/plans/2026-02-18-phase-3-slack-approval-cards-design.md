# Phase 3 Slack Approval Cards Design

## Scope
Add a deterministic approval-card contract for Slack with two capabilities:
1. generate interactive Block Kit payloads for mutating commands
2. parse Slack interaction callbacks into approve/reject decisions

This slice does not include live Slack network transport, signing verification, or persistence.

## Options Considered

1. Full Slack interaction webhook implementation now.
2. Pure contract module with CLI validation helpers first (recommended).
3. Defer approval cards until final Phase 3 integration.

## Selected Approach
Option 2. It keeps behavior testable and minimizes integration risk before adding live webhook handlers.

## Data Flow

1. Command enters approval planner.
2. Runtime risk planner (`oax_tools::approval::plan_command_execution`) determines if approval is required.
3. If required, adapter builds a Slack card with blast radius and rollback details.
4. Slack callback payload contains `action_id`.
5. Adapter parses `action_id` into `{run_id, approve|reject}`.

## Failure Modes

- Read-only command passed to card builder -> explicit error.
- Missing run ID or command -> explicit validation error.
- malformed interaction payload JSON -> parse error.
- unknown action id format -> parse error.

## Test Strategy

- card builder includes command/effect/blast/rollback and both action IDs.
- action IDs round-trip parse for approve and reject.
- malformed payload and invalid action IDs are rejected.
- read-only commands do not generate approval cards.
