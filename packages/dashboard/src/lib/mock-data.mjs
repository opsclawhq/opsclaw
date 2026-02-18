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

export const mockEconomicsSnapshot = {
  agents: [
    {
      agent_id: "remy",
      tokens_consumed: 12000,
      api_spend_usd: 2.4,
      incidents_handled: 3,
      minutes_saved: 90
    },
    {
      agent_id: "ferris",
      tokens_consumed: 9000,
      api_spend_usd: 1.8,
      incidents_handled: 2,
      minutes_saved: 60
    },
    {
      agent_id: "wren",
      tokens_consumed: 7000,
      api_spend_usd: 1.3,
      incidents_handled: 1,
      minutes_saved: 45
    }
  ]
};

export const mockConversations = [
  {
    conversation_id: "conv-1",
    title: "Payments deploy rollback",
    entries: [
      {
        kind: "message",
        actor: "remy",
        text: "Investigating deploy regression in payments.",
        occurred_at: "2026-02-18T03:00:00Z"
      },
      {
        kind: "tool_call",
        actor: "remy",
        command: "kubectl get pods -n payments",
        output: "api-1 CrashLoopBackOff",
        occurred_at: "2026-02-18T03:01:00Z"
      },
      {
        kind: "message",
        actor: "ferris",
        text: "Rollback checklist validated, requesting approval.",
        occurred_at: "2026-02-18T03:02:00Z"
      }
    ]
  },
  {
    conversation_id: "conv-2",
    title: "Idle node group rightsizing",
    entries: [
      {
        kind: "message",
        actor: "wren",
        text: "Analyzed cluster utilization; preparing cost optimization proposal.",
        occurred_at: "2026-02-18T03:10:00Z"
      },
      {
        kind: "tool_call",
        actor: "wren",
        command: "kubectl top nodes",
        output: "avg cpu 22%, avg mem 31%",
        occurred_at: "2026-02-18T03:11:00Z"
      }
    ]
  }
];
