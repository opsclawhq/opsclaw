import React, { useMemo, useState } from "react";
import { ActivityFeedPanel } from "./components/ActivityFeedPanel";
import { AgentProfilePanel } from "./components/AgentProfilePanel";
import { OrgHierarchyPanel } from "./components/OrgHierarchyPanel";
import { mockActivities, mockAgents } from "./lib/mock-data.mjs";
import { buildActivityFeed, buildAgentProfile, buildOrgHierarchy } from "./lib/view-models.mjs";
import "./styles.css";

export function App() {
  const [selectedAgentId, setSelectedAgentId] = useState(mockAgents[0]?.agent_id ?? "");

  const hierarchy = useMemo(() => buildOrgHierarchy(mockAgents), []);
  const activityFeed = useMemo(() => buildActivityFeed(mockActivities, 20), []);
  const profile = useMemo(
    () => buildAgentProfile(selectedAgentId, mockAgents, mockActivities),
    [selectedAgentId]
  );

  return (
    <main className="layout">
      <OrgHierarchyPanel
        hierarchy={hierarchy}
        selectedAgentId={selectedAgentId}
        onSelectAgent={setSelectedAgentId}
      />
      <AgentProfilePanel profile={profile} />
      <ActivityFeedPanel items={activityFeed} />
    </main>
  );
}
