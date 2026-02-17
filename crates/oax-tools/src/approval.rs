use crate::shell::is_read_only_command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalCard {
    pub command: String,
    pub expected_effect: String,
    pub blast_radius: String,
    pub rollback_steps: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionDecision {
    AllowReadOnly,
    RequireApproval(ApprovalCard),
}

pub fn plan_command_execution(
    command: &str,
    rollback_template: Option<&str>,
) -> ExecutionDecision {
    if is_read_only_command(command) {
        return ExecutionDecision::AllowReadOnly;
    }

    ExecutionDecision::RequireApproval(ApprovalCard {
        command: command.trim().to_string(),
        expected_effect: infer_expected_effect(command),
        blast_radius: infer_blast_radius(command),
        rollback_steps: rollback_template
            .filter(|s| !s.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| default_rollback(command)),
    })
}

fn infer_expected_effect(command: &str) -> String {
    let normalized = command.to_lowercase();
    if normalized.contains("delete") {
        "Deletes or removes resources from the target environment.".to_string()
    } else if normalized.contains("restart") || normalized.contains("rollout") {
        "Restarts or redeploys workloads in the target environment.".to_string()
    } else if normalized.contains("apply") {
        "Applies configuration changes to live infrastructure.".to_string()
    } else {
        "Performs a mutating operation against infrastructure or source control.".to_string()
    }
}

fn infer_blast_radius(command: &str) -> String {
    let normalized = command.to_lowercase();
    if normalized.contains(" -n ") || normalized.contains("--namespace") {
        "Namespace-scoped impact".to_string()
    } else if normalized.contains("all") || normalized.contains("-a") {
        "Potentially broad impact (multiple resources)".to_string()
    } else {
        "Resource-scoped impact (single target assumed)".to_string()
    }
}

fn default_rollback(command: &str) -> String {
    let normalized = command.to_lowercase();
    if normalized.contains("rollout restart") {
        "Use `kubectl rollout undo` for the affected deployment.".to_string()
    } else if normalized.contains("apply") {
        "Re-apply the previous known-good manifest revision.".to_string()
    } else if normalized.contains("delete") {
        "Recreate deleted resources from source manifests/backups.".to_string()
    } else {
        "Document and provide rollback steps before approval.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{plan_command_execution, ExecutionDecision};

    #[test]
    fn read_only_command_is_allowed_without_hitl() {
        let decision = plan_command_execution("kubectl get pods", None);
        assert_eq!(decision, ExecutionDecision::AllowReadOnly);
    }

    #[test]
    fn mutating_command_requires_approval_card() {
        let decision = plan_command_execution("kubectl delete pod api-1", None);
        let ExecutionDecision::RequireApproval(card) = decision else {
            panic!("mutating command should require approval");
        };

        assert_eq!(card.command, "kubectl delete pod api-1".to_string());
        assert!(!card.expected_effect.trim().is_empty());
        assert!(!card.blast_radius.trim().is_empty());
        assert!(!card.rollback_steps.trim().is_empty());
    }

    #[test]
    fn explicit_rollback_template_is_used() {
        let decision = plan_command_execution(
            "kubectl rollout restart deploy/api",
            Some("kubectl rollout undo deploy/api"),
        );
        let ExecutionDecision::RequireApproval(card) = decision else {
            panic!("mutating command should require approval");
        };

        assert_eq!(
            card.rollback_steps,
            "kubectl rollout undo deploy/api".to_string()
        );
    }
}
