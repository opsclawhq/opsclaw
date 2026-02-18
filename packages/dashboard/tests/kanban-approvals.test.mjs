import assert from "node:assert/strict";
import test from "node:test";
import {
  applyDashboardEvent,
  createInitialDashboardState,
  getPendingApprovals
} from "../src/lib/dashboard-state.mjs";

const initialTasks = [
  { task_id: "task-1", title: "Investigate API latency", stage: "Inbox", assignee_agent_id: null },
  { task_id: "task-2", title: "Validate deploy rollback", stage: "Assigned", assignee_agent_id: "ferris" }
];

const initialActivities = [
  {
    event_id: "evt-0",
    agent_id: "remy",
    event_type: "task.created",
    summary: "Created triage task",
    occurred_at: "2026-02-18T02:00:00Z"
  }
];

test("task.moved event updates kanban stage", () => {
  const state = createInitialDashboardState({ tasks: initialTasks, activities: initialActivities });
  const next = applyDashboardEvent(state, {
    event_type: "task.moved",
    task_id: "task-1",
    to_stage: "In Progress",
    occurred_at: "2026-02-18T02:01:00Z"
  });

  const updated = next.tasks.find((item) => item.task_id === "task-1");
  assert.equal(updated?.stage, "In Progress");
});

test("approval.requested event creates pending approval entry", () => {
  const state = createInitialDashboardState({ tasks: initialTasks, activities: initialActivities });
  const next = applyDashboardEvent(state, {
    event_type: "approval.requested",
    approval_id: "apr-10",
    task_id: "task-2",
    command: "kubectl delete pod api-1",
    blast_radius: "single deployment",
    rollback: "kubectl rollout undo deploy/api",
    occurred_at: "2026-02-18T02:02:00Z"
  });

  const pending = getPendingApprovals(next);
  assert.equal(pending.length, 1);
  assert.equal(pending[0].approval_id, "apr-10");
});

test("approval.decided event removes pending item", () => {
  const state = createInitialDashboardState({ tasks: initialTasks, activities: initialActivities });
  const withRequest = applyDashboardEvent(state, {
    event_type: "approval.requested",
    approval_id: "apr-11",
    task_id: "task-2",
    command: "terraform apply",
    blast_radius: "cluster-wide",
    rollback: "terraform apply saved-plan",
    occurred_at: "2026-02-18T02:03:00Z"
  });

  const next = applyDashboardEvent(withRequest, {
    event_type: "approval.decided",
    approval_id: "apr-11",
    decision: "approved",
    occurred_at: "2026-02-18T02:04:00Z"
  });

  const pending = getPendingApprovals(next);
  assert.equal(pending.length, 0);
});

test("unknown task move does not mutate task list", () => {
  const state = createInitialDashboardState({ tasks: initialTasks, activities: initialActivities });
  const next = applyDashboardEvent(state, {
    event_type: "task.moved",
    task_id: "missing-task",
    to_stage: "Review",
    occurred_at: "2026-02-18T02:05:00Z"
  });

  assert.deepEqual(next.tasks, state.tasks);
});
