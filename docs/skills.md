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

## Skill Install Command (Phase 2)

You can install a validated local skill file with:

`opsclaw skill install /path/to/skill.md`

Install-time checks:
- markdown frontmatter parses successfully
- trust policy is satisfied
- destructive skills include rollback template
- required binaries exist on host

## Bundled Seed Skills (Phase 2)

Bundled skills now ship at `skills/bundled/`:

- `k8s-pod-debugger.md`
- `log-analyzer.md`
- `incident-responder.md`
- `pr-reviewer.md`
- `cost-optimizer.md`

## SOUL Profiles (Phase 2)

Bot identity is now represented by typed SOUL markdown profiles in `oax-core`:

- `parse_soul_markdown` parses frontmatter + system prompt body.
- `load_soul_file` loads profile files directly for user customization flows.
- `preset_soul_paths` discovers bundled presets under `souls/presets/`.

Bundled SOUL presets currently include:
- `remy.md` (SRE)
- `ferris.md` (Deploy Bot)
- `wren.md` (Cost Optimizer)

Runtime now injects SOUL identity into the final system prompt via `oax-runtime::prompt`:

- `compose_system_prompt(base, soul)` for direct typed composition
- `compose_system_prompt_from_file(base, soul_path)` for file-backed composition

This makes SOUL differences explicit in prompt text and testable end-to-end.
