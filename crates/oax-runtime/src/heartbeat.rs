use std::collections::HashMap;

#[derive(Debug)]
pub struct HeartbeatRegistry {
    interval_ms: u64,
    allowed_misses: u8,
    agents: HashMap<String, Option<u64>>,
}

impl HeartbeatRegistry {
    pub fn new(interval_ms: u64, allowed_misses: u8) -> Self {
        Self {
            interval_ms,
            allowed_misses,
            agents: HashMap::new(),
        }
    }

    pub fn register_agent(&mut self, agent_id: &str) {
        self.agents.entry(agent_id.to_string()).or_insert(None);
    }

    pub fn record_heartbeat(&mut self, agent_id: &str, ts_ms: u64) -> Result<(), String> {
        let entry = self
            .agents
            .get_mut(agent_id)
            .ok_or_else(|| format!("unknown agent: {agent_id}"))?;
        *entry = Some(ts_ms);
        Ok(())
    }

    pub fn is_alive(&self, agent_id: &str, now_ms: u64) -> bool {
        let Some(Some(last_seen)) = self.agents.get(agent_id) else {
            return false;
        };
        now_ms.saturating_sub(*last_seen) <= self.stale_after_ms()
    }

    pub fn due_agents(&self, now_ms: u64) -> Vec<String> {
        self.agents
            .iter()
            .filter_map(|(agent, last_seen)| match last_seen {
                Some(last_seen) if now_ms.saturating_sub(*last_seen) < self.interval_ms => None,
                _ => Some(agent.clone()),
            })
            .collect()
    }

    fn stale_after_ms(&self) -> u64 {
        self.interval_ms
            .saturating_mul(u64::from(self.allowed_misses).saturating_add(1))
    }
}

#[cfg(test)]
mod tests {
    use super::HeartbeatRegistry;

    #[test]
    fn recording_unregistered_agent_returns_error() {
        let mut registry = HeartbeatRegistry::new(10_000, 1);
        let err = registry
            .record_heartbeat("agent-a", 0)
            .expect_err("unknown agent should fail");
        assert!(err.contains("unknown agent"));
    }

    #[test]
    fn heartbeat_within_window_keeps_agent_alive() {
        let mut registry = HeartbeatRegistry::new(10_000, 1);
        registry.register_agent("agent-a");
        registry
            .record_heartbeat("agent-a", 1_000)
            .expect("heartbeat should record");

        assert!(registry.is_alive("agent-a", 20_500));
        assert!(!registry.is_alive("agent-a", 21_500));
    }

    #[test]
    fn due_agents_reports_missing_beats() {
        let mut registry = HeartbeatRegistry::new(10_000, 1);
        registry.register_agent("agent-a");
        registry.register_agent("agent-b");
        registry
            .record_heartbeat("agent-a", 0)
            .expect("heartbeat should record");

        let mut due = registry.due_agents(10_000);
        due.sort();
        assert_eq!(due, vec!["agent-a".to_string(), "agent-b".to_string()]);
    }
}
