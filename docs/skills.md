# Skills Guide

OpsClaw execution follows a mandatory Superpowers skill chain for design, planning, implementation, review, and verification.

## Risk Classification Baseline (Phase 1)

Runtime and tools now use explicit risk classes:

- `READ`: read-only operations
- `SAFE_WRITE`: bounded mutating operations
- `DESTRUCTIVE`: high-impact mutating operations requiring stronger controls
- `FORBIDDEN`: blocked commands that must never run

Initial command classification is implemented in `oax_tools::risk::classify_command_risk`.

## Skills Loader Baseline (Phase 2 Kickoff)

The new `oax-skills` crate parses markdown skills with YAML frontmatter:

- required frontmatter keys: `name`, `description`, `risk`
- optional frontmatter keys: `required_bins`, `trust`, `rollback_template`
- risk enum: `READ`, `SAFE_WRITE`, `DESTRUCTIVE`, `FORBIDDEN`

Policy checks currently enforce:

- required binaries must exist on host (`required_bins`)
- missing `trust` rejects installation policy
- `DESTRUCTIVE` skills require `rollback_template`

## Source Precedence (Phase 2)

Skill resolution precedence is now implemented as:

`bundled < global < workspace`

If the same skill name exists in multiple roots, the workspace-local file wins, then global, then bundled.
