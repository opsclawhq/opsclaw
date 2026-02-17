# Skills Guide

OpsClaw execution follows a mandatory Superpowers skill chain for design, planning, implementation, review, and verification.

## Risk Classification Baseline (Phase 1)

Runtime and tools now use explicit risk classes:

- `READ`: read-only operations
- `SAFE_WRITE`: bounded mutating operations
- `DESTRUCTIVE`: high-impact mutating operations requiring stronger controls
- `FORBIDDEN`: blocked commands that must never run

Initial command classification is implemented in `oax_tools::risk::classify_command_risk`.
