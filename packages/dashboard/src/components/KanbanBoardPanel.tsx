import React from "react";

interface KanbanBoardPanelProps {
  columns: Array<{
    stage: string;
    tasks: Array<{
      task_id: string;
      title: string;
      assignee_agent_id: string | null;
    }>;
  }>;
}

export function KanbanBoardPanel({ columns }: KanbanBoardPanelProps) {
  return (
    <section className="panel panel-wide">
      <h2>Task Flow</h2>
      <div className="kanban-grid">
        {columns.map((column) => (
          <div key={column.stage} className="kanban-column">
            <h3>{column.stage}</h3>
            <ul>
              {column.tasks.map((task) => (
                <li key={task.task_id} className="kanban-card">
                  <strong>{task.title}</strong>
                  <p className="subtle">{task.assignee_agent_id ? `Owner: ${task.assignee_agent_id}` : "Unassigned"}</p>
                </li>
              ))}
              {column.tasks.length === 0 ? <li className="subtle">No tasks</li> : null}
            </ul>
          </div>
        ))}
      </div>
    </section>
  );
}
