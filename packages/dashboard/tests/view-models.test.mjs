import assert from "node:assert/strict";
import test from "node:test";
import { buildActivityFeed, buildAgentProfile, buildOrgHierarchy } from "../src/lib/view-models.mjs";
import { mockActivities, mockAgents } from "../src/lib/mock-data.mjs";

test("buildOrgHierarchy returns deterministic grouped structure", () => {
  const hierarchy = buildOrgHierarchy(mockAgents);

  assert.equal(hierarchy.root_label, "OpsClaw Mission Control");
  assert.equal(hierarchy.lead?.agent_id, "remy");
  assert.equal(hierarchy.groups.length, 3);
  assert.equal(hierarchy.groups[0].role, "Cost Optimizer");
});

test("buildAgentProfile returns profile with recent activity", () => {
  const profile = buildAgentProfile("wren", mockAgents, mockActivities);
  assert.ok(profile);
  assert.equal(profile.agent_id, "wren");
  assert.equal(profile.recent_activity.length, 1);
  assert.equal(profile.recent_activity[0].event_id, "evt-1003");
});

test("buildAgentProfile returns null for unknown agent", () => {
  const profile = buildAgentProfile("missing", mockAgents, mockActivities);
  assert.equal(profile, null);
});

test("buildActivityFeed sorts newest-first and applies limit", () => {
  const feed = buildActivityFeed(mockActivities, 2);
  assert.equal(feed.length, 2);
  assert.equal(feed[0].event_id, "evt-1003");
  assert.equal(feed[1].event_id, "evt-1002");
});
