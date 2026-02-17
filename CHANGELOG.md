# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0-phase0] - 2026-02-17

### Added

- Rust workspace scaffold with five crates: `oax-core`, `oax-runtime`, `oax-llm`, `oax-security`, and `opsclaw`.
- CI workflow for workspace checks and tests.
- `typeshare` type-generation pipeline and generated TypeScript output at `packages/sdk/src/generated/types.ts`.
- Local development stack via Docker Compose (PostgreSQL, Redis, NATS JetStream with health checks).
- Initial docs skeleton for Getting Started, Concepts, Skills, Architecture, and API Reference.
- Planning governance enhancements, metadata contract, and phase execution templates.

## [0.1.0-phase1-slice1] - 2026-02-17

### Added

- `oax-core` contract modules for `agent`, `model`, `tool`, and `memory`.
- `oax-runtime` modules for message tag routing, conversation budget enforcement, and task state transitions.
- `oax-security` modules for credential placeholder injection and secret leak pattern scanning.
- `oax-tools` crate with initial read-only command policy helper for shell tool gating.
- Runtime tests for routing parsing, budget ceilings, and transition guards.
- Security tests for secret injection and leak detection behavior.
- Tool policy tests for read-only vs mutating command detection.
- Regenerated TypeScript interface output including new core contract types.

## [0.1.0-phase1-slice2] - 2026-02-17

### Added

- `oax-runtime::executor` queue manager for per-agent sequential message handling.
- Pending counter accounting helpers for queued and processed work tracking.
- Runtime unit tests covering per-agent FIFO ordering and pending lifecycle.
