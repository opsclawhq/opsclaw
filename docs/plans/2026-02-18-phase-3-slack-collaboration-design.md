# Phase 3 Slack Collaboration Contracts Design

## Scope
Implement Phase 3 plan `03-04` as deterministic contracts and CLI helpers, covering:
1. bot introduction message generation
2. visible multi-agent discussion planning in-channel
3. long-response overflow handling into snippet payload + preview

This slice intentionally excludes live Slack network/webhook integration.

## Requirement Coverage
- BOT-06
- CHAT-07
- COORD-06
- COORD-07

## Options Considered

1. Full `@slack/bolt` runtime integration now.
2. Contract-first Rust module + CLI verification helpers (recommended).
3. Delay visible discussion/overflow handling until final launch packaging.

## Selected Approach
Option 2. Keep behavior deterministic and testable in isolation before adding network concerns.

## Data Flow

1. Operator supplies agent profiles and task context.
2. Intro builder generates channel-safe self-intro text per bot.
3. Discussion planner selects specialist by task/specialty match, emits visible turn sequence, and marks escalation if no confident match.
4. Response formatter checks Slack char limit and either:
   - emits inline text, or
   - emits snippet payload with preview + full content.

## Failure Modes

- Missing required profile fields -> validation error.
- Discussion called with fewer than 2 agents -> validation error.
- Invalid/empty task text -> validation error.
- Invalid response limit (< 1) -> validation error.

## Test Strategy

- Intro message includes agent name + role + personality statement.
- Discussion planner produces multiple visible turns and specialist assignment.
- No-specialty-match task triggers escalation marker.
- Overflow response returns snippet payload with preview and full content preserved.
- Under-limit response remains inline.
