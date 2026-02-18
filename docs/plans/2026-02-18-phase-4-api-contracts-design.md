# Phase 4 API Contracts-First Design

## Scope
Implement roadmap plan `04-01` by defining Mission Control REST and WebSocket contracts before backend implementation.

Deliverables:
1. OpenAPI contract for core dashboard endpoints
2. typed Rust/TypeScript contract models for dashboard payloads
3. verification evidence for schema/type stability

## Requirement Coverage
- DASH-01
- DASH-02
- DASH-03
- DASH-08

## Options Considered

1. Implement backend handlers first and infer API shape from code.
2. OpenAPI + typed shared models first (recommended).
3. Dashboard frontend mock data only, defer formal API contracts.

## Selected Approach
Option 2. Contract-first is the explicit requirement for DASH-08 and reduces integration churn between backend and dashboard clients.

## Contract Shape

REST paths (v1 draft):

- `GET /api/v1/agents`
- `GET /api/v1/kanban`
- `GET /api/v1/activity`
- `GET /api/v1/approvals`

WebSocket path:

- `GET /api/v1/stream` (upgrade)

Event envelope:

- `event_type`
- `occurred_at`
- `payload_json`

## Failure Modes

- OpenAPI and Rust shared types diverge -> client/server mismatch
- missing enum/state contracts for kanban or agent status -> UI logic ambiguity
- websocket event shape instability -> brittle real-time UI updates

## Verification Strategy

- RED: missing OpenAPI file and missing dashboard contract symbols
- GREEN: contract file exists and round-trip tests pass
- full verification: workspace tests, clippy, typeshare generation
