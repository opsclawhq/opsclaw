#[derive(Debug, Clone)]
pub struct ConversationBudget {
    max_messages: usize,
    seen_messages: usize,
}

impl ConversationBudget {
    pub fn with_limit(max_messages: usize) -> Self {
        let bounded_limit = if max_messages == 0 { 1 } else { max_messages };
        Self {
            max_messages: bounded_limit,
            seen_messages: 0,
        }
    }

    pub fn default_limit() -> Self {
        Self::with_limit(50)
    }

    pub fn can_process_next(&self) -> bool {
        self.seen_messages < self.max_messages
    }

    pub fn record_processed_message(&mut self) {
        self.seen_messages += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::ConversationBudget;

    #[test]
    fn default_budget_limit_is_50_messages() {
        let mut budget = ConversationBudget::default_limit();
        for _ in 0..50 {
            assert!(budget.can_process_next());
            budget.record_processed_message();
        }
        assert!(!budget.can_process_next());
    }

    #[test]
    fn custom_limit_applies() {
        let mut budget = ConversationBudget::with_limit(2);
        assert!(budget.can_process_next());
        budget.record_processed_message();
        assert!(budget.can_process_next());
        budget.record_processed_message();
        assert!(!budget.can_process_next());
    }
}
