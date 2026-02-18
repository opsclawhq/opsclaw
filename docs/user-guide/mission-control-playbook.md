# Mission Control Playbook (Phase 4)

This playbook explains how operators should use the Mission Control dashboard shipped in Phase 4.

## Dashboard Surfaces

- Org hierarchy: select an agent and inspect role/status profile.
- Activity feed: monitor newest actions, tool calls, and approvals.
- Task flow: follow work movement across Inbox -> Assigned -> In Progress -> Review -> Done.
- Approval queue: review command, blast radius, and rollback, then approve/reject.
- Economics and ROI: monitor spend, usage, and estimated value.
- Conversation viewer: inspect full transcript, including tool command/output entries.

## Daily Operator Workflow

1. Open Mission Control and check pending approvals first.
2. Review task flow for stalled cards in `In Progress` or `Review`.
3. Open activity feed and confirm newest events match expected operations.
4. Review economics panel for abnormal spend spikes.
5. Use conversation viewer for incident retrospectives and handoffs.

## Approval Handling Guidance

When an approval request appears:

1. Read command and blast radius.
2. Validate rollback path exists and is actionable.
3. Approve only if command scope is bounded and owner is clear.
4. Reject if context is incomplete or rollback is risky.

## Troubleshooting

- No task movement visible:
  - verify activity feed still updates; if not, check dashboard stream source.
- Approval queue empty but Slack shows pending card:
  - compare timestamps and ensure latest events were ingested.
- ROI seems stale:
  - verify economics snapshot refresh source and interval.

## Verification Reference

```bash
npm --prefix packages/dashboard test
cargo test --workspace
cargo clippy --workspace --all-targets
```
