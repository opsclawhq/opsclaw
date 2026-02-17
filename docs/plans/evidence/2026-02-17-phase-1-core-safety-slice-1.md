# Phase 1 Core Safety Slice 1 Evidence

Date: 2026-02-17  
Plan: `docs/plans/2026-02-17-phase-1-core-safety-slice-1.md`

## RED -> GREEN

1. RED: `cargo test -p oax-runtime` failed with 4 failing tests from unimplemented router and budget functions.
2. GREEN: Implemented `parse_agent_tags` and `ConversationBudget::with_limit`, then `cargo test -p oax-runtime` passed.
3. Added core contract modules and tests in `oax-core` (`agent`, `model`, `tool`, `memory`), then `cargo test -p oax-core` passed.
4. Re-ran `cargo test --workspace` and all tests passed.

## Command Evidence

```bash
cargo test -p oax-runtime
cargo test -p oax-core
cargo test --workspace
bash scripts/generate-types.sh
```

All commands passed in local verification after implementation.
