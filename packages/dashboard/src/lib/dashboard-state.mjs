const KANBAN_STAGES = ["Inbox", "Assigned", "In Progress", "Review", "Done"];

function toActivityEvent(eventType, summary, occurredAt, agentId = "system") {
  return {
    event_id: `${eventType}:${occurredAt}`,
    agent_id: agentId,
    event_type: eventType,
    summary,
    occurred_at: occurredAt
  };
}

export function createInitialDashboardState({ tasks = [], activities = [], approvals = [] } = {}) {
  return {
    tasks: tasks.map((task) => ({ ...task })),
    activities: activities.map((item) => ({ ...item })),
    approvals: approvals.map((item) => ({ ...item }))
  };
}

function moveTask(tasks, taskId, toStage) {
  if (!KANBAN_STAGES.includes(toStage)) {
    return tasks;
  }

  let touched = false;
  const next = tasks.map((task) => {
    if (task.task_id !== taskId) {
      return task;
    }

    if (task.stage === toStage) {
      return task;
    }

    touched = true;
    return {
      ...task,
      stage: toStage
    };
  });

  return touched ? next : tasks;
}

export function getPendingApprovals(state) {
  return state.approvals.filter((item) => item.status === "pending");
}

export function applyDashboardEvent(state, event) {
  switch (event.event_type) {
    case "task.moved": {
      const movedTasks = moveTask(state.tasks, event.task_id, event.to_stage);
      if (movedTasks === state.tasks) {
        return state;
      }

      const summary = `Moved ${event.task_id} to ${event.to_stage}`;
      return {
        ...state,
        tasks: movedTasks,
        activities: [
          toActivityEvent("task.moved", summary, event.occurred_at, "system"),
          ...state.activities
        ]
      };
    }

    case "approval.requested": {
      const approval = {
        approval_id: event.approval_id,
        task_id: event.task_id,
        command: event.command,
        blast_radius: event.blast_radius,
        rollback: event.rollback,
        status: "pending",
        occurred_at: event.occurred_at
      };

      const summary = `Approval requested for ${event.task_id}`;
      return {
        ...state,
        approvals: [approval, ...state.approvals],
        activities: [
          toActivityEvent("approval.requested", summary, event.occurred_at, "system"),
          ...state.activities
        ]
      };
    }

    case "approval.decided": {
      const approval = state.approvals.find((item) => item.approval_id === event.approval_id);
      if (!approval) {
        return state;
      }

      const nextApprovals = state.approvals.map((item) => {
        if (item.approval_id !== event.approval_id) {
          return item;
        }
        return {
          ...item,
          status: event.decision,
          decided_at: event.occurred_at
        };
      });

      const nextTasks =
        event.decision === "approved"
          ? moveTask(state.tasks, approval.task_id, "Review")
          : state.tasks;

      const summary = `Approval ${event.approval_id} ${event.decision}`;
      return {
        ...state,
        tasks: nextTasks,
        approvals: nextApprovals,
        activities: [
          toActivityEvent("approval.decided", summary, event.occurred_at, "system"),
          ...state.activities
        ]
      };
    }

    default:
      return state;
  }
}

export { KANBAN_STAGES };
