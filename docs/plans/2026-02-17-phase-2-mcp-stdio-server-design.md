# Phase 2 MCP Stdio Server Design

## Scope
Implement a transport endpoint so external MCP clients can interact with OpsClaw over stdio.

## Options Considered
1. Implement full HTTP/WebSocket MCP transport first.
2. Implement stdio MCP transport first (recommended).
3. Defer transport and keep contract-only.

## Selected Approach
Option 2. Stdio is the simplest transport for early integration and aligns with typical MCP client expectations.

## Data Flow
1. Read one JSON line from stdin.
2. Parse request envelope:
- `method = tools/list`
- `method = tools/call`
3. For `tools/list`, return `oax_runtime::mcp::opsclaw_mcp_tools`.
4. For `tools/call`, evaluate via `oax_runtime::mcp::evaluate_mcp_call`.
5. Write JSON response line to stdout.

## Failure Modes
- Invalid JSON input returns structured error response.
- Unknown method returns structured error response.
- Unknown tool/forbidden command returns explicit policy result in response body.

## Test Strategy
- request line for `tools/list` returns tool catalog.
- request line for `tools/call` returns expected allow/approval/forbidden decision shape.
- malformed request returns parse/validation error response.
