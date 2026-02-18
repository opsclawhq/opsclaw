function compareByRoleThenName(a, b) {
  if (a.role === b.role) {
    return a.name.localeCompare(b.name);
  }
  return a.role.localeCompare(b.role);
}

const STAGE_ORDER = ["Inbox", "Assigned", "In Progress", "Review", "Done"];

export function buildOrgHierarchy(agents) {
  const sorted = [...agents].sort(compareByRoleThenName);
  const byRole = new Map();

  for (const agent of sorted) {
    if (!byRole.has(agent.role)) {
      byRole.set(agent.role, []);
    }
    byRole.get(agent.role).push(agent);
  }

  const groups = [...byRole.entries()].map(([role, members]) => ({
    role,
    members
  }));

  const lead = sorted.find((agent) => agent.role.toLowerCase().includes("lead")) ?? sorted[0] ?? null;

  return {
    root_label: "OpsClaw Mission Control",
    lead,
    groups
  };
}

export function buildAgentProfile(agentId, agents, activities) {
  const agent = agents.find((item) => item.agent_id === agentId);
  if (!agent) {
    return null;
  }

  const recent_activity = activities
    .filter((item) => item.agent_id === agentId)
    .sort((a, b) => new Date(b.occurred_at) - new Date(a.occurred_at))
    .slice(0, 10);

  return {
    ...agent,
    recent_activity
  };
}

export function buildActivityFeed(activities, limit = 50) {
  return [...activities]
    .sort((a, b) => new Date(b.occurred_at) - new Date(a.occurred_at))
    .slice(0, limit);
}

export function buildKanbanColumns(tasks) {
  return STAGE_ORDER.map((stage) => ({
    stage,
    tasks: tasks.filter((task) => task.stage === stage)
  }));
}
