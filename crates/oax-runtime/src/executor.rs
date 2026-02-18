use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
pub struct AgentQueueManager {
    queues: HashMap<String, VecDeque<String>>,
    pending: usize,
}

impl AgentQueueManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue_message(&mut self, agent: &str, payload: String) {
        self.queues
            .entry(agent.to_string())
            .or_default()
            .push_back(payload);
        self.pending += 1;
    }

    pub fn dequeue_for_agent(&mut self, agent: &str) -> Option<String> {
        self.queues.get_mut(agent).and_then(|q| q.pop_front())
    }

    pub fn mark_processed(&mut self) {
        self.pending = self.pending.saturating_sub(1);
    }

    pub fn pending_count(&self) -> usize {
        self.pending
    }
}

#[cfg(test)]
mod tests {
    use super::AgentQueueManager;

    #[test]
    fn per_agent_queue_is_sequential() {
        let mut manager = AgentQueueManager::new();
        manager.enqueue_message("db-agent", "first".to_string());
        manager.enqueue_message("db-agent", "second".to_string());

        assert_eq!(manager.dequeue_for_agent("db-agent"), Some("first".to_string()));
        assert_eq!(manager.dequeue_for_agent("db-agent"), Some("second".to_string()));
        assert_eq!(manager.dequeue_for_agent("db-agent"), None);
    }

    #[test]
    fn pending_counter_tracks_enqueued_and_processed_work() {
        let mut manager = AgentQueueManager::new();
        manager.enqueue_message("a", "one".to_string());
        manager.enqueue_message("b", "two".to_string());
        assert_eq!(manager.pending_count(), 2);

        manager.mark_processed();
        assert_eq!(manager.pending_count(), 1);
        manager.mark_processed();
        assert_eq!(manager.pending_count(), 0);
    }
}
