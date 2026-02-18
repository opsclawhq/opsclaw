import React from "react";

interface ActivityFeedPanelProps {
  items: Array<{ event_id: string; event_type: string; summary: string; occurred_at: string }>;
}

export function ActivityFeedPanel({ items }: ActivityFeedPanelProps) {
  return (
    <section className="panel">
      <h2>Activity Feed</h2>
      <ul>
        {items.map((item) => (
          <li key={item.event_id} className="activity-row">
            <div>
              <strong>{item.event_type}</strong>
              <p>{item.summary}</p>
            </div>
            <time className="subtle">{item.occurred_at}</time>
          </li>
        ))}
      </ul>
    </section>
  );
}
