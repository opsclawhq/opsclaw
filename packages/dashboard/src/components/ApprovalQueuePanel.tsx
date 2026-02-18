import React from "react";

interface ApprovalQueuePanelProps {
  approvals: Array<{
    approval_id: string;
    task_id: string;
    command: string;
    blast_radius: string;
    rollback: string;
    status: string;
    occurred_at: string;
  }>;
  onDecision: (approvalId: string, decision: "approved" | "rejected") => void;
}

export function ApprovalQueuePanel({ approvals, onDecision }: ApprovalQueuePanelProps) {
  return (
    <section className="panel panel-wide">
      <h2>Approval Queue</h2>
      {approvals.length === 0 ? <p className="subtle">No pending approvals.</p> : null}
      <ul>
        {approvals.map((item) => (
          <li key={item.approval_id} className="approval-card">
            <strong>{item.task_id}</strong>
            <p><code>{item.command}</code></p>
            <p className="subtle">Blast radius: {item.blast_radius}</p>
            <p className="subtle">Rollback: {item.rollback}</p>
            <p className="subtle">Requested at: {item.occurred_at}</p>
            <div className="approval-actions">
              <button type="button" className="decision-button approve" onClick={() => onDecision(item.approval_id, "approved")}>Approve</button>
              <button type="button" className="decision-button reject" onClick={() => onDecision(item.approval_id, "rejected")}>Reject</button>
            </div>
          </li>
        ))}
      </ul>
    </section>
  );
}
