# Phase 2 MCP Integration Design

## Scope
Add a typed runtime MCP contract that:
- publishes a baseline OpsClaw MCP tool catalog
- evaluates MCP-originated tool calls through existing risk/approval policy

## Options Considered
1. Build a full MCP server transport now.
2. Build transport-agnostic MCP contract and validation first (recommended).
3. Defer MCP entirely to post-Slack phase.

## Selected Approach
Implement option 2 so runtime contracts and safety enforcement are in place now; transport listeners can be layered later without changing policy behavior.

## Data Flow
1. `opsclaw_mcp_tools` returns tool definitions with risk metadata.
2. MCP client submits call inputs (`tool_name`, `command`, optional rollback template).
3. `evaluate_mcp_call` delegates to risk classification and approval planner to produce decision.

## Failure Modes
- unknown tool names rejected
- forbidden commands explicitly blocked
- mutating commands return approval-required decisions

## Test Strategy
- catalog includes expected built-in tools
- read-only calls are allowed
- mutating calls require approval
- forbidden commands are rejected
