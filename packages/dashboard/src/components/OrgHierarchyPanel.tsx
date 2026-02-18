import React from "react";

interface OrgHierarchyPanelProps {
  hierarchy: {
    root_label: string;
    lead: { name: string; role: string } | null;
    groups: Array<{ role: string; members: Array<{ agent_id: string; name: string; status: string }> }>;
  };
  selectedAgentId: string;
  onSelectAgent: (agentId: string) => void;
}

export function OrgHierarchyPanel({ hierarchy, selectedAgentId, onSelectAgent }: OrgHierarchyPanelProps) {
  return (
    <section className="panel">
      <h2>{hierarchy.root_label}</h2>
      {hierarchy.lead ? <p className="subtle">Lead: {hierarchy.lead.name} ({hierarchy.lead.role})</p> : null}
      {hierarchy.groups.map((group) => (
        <div key={group.role} className="group-block">
          <h3>{group.role}</h3>
          <ul>
            {group.members.map((agent) => (
              <li key={agent.agent_id}>
                <button
                  type="button"
                  className={agent.agent_id === selectedAgentId ? "agent-button active" : "agent-button"}
                  onClick={() => onSelectAgent(agent.agent_id)}
                >
                  <span>{agent.name}</span>
                  <span className="badge">{agent.status}</span>
                </button>
              </li>
            ))}
          </ul>
        </div>
      ))}
    </section>
  );
}
