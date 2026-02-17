---
name: log-analyzer
description: Parse and summarize operational logs from stdout, Loki, or CloudWatch exports.
required_bins: []
risk: READ
trust: bundled
---

1. Extract high-signal error clusters.
2. Group by service and time window.
3. Produce concise remediation hints.
