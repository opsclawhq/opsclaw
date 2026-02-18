# Phase 3 Slack Adapter Foundation Design

## Scope
Implement a deterministic Slack adapter foundation that covers:
- OAuth install URL generation
- @mention routing from Slack Events payloads
- thread-reply behavior (`thread_ts` handling)
- rate-limit handling policy for 429 responses

This slice focuses on contracts and logic, not live Slack network IO.

## Options Considered

1. Integrate `@slack/bolt` runtime immediately.
2. Build a pure adapter contract first (recommended).
3. Defer Slack logic until all channel adapters are built.

## Selected Approach
Option 2. A pure contract module enables testable behavior for routing/threading/rate-limit policy before coupling to external APIs.

## Data Flow

1. Slack event payload JSON enters adapter.
2. Adapter classifies event (`url_verification`, `app_mention`, `message`, ignore).
3. If bot mention exists, adapter emits a route target with:
   - channel
   - reply `thread_ts` (`event.thread_ts` else `event.ts`)
   - cleaned message text (bot mention removed)
4. Delivery policy inspects response status + retry header and emits retry/drop guidance.

## Failure Modes

- malformed payload JSON -> parse error.
- missing required event fields (`channel`, `ts`) -> parse error.
- payload without bot mention -> `Ignore` route decision.
- invalid `Retry-After` header -> no retry duration returned.

## Test Strategy

- OAuth URL generation includes client ID, scopes, redirect, state.
- mention in thread preserves original `thread_ts`.
- mention in channel uses message `ts` as `thread_ts`.
- non-mention message ignored.
- 429 + Retry-After returns retry delay.
- non-429 returns no retry.
