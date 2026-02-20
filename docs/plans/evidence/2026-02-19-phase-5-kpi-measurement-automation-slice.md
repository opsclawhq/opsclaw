# Evidence: Phase 5 KPI Measurement Automation (05-21)

## RED -> GREEN

### RED precheck

Command:
```bash
test -f scripts/planning/compute-kpi-snapshot.sh
```

Result:
- failed before implementation because the script did not exist.

### GREEN implementation

Commands:
```bash
test -f scripts/planning/compute-kpi-snapshot.sh
bash scripts/planning/compute-kpi-snapshot.sh --owner opsclawhq --repo opsclaw --project 1 --window-days 30 --output /Users/gshah/work/opsflow-sh/opsclaw/.planning/contracts/kpi-snapshot.latest.json
bash scripts/planning/apply-kpi-snapshot.sh --snapshot /Users/gshah/work/opsflow-sh/opsclaw/.planning/contracts/kpi-snapshot.latest.json --contracts-dir /Users/gshah/work/opsflow-sh/opsclaw/.planning/contracts --owner opsclawhq --project 1
```

Result:
- measured snapshot computed:
  - `{"activation":58,"ttfv_minutes":2.03,"reliability_success_rate":100,"retention_d30":0,"enterprise_pilots":0}`
- metadata files updated: `48`
- project items updated: `57`

## Strict Verification (No Placeholder Override)

Commands:
```bash
bash scripts/planning/validate-planning-metadata.sh
bash scripts/planning/audit-project-kpi-snapshots.sh --owner opsclawhq --project 1
```

Result:
- metadata validation: `files=48 valid=48 errors=0 placeholders=0`
- project audit: `total_items=57 audited=57 errors=0 placeholders=0`

## Full Verification Gate

Commands:
```bash
bash scripts/docs/validate-release-docs.sh
cargo test --workspace
cargo clippy --workspace --all-targets
```

Result:
- release docs validation passed
- workspace tests passed (`171 passed`)
- clippy passed (no issues)
