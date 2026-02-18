# KPI Telemetry Hardening

OpsClaw phase-gate decisions require KPI snapshot evidence. This guide defines repeatable checks so gate status does not drift from actual telemetry quality.

## Scripts

- `scripts/planning/validate-planning-metadata.sh`
  - validates local `.planning/contracts/*.metadata.json`
  - enforces required fields and numeric KPI keys
  - blocks placeholder KPI snapshots by default

- `scripts/planning/audit-project-kpi-snapshots.sh --owner <org> --project <n>`
  - audits GitHub Project items with gate status `passed` or `ready`
  - validates `KPI Snapshot` payload shape and numeric values
  - blocks placeholder KPI snapshots by default

## Modes

Strict mode (default):

```bash
bash scripts/planning/validate-planning-metadata.sh
bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1
```

Temporary permissive mode while real telemetry is unavailable:

```bash
ALLOW_PLACEHOLDER_KPI=1 bash scripts/planning/validate-planning-metadata.sh
ALLOW_PLACEHOLDER_KPI=1 bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1
```

## Recommended Gate Workflow

1. Run strict metadata validation.
2. Run strict project KPI audit.
3. If strict checks fail due to placeholder values, keep gate in `ready` or log explicit temporary exception before setting `passed`.
4. Record resulting command evidence in the plan evidence document.
