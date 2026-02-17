use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct CancellationRegistry {
    active_runs: HashSet<String>,
    canceled_runs: HashSet<String>,
}

impl CancellationRegistry {
    pub fn register_run(&mut self, run_id: &str) {
        self.active_runs.insert(run_id.to_string());
    }

    pub fn cancel(&mut self, run_id: &str) -> bool {
        if !self.active_runs.contains(run_id) {
            return false;
        }
        self.canceled_runs.insert(run_id.to_string());
        true
    }

    pub fn is_canceled(&self, run_id: &str) -> bool {
        self.canceled_runs.contains(run_id)
    }

    pub fn should_continue(&self, run_id: &str) -> bool {
        self.active_runs.contains(run_id) && !self.canceled_runs.contains(run_id)
    }

    pub fn complete_run(&mut self, run_id: &str) {
        self.active_runs.remove(run_id);
        self.canceled_runs.remove(run_id);
    }
}

#[cfg(test)]
mod tests {
    use super::CancellationRegistry;

    #[test]
    fn cancel_marks_active_run_as_canceled() {
        let mut registry = CancellationRegistry::default();
        registry.register_run("run-1");
        assert!(registry.should_continue("run-1"));

        assert!(registry.cancel("run-1"));
        assert!(registry.is_canceled("run-1"));
        assert!(!registry.should_continue("run-1"));
    }

    #[test]
    fn cancel_unknown_run_returns_false() {
        let mut registry = CancellationRegistry::default();
        assert!(!registry.cancel("missing-run"));
        assert!(!registry.is_canceled("missing-run"));
    }

    #[test]
    fn complete_run_clears_state() {
        let mut registry = CancellationRegistry::default();
        registry.register_run("run-1");
        registry.cancel("run-1");
        registry.complete_run("run-1");

        assert!(!registry.is_canceled("run-1"));
        assert!(!registry.should_continue("run-1"));
    }
}
