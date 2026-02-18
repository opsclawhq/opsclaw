import React from "react";

interface AgentProfilePanelProps {
  profile: {
    name: string;
    role: string;
    soul_profile: string;
    skills: string[];
    token_budget_remaining: number;
    recent_activity: Array<{ event_id: string; summary: string; occurred_at: string }>;
  } | null;
}

export function AgentProfilePanel({ profile }: AgentProfilePanelProps) {
  if (!profile) {
    return <section className="panel"><h2>Agent Profile</h2><p className="subtle">Select an agent to view details.</p></section>;
  }

  return (
    <section className="panel">
      <h2>{profile.name}</h2>
      <p className="subtle">{profile.role}</p>
      <p>SOUL: <code>{profile.soul_profile}</code></p>
      <p>Token budget remaining: <strong>{profile.token_budget_remaining}</strong></p>
      <h3>Skills</h3>
      <ul>
        {profile.skills.map((skill) => <li key={skill}>{skill}</li>)}
      </ul>
      <h3>Recent Activity</h3>
      <ul>
        {profile.recent_activity.map((item) => (
          <li key={item.event_id}>{item.summary} <span className="subtle">({item.occurred_at})</span></li>
        ))}
      </ul>
    </section>
  );
}
