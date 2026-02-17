# API Reference

## Alert Ingestion Payload Normalization (Phase 1 Preview)

OpsClaw runtime now includes a parser that normalizes webhook payloads from PagerDuty and Prometheus into a shared internal alert model.

### Supported Input Shapes

1. PagerDuty:
- path: `data.incident.id`
- optional: `data.incident.urgency` (defaults to `unknown`)
- optional: `data.incident.title` (defaults to `pagerduty incident`)

2. Prometheus:
- path: `alerts[0].labels.alertname`
- optional: `alerts[0].labels.severity` (falls back to top-level `status`, then `unknown`)
- optional: `alerts[0].annotations.summary` (defaults to `prometheus alert`)

### Runtime Contract

- parser entrypoint: `oax_runtime::alert::parse_alert_payload(&str)`
- return type: `Result<AlertPayload, String>`
- failure mode: unknown/unsupported shapes return an error string

This is the Phase 1 parser contract used by upcoming webhook endpoint wiring.
