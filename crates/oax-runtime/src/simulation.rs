use crate::budget::ConversationBudget;
use crate::executor::AgentQueueManager;
use crate::router::parse_agent_tags;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationResult {
    pub processed_messages: usize,
    pub pending_remaining: usize,
    pub budget_exhausted: bool,
}

pub fn simulate_tagged_conversation(
    scripted_turns: &[(&str, String)],
    budget_limit: usize,
) -> SimulationResult {
    if scripted_turns.is_empty() {
        return SimulationResult {
            processed_messages: 0,
            pending_remaining: 0,
            budget_exhausted: false,
        };
    }

    let mut queue = AgentQueueManager::new();
    let mut budget = ConversationBudget::with_limit(budget_limit);
    let mut processed = 0usize;
    let mut turn_idx = 0usize;

    queue.enqueue_message(scripted_turns[0].0, "seed".to_string());

    while queue.pending_count() > 0 && turn_idx < scripted_turns.len() && budget.can_process_next() {
        let (agent, message) = &scripted_turns[turn_idx];

        let Some(_queued_payload) = queue.dequeue_for_agent(agent) else {
            break;
        };

        queue.mark_processed();
        processed += 1;
        budget.record_processed_message();

        for routed in parse_agent_tags(message) {
            queue.enqueue_message(&routed.target, routed.payload);
        }

        turn_idx += 1;
    }

    SimulationResult {
        processed_messages: processed,
        pending_remaining: queue.pending_count(),
        budget_exhausted: !budget.can_process_next() && queue.pending_count() > 0,
    }
}

#[cfg(test)]
mod tests {
    use super::simulate_tagged_conversation;

    #[test]
    fn ping_pong_conversation_reaches_zero_pending() {
        let scripted = vec![
            (
                "agent-a",
                "start [@agent-b: inspect db]".to_string(),
            ),
            ("agent-b", "done [@agent-a: ack]".to_string()),
            ("agent-a", "complete".to_string()),
        ];

        let result = simulate_tagged_conversation(&scripted, 10);
        assert_eq!(result.pending_remaining, 0);
        assert_eq!(result.processed_messages, 3);
        assert!(!result.budget_exhausted);
    }

    #[test]
    fn simulation_stops_when_budget_exhausted() {
        let scripted = vec![
            (
                "agent-a",
                "start [@agent-b: inspect db]".to_string(),
            ),
            ("agent-b", "done [@agent-a: ack]".to_string()),
            ("agent-a", "complete".to_string()),
        ];

        let result = simulate_tagged_conversation(&scripted, 2);
        assert!(result.budget_exhausted);
        assert_eq!(result.processed_messages, 2);
    }
}
