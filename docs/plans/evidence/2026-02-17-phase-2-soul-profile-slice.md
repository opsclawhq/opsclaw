# Phase 2 SOUL Profile Slice Evidence

Date: 2026-02-17  
Plan: `docs/plans/2026-02-17-phase-2-soul-profile-slice.md`

## RED -> GREEN

1. RED: added SOUL discovery test before creating bundled preset files.
2. RED verification command:
   - `cargo test -p oax-core soul::tests -- --nocapture`
   - Result: `preset_souls_are_discoverable` failed because `souls/presets/remy.md` was missing.
3. GREEN: added parser/loading implementation and bundled presets:
   - `parse_soul_markdown`
   - `load_soul_file`
   - `preset_soul_paths`
   - `souls/presets/remy.md`, `souls/presets/ferris.md`, `souls/presets/wren.md`
4. GREEN verification:
   - `cargo test -p oax-core soul::tests -- --nocapture` passed.
   - `cargo test -p oax-core` passed.
5. Broad verification:
   - `cargo test --workspace` passed.
   - `cargo clippy --workspace --all-targets` passed.

## Command Evidence

```bash
cargo test -p oax-core soul::tests -- --nocapture
cargo test -p oax-core
cargo test --workspace
cargo clippy --workspace --all-targets
```
