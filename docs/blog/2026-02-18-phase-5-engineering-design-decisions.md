# Phase 5 Engineering Design Decisions

## Accepted Decisions

- Ship platform integrations contract-first (Slack/Discord/Telegram routing and output contracts) before live transport plumbing.
- Normalize cross-platform routes into a single payload shape (`platform`, `route_kind`, `target_ref`, `text`) to keep downstream execution platform-agnostic.
- Treat docs, changelog, social drafts, and traceability updates as mandatory slice-level DoD.
- Add deterministic scripts for content/blog generation and docs validation rather than relying on manual phase-close checklists.

## Rejected Alternatives

1. Build live channel transports first and backfill tests later.
2. Keep content/blog workflow manual until after all feature slices complete.
3. Maintain only human-readable docs without a structured agent index.

## Tradeoffs

- More up-front process overhead per slice, but lower rework and clearer release evidence.
- Contract-first adapters accelerate deterministic testing, but defer live networking concerns to later iterations.
- Additional scripts increase maintenance surface, but materially reduce phase-close drift and missing artifacts.
