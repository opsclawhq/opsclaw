---
name: incident-responder
description: Triage PagerDuty/Opsgenie alerts and coordinate first-response diagnostics.
required_bins: [kubectl]
risk: SAFE_WRITE
trust: bundled
---

1. Capture alert context and impacted services.
2. Run non-destructive diagnostics.
3. Draft escalation and mitigation plan for human approval.
