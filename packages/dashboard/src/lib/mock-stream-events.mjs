export const mockStreamEvents = [
  {
    event_type: "task.moved",
    task_id: "task-1",
    to_stage: "Assigned",
    occurred_at: "2026-02-18T02:06:00Z"
  },
  {
    event_type: "approval.requested",
    approval_id: "apr-99",
    task_id: "task-2",
    command: "kubectl cordon node-1",
    blast_radius: "single node",
    rollback: "kubectl uncordon node-1",
    occurred_at: "2026-02-18T02:07:00Z"
  }
];
