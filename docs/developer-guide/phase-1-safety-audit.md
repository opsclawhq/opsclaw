# Phase 1 Safety Integration Audit

This audit consolidates Phase 1 safety/runtime verification after merged slices and before phase gate decisions.

## Scope

- Safety invariants (read-only defaults, HITL primitives, leak detection, credential boundary)
- Coordination invariants (routing, queueing, pending-counter, budget ceiling)
- Runtime persistence/audit primitives (event journal replay, heartbeat/alert/persistence slices)

## Integration Verification Commands

```bash
cargo test -p oax-runtime simulation::tests::ping_pong_conversation_reaches_zero_pending -- --exact
cargo test -p oax-runtime simulation::tests::simulation_stops_when_budget_exhausted -- --exact
cargo test -p oax-runtime executor::tests::pending_counter_tracks_enqueued_and_processed_work -- --exact
cargo test -p oax-runtime tool_boundary::tests::injects_credentials_at_tool_boundary -- --exact
cargo test -p oax-runtime tool_boundary::tests::leak_output_is_blocked_before_llm_context -- --exact
cargo test -p oax-runtime events::tests::appends_and_reads_events_in_order -- --exact
cargo test -p oax-runtime events::tests::returns_empty_when_journal_file_missing -- --exact
cargo test -p oax-security injector::tests::injects_secret_placeholders -- --exact
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Success-Criteria Mapping

1. Budget and loop-stop behavior: `simulation_stops_when_budget_exhausted`.
2. Ping-pong routing and pending counter convergence: `ping_pong_conversation_reaches_zero_pending` and pending-counter unit test.
3. Tool boundary secret handling: credential injection + leak-blocking tests.
4. Event journal replay with schema fields: append/read journal tests.
5. Phase-level regression guard: workspace test + clippy passes.

## Notes

- The criterion "docker exec into running agent container returns no creds" is represented by boundary tests in this phase and reinforced by container isolation policy checks added later in Phase 2.
- Use this document with per-slice evidence files under `docs/plans/evidence/` for gate review.
