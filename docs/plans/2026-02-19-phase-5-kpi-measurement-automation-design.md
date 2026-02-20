# Phase 5 KPI Measurement Automation (05-21) Design

## Context

Current KPI snapshots are placeholder zeros across planning contracts and GitHub Project items. Strict KPI audits fail unless `ALLOW_PLACEHOLDER_KPI=1` is enabled.

## Goal

Replace manual placeholder KPI handling with measurable, reproducible KPI snapshot automation sourced from current delivery telemetry.

## Requirement IDs

- ROADMAP-GATE-KPI
- PUB-07

## Approach

Add two planning scripts:

1. `compute-kpi-snapshot.sh`
- computes a numeric KPI snapshot from GitHub telemetry over a configurable time window.
- output schema matches `.planning/contracts/planning-metadata.schema.json` KPI keys.

2. `apply-kpi-snapshot.sh`
- applies a computed snapshot to local `.planning/contracts/*.metadata.json`.
- optionally updates GitHub Project `KPI Snapshot` field for `passed|ready` items.

## KPI Definitions (automation scope)

- `activation`: merged PR count in lookback window.
- `ttfv_minutes`: median PR lead time (`mergedAt - createdAt`) in minutes.
- `reliability_success_rate`: ratio of `passed` gate items to all gate-tracked items (`passed|ready|blocked`).
- `retention_d30`: contributor retention ratio across first/second halves of lookback window.
- `enterprise_pilots`: merged PR count with `enterprise` or `pilot` tokens in title/body.

These are measured operational KPIs for delivery governance, not end-user product adoption metrics.

## Validation

- strict local metadata validation should pass without `ALLOW_PLACEHOLDER_KPI`.
- strict project KPI audit should pass without `ALLOW_PLACEHOLDER_KPI`.

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
