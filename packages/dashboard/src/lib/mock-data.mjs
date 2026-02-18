export const mockAgents = [
  {
    agent_id: "remy",
    name: "Remy",
    role: "SRE Lead",
    status: "InProgress",
    soul_profile: "remy-sre",
    skills: ["incident-response", "kubernetes-debug"],
    token_budget_remaining: 18000
  },
  {
    agent_id: "ferris",
    name: "Ferris",
    role: "Deploy Bot",
    status: "Idle",
    soul_profile: "ferris-deploy",
    skills: ["release-checklist", "rollback"],
    token_budget_remaining: 22000
  },
  {
    agent_id: "wren",
    name: "Wren",
    role: "Cost Optimizer",
    status: "WaitingApproval",
    soul_profile: "wren-cost",
    skills: ["cost-audit", "rightsizing"],
    token_budget_remaining: 15000
  }
];

export const mockActivities = [
  {
    event_id: "evt-1003",
    agent_id: "wren",
    event_type: "approval.requested",
    summary: "Requested approval to scale down idle node group",
    occurred_at: "2026-02-18T01:58:00Z"
  },
  {
    event_id: "evt-1002",
    agent_id: "ferris",
    event_type: "deploy.precheck",
    summary: "Validated rollback checklist for service payments",
    occurred_at: "2026-02-18T01:52:00Z"
  },
  {
    event_id: "evt-1001",
    agent_id: "remy",
    event_type: "incident.triage",
    summary: "Linked crash-loop alerts to pod api-1",
    occurred_at: "2026-02-18T01:49:00Z"
  }
];

export const mockKanbanTasks = [
  {
    task_id: "task-1",
    title: "Investigate API latency regression",
    stage: "Inbox",
    assignee_agent_id: null
  },
  {
    task_id: "task-2",
    title: "Validate rollback plan for payments deploy",
    stage: "Assigned",
    assignee_agent_id: "ferris"
  },
  {
    task_id: "task-3",
    title: "Review idle cluster rightsizing proposal",
    stage: "In Progress",
    assignee_agent_id: "wren"
  },
  {
    task_id: "task-4",
    title: "Publish incident timeline summary",
    stage: "Review",
    assignee_agent_id: "remy"
  }
];
