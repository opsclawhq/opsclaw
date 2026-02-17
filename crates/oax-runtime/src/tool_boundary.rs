use oax_security::injector::CredentialInjector;
use oax_security::leak::LeakDetector;
use oax_tools::approval::{plan_command_execution, ApprovalCard, ExecutionDecision};
use oax_tools::risk::{classify_command_risk, RiskClass};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolBoundaryDecision {
    AllowReadOnly,
    RequireApproval(ApprovalCard),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedToolCall {
    pub rendered_command: String,
    pub risk_class: RiskClass,
    pub decision: ToolBoundaryDecision,
}

pub fn prepare_tool_call(
    command_template: &str,
    secrets: &HashMap<String, String>,
    rollback_template: Option<&str>,
) -> Result<PreparedToolCall, Vec<String>> {
    let injector = CredentialInjector::new(secrets.clone());
    let rendered_command = injector.inject(command_template)?;
    let risk_class = classify_command_risk(&rendered_command);
    let decision = match plan_command_execution(&rendered_command, rollback_template) {
        ExecutionDecision::AllowReadOnly => ToolBoundaryDecision::AllowReadOnly,
        ExecutionDecision::RequireApproval(card) => ToolBoundaryDecision::RequireApproval(card),
    };

    Ok(PreparedToolCall {
        rendered_command,
        risk_class,
        decision,
    })
}

pub fn filter_tool_output_for_llm(output: &str, leak_patterns: &[&str]) -> Result<String, Vec<String>> {
    let detector = LeakDetector::new(leak_patterns);
    let warnings = detector.scan(output);
    if warnings.is_empty() {
        Ok(output.to_string())
    } else {
        Err(warnings.into_iter().map(|w| w.needle).collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{filter_tool_output_for_llm, prepare_tool_call, ToolBoundaryDecision};

    #[test]
    fn injects_credentials_at_tool_boundary() {
        let mut secrets = HashMap::new();
        secrets.insert("GITHUB_TOKEN".to_string(), "ghp_secret".to_string());

        let prepared = prepare_tool_call(
            "curl -H 'Authorization: Bearer ${GITHUB_TOKEN}' https://api.github.com/user",
            &secrets,
            None,
        )
        .expect("command should prepare");

        assert!(prepared.rendered_command.contains("ghp_secret"));
    }

    #[test]
    fn missing_credentials_are_rejected() {
        let secrets = HashMap::new();
        let err = prepare_tool_call("echo ${MISSING_TOKEN}", &secrets, None)
            .expect_err("missing placeholders should fail");
        assert_eq!(err, vec!["MISSING_TOKEN".to_string()]);
    }

    #[test]
    fn mutating_commands_require_approval() {
        let prepared = prepare_tool_call("kubectl delete pod api-1", &HashMap::new(), None)
            .expect("command should prepare");
        assert!(matches!(
            prepared.decision,
            ToolBoundaryDecision::RequireApproval(_)
        ));
    }

    #[test]
    fn leak_output_is_blocked_before_llm_context() {
        let err = filter_tool_output_for_llm("token=ghp_12345", &["ghp_", "AKIA"])
            .expect_err("secret-like output must be blocked");
        assert_eq!(err, vec!["ghp_".to_string()]);
    }

    #[test]
    fn clean_output_passes_to_llm_context() {
        let out =
            filter_tool_output_for_llm("pods are healthy", &["ghp_", "AKIA"]).expect("clean out");
        assert_eq!(out, "pods are healthy".to_string());
    }
}
