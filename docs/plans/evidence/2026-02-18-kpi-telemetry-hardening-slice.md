# Verification Evidence: KPI Telemetry Hardening Slice

## RED (expected failure before implementation)

Command:

```bash
test -f scripts/planning/validate-planning-metadata.sh
```

Result:

- failed before implementation because metadata validator script did not exist.

## GREEN (permissive validation mode)

Commands:

```bash
test -f scripts/planning/validate-planning-metadata.sh
ALLOW_PLACEHOLDER_KPI=1 bash scripts/planning/validate-planning-metadata.sh
ALLOW_PLACEHOLDER_KPI=1 bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1
```

Result:

- metadata validator script exists
- local metadata contract files validate successfully
- project KPI audit validates structure and reports placeholder count while permissive mode is enabled

## Strict-Mode Enforcement Check

Command:

```bash
bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1
```

Result:

- failed as expected with exit code `1` because placeholder KPI snapshots are still present.
- confirms hardening scripts block gate-quality drift once strict mode is enforced.

## Full Verification

Commands:

```bash
bash scripts/docs/validate-release-docs.sh
cargo test --workspace
cargo clippy --workspace --all-targets
```

Result:

- release docs validation passed
- workspace tests passed
- clippy reported no issues
