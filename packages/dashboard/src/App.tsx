import React, { useEffect, useMemo, useRef, useState } from "react";
import { ActivityFeedPanel } from "./components/ActivityFeedPanel";
import { ApprovalQueuePanel } from "./components/ApprovalQueuePanel";
import { AgentProfilePanel } from "./components/AgentProfilePanel";
import { KanbanBoardPanel } from "./components/KanbanBoardPanel";
import { OrgHierarchyPanel } from "./components/OrgHierarchyPanel";
import { applyDashboardEvent, createInitialDashboardState, getPendingApprovals } from "./lib/dashboard-state.mjs";
import { mockActivities, mockAgents, mockKanbanTasks } from "./lib/mock-data.mjs";
import { mockStreamEvents } from "./lib/mock-stream-events.mjs";
import { buildActivityFeed, buildAgentProfile, buildKanbanColumns, buildOrgHierarchy } from "./lib/view-models.mjs";
import "./styles.css";

export function App() {
  const [selectedAgentId, setSelectedAgentId] = useState(mockAgents[0]?.agent_id ?? "");
  const [dashboardState, setDashboardState] = useState(() =>
    createInitialDashboardState({
      tasks: mockKanbanTasks,
      activities: mockActivities
    })
  );
  const streamCursorRef = useRef(0);

  useEffect(() => {
    if (mockStreamEvents.length === 0) {
      return undefined;
    }

    const timer = setInterval(() => {
      setDashboardState((current) => {
        const nextEvent = mockStreamEvents[streamCursorRef.current % mockStreamEvents.length];
        streamCursorRef.current += 1;
        return applyDashboardEvent(current, nextEvent);
      });
    }, 5000);

    return () => clearInterval(timer);
  }, []);

  const hierarchy = useMemo(() => buildOrgHierarchy(mockAgents), []);
  const activityFeed = useMemo(() => buildActivityFeed(dashboardState.activities, 20), [dashboardState.activities]);
  const kanbanColumns = useMemo(() => buildKanbanColumns(dashboardState.tasks), [dashboardState.tasks]);
  const pendingApprovals = useMemo(() => getPendingApprovals(dashboardState), [dashboardState]);
  const profile = useMemo(
    () => buildAgentProfile(selectedAgentId, mockAgents, dashboardState.activities),
    [selectedAgentId, dashboardState.activities]
  );

  function handleApprovalDecision(approvalId: string, decision: "approved" | "rejected") {
    setDashboardState((current) =>
      applyDashboardEvent(current, {
        event_type: "approval.decided",
        approval_id: approvalId,
        decision,
        occurred_at: new Date().toISOString()
      })
    );
  }

  return (
    <main className="layout">
      <OrgHierarchyPanel
        hierarchy={hierarchy}
        selectedAgentId={selectedAgentId}
        onSelectAgent={setSelectedAgentId}
      />
      <AgentProfilePanel profile={profile} />
      <ActivityFeedPanel items={activityFeed} />
      <KanbanBoardPanel columns={kanbanColumns} />
      <ApprovalQueuePanel approvals={pendingApprovals} onDecision={handleApprovalDecision} />
    </main>
  );
}
