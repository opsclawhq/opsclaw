import assert from "node:assert/strict";
import test from "node:test";
import {
  buildConversationTranscript,
  buildEconomicsRows,
  buildRoiSummary
} from "../src/lib/economics-conversation.mjs";

const economicsSnapshot = {
  agents: [
    { agent_id: "remy", tokens_consumed: 12000, api_spend_usd: 2.4, incidents_handled: 3, minutes_saved: 90 },
    { agent_id: "ferris", tokens_consumed: 9000, api_spend_usd: 1.8, incidents_handled: 2, minutes_saved: 60 }
  ]
};

const conversations = [
  {
    conversation_id: "conv-1",
    title: "Payments deploy rollback",
    entries: [
      { kind: "message", actor: "remy", text: "Investigating deploy regression", occurred_at: "2026-02-18T03:00:00Z" },
      {
        kind: "tool_call",
        actor: "remy",
        command: "kubectl get pods -n payments",
        output: "api-1 CrashLoopBackOff",
        occurred_at: "2026-02-18T03:01:00Z"
      }
    ]
  }
];

test("buildEconomicsRows computes total spend and cost-per-agent rows", () => {
  const rows = buildEconomicsRows(economicsSnapshot);
  assert.equal(rows.length, 2);
  assert.equal(rows[0].agent_id, "remy");
  assert.equal(rows[0].api_spend_usd, 2.4);
});

test("buildRoiSummary computes ratio and rounded totals", () => {
  const summary = buildRoiSummary(economicsSnapshot, 180);
  assert.equal(summary.total_spend_usd, 4.2);
  assert.equal(summary.total_minutes_saved, 150);
  assert.equal(summary.estimated_value_usd, 450);
  assert.equal(summary.roi_ratio, 107.14);
});

test("buildConversationTranscript returns sorted transcript entries", () => {
  const transcript = buildConversationTranscript("conv-1", conversations);
  assert.equal(transcript.conversation_id, "conv-1");
  assert.equal(transcript.entries.length, 2);
  assert.equal(transcript.entries[1].kind, "tool_call");
});

test("buildConversationTranscript returns null for unknown conversation", () => {
  const transcript = buildConversationTranscript("missing", conversations);
  assert.equal(transcript, null);
});
