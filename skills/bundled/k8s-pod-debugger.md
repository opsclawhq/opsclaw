---
name: k8s-pod-debugger
description: Diagnose common Kubernetes pod failures (CrashLoopBackOff, OOMKilled, ImagePullBackOff).
required_bins: [kubectl]
risk: READ
trust: bundled
---

1. Inspect pod status and recent events.
2. Summarize probable root cause.
3. Recommend safe next diagnostics.
