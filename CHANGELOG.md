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

## [0.1.0-phase1-slice3] - 2026-02-17

### Added

- `oax-runtime::events` JSONL event journal with append and replay APIs.
- `RuntimeEvent` schema containing `schema_version`, `event_type`, `run_id`, and payload.
- Event journal tests for ordered replay and missing-file behavior.

## [0.1.0-phase1-slice4] - 2026-02-17

### Added

- `oax-runtime::alert` parser for PagerDuty and Prometheus webhook payload shapes.
- `AlertPayload` normalized enum with source-specific fields for incident routing.
- Parser tests for PagerDuty payloads, Prometheus payloads, and unsupported payload rejection.
- Prometheus compatibility behavior for missing `annotations` and severity fallback to top-level `status`.

## [0.1.0-phase1-slice5] - 2026-02-17

### Added

- `oax-runtime::heartbeat` registry for agent liveness tracking.
- Interval-based due-agent detection for heartbeat scheduling.
- Missed-heartbeat tolerance checks via `is_alive`.
- Unit tests for unknown-agent errors, liveness window behavior, and due-agent reporting.

## [0.1.0-phase1-slice6] - 2026-02-17

### Added

- `oax-runtime::memory_store` JSON-backed persistent memory implementation.
- `JsonFileMemoryStore` API with `new`, `put`, `get`, and `save` methods.
- Persistence tests for missing-file bootstrap, overwrite behavior, and reload survival.

## [0.1.0-phase1-slice7] - 2026-02-17

### Added

- `oax-tools::approval` HITL planning module for mutating command gating.
- `ExecutionDecision` enum and `ApprovalCard` struct with command/effect/blast-radius/rollback fields.
- Approval planner tests covering read-only bypass, approval-card generation, and explicit rollback template usage.

## [0.1.0-phase1-slice8] - 2026-02-17

### Added

- `oax-tools::risk` command risk classification module.
- `RiskClass` enum (`Read`, `SafeWrite`, `Destructive`, `Forbidden`).
- Risk classifier tests for read-only, safe-write, destructive, and forbidden command cases.

## [0.1.0-phase1-slice9] - 2026-02-17

### Added

- `oax-runtime::cancellation` module for run-level cancellation control.
- `CancellationRegistry` with `register_run`, `cancel`, `is_canceled`, `should_continue`, and `complete_run`.
- Cancellation tests for active-run cancellation, unknown-run handling, and state cleanup.

## [0.1.0-phase1-slice10] - 2026-02-17

### Added

- `oax-runtime::simulation` tagged ping-pong conversation harness.
- `simulate_tagged_conversation` result contract with processed count, pending remainder, and budget-exhausted flag.
- Simulation tests for clean pending drain and budget-stop behavior.

## [0.1.0-phase1-slice11] - 2026-02-17

### Added

- `oax-runtime::tool_boundary` module for integrated command-boundary safety checks.
- `prepare_tool_call` for credential injection plus risk/approval decision planning.
- `filter_tool_output_for_llm` for secret-pattern leak blocking before LLM context ingress.
- Boundary tests for missing secrets, approval requirement, and leak-block behavior.

## [0.1.0-phase2-slice1] - 2026-02-17

### Added

- New `oax-skills` workspace crate for markdown skill parsing and policy validation.
- Typed `SkillFrontmatter` + `SkillRiskClass` contract (`READ`, `SAFE_WRITE`, `DESTRUCTIVE`, `FORBIDDEN`).
- `parse_skill_markdown`, `validate_required_bins`, and `validate_install_policy` APIs.
- Tests for frontmatter parsing, required-bin validation, and destructive/trust policy enforcement.
