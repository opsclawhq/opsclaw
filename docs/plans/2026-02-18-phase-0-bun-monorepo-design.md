# Phase 0 Bun Monorepo Structure (00-02) Design

## Scope
Complete the missing JavaScript/TypeScript monorepo foundation required by roadmap item 00-02.

1. add root Bun workspace config for `packages/*`
2. ensure all expected package surfaces exist: `@opsclaw/sdk`, `@opsclaw/channels`, `@opsclaw/dashboard`
3. add shared TypeScript base config and per-package tsconfig wiring
4. validate install + typecheck at workspace and package scope

## Requirement Coverage
- INFRA-02

## Options Considered

1. Keep ad-hoc per-package setup without a root workspace.
2. Add Bun workspace at root and normalize package manifests/configs (recommended).
3. Use npm/pnpm monorepo tooling instead of Bun.

## Selected Approach
Option 2. It matches roadmap language directly, minimizes migration risk, and enables consistent package-level toolchain behavior.

## Architecture

- Root workspace:
  - `package.json` with `workspaces: ["packages/*"]`
  - `bunfig.toml` for Bun workspace defaults
  - `tsconfig.base.json` for shared compiler options
- Package normalization:
  - add `packages/sdk/package.json` + `packages/sdk/tsconfig.json`
  - add new `packages/channels/` package with minimal contracts + tests
  - add `packages/dashboard/tsconfig.json` extending shared base
- Verification:
  - Bun install across workspace
  - workspace/package typecheck commands
  - existing Rust workspace checks remain green

## Failure Modes

- workspace exists but packages are not linked through Bun workspaces
- `@opsclaw/channels` missing, leaving roadmap structure incomplete
- divergent tsconfig options across packages causing type drift

## Verification Strategy

- RED: `packages/channels/package.json` does not exist before implementation
- GREEN: Bun workspace install and package typechecks succeed
- full: `cargo test --workspace` and `cargo clippy --workspace --all-targets` still pass
