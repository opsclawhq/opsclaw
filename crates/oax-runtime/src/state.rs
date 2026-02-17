#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentTaskState {
    Pending,
    Running,
    WaitingForApproval,
    Completed,
    Failed,
    Stuck,
}

impl AgentTaskState {
    pub fn can_transition_to(&self, next: &AgentTaskState) -> bool {
        use AgentTaskState::*;
        matches!(
            (self, next),
            (Pending, Running)
                | (Running, WaitingForApproval)
                | (Running, Completed)
                | (Running, Failed)
                | (Running, Stuck)
                | (WaitingForApproval, Running)
                | (WaitingForApproval, Failed)
                | (Stuck, Running)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AgentTaskState;

    #[test]
    fn allows_expected_state_transitions() {
        assert!(AgentTaskState::Pending.can_transition_to(&AgentTaskState::Running));
        assert!(AgentTaskState::Running.can_transition_to(&AgentTaskState::WaitingForApproval));
        assert!(AgentTaskState::WaitingForApproval.can_transition_to(&AgentTaskState::Running));
    }

    #[test]
    fn rejects_invalid_state_transitions() {
        assert!(!AgentTaskState::Completed.can_transition_to(&AgentTaskState::Running));
        assert!(!AgentTaskState::Failed.can_transition_to(&AgentTaskState::Pending));
    }
}
