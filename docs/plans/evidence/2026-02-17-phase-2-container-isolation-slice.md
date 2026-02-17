# Phase 2 Container Isolation Slice Evidence

Date: 2026-02-17  
Plan: `docs/plans/2026-02-17-phase-2-container-isolation-slice.md`

## RED -> GREEN

1. RED: added isolation tests before implementing container isolation symbols.
2. RED verification command:
   - `cargo test -p oax-runtime isolation::tests::build_spec_enforces_no_host_network_and_read_only_root -- --exact`
   - Result: compile failure due to unresolved `build_agent_container_spec`, `validate_isolation_spec`, `to_bollard_config`, and `AgentContainerSpec`.
3. GREEN: implemented runtime isolation contract + `bollard` conversion:
   - `AgentContainerSpec`
   - `MountSpec`
   - `build_agent_container_spec`
   - `validate_isolation_spec`
   - `to_bollard_config`
4. GREEN verification:
   - `cargo test -p oax-runtime isolation::tests::build_spec_enforces_no_host_network_and_read_only_root -- --exact` passed.
   - `cargo test -p oax-runtime` passed.
5. Broad verification:
   - `cargo test --workspace` passed.
   - `cargo clippy --workspace --all-targets` passed.

## Command Evidence

```bash
cargo test -p oax-runtime isolation::tests::build_spec_enforces_no_host_network_and_read_only_root -- --exact
cargo test -p oax-runtime
cargo test --workspace
cargo clippy --workspace --all-targets
```
