---
name: cost-optimizer
description: Identify cloud spend waste and propose right-sizing or cleanup opportunities.
required_bins: []
risk: DESTRUCTIVE
trust: bundled
rollback_template: Revert optimization changes using previous infra config snapshots.
---

1. Highlight top cost drivers.
2. Suggest high-confidence savings actions.
3. Mark potentially disruptive actions for explicit human approval.
