# Verification Evidence: Phase 4 API Contracts-First Slice

## RED (expected failing state)

1. Missing OpenAPI file check:

```bash
test -f docs/api/mission-control-openapi.yaml
```

Result:

- exit code `1` before contract creation.

2. Missing dashboard contract symbols:

```bash
cargo test -p oax-core types::tests::dashboard_stream_event_roundtrip_json -- --exact
```

Result:

- compile failures before implementation:
  - `cannot find struct DashboardStreamEvent`
  - `use of undeclared type DashboardEventType`

## GREEN (target contract test)

Command:

```bash
cargo test -p oax-core types::tests::dashboard_stream_event_roundtrip_json -- --exact
```

Result:

- `1 passed; 0 failed`

OpenAPI presence check:

```bash
test -f docs/api/mission-control-openapi.yaml
```

Result:

- pass (`openapi_present`).

## Full Verification

Commands:

```bash
cargo test -p oax-core
cargo test --workspace
cargo clippy --workspace --all-targets
bash scripts/generate-types.sh
```

Result:

- all tests passed
- clippy exited `0`
- TypeScript contracts regenerated successfully from `oax-core` types
