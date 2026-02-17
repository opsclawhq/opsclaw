use oax_tools::approval::{plan_command_execution, ApprovalCard, ExecutionDecision};
use oax_tools::risk::{classify_command_risk, RiskClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    pub risk_class: RiskClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpCallDecision {
    AllowReadOnly,
    RequireApproval(ApprovalCard),
    Forbidden(String),
    UnknownTool(String),
}

pub fn opsclaw_mcp_tools() -> Vec<McpToolDefinition> {
    vec![
        McpToolDefinition {
            name: "shell".to_string(),
            description: "Execute shell commands with runtime safety gates.".to_string(),
            risk_class: RiskClass::SafeWrite,
        },
        McpToolDefinition {
            name: "http".to_string(),
            description: "Perform HTTP requests with policy-aware controls.".to_string(),
            risk_class: RiskClass::Read,
        },
        McpToolDefinition {
            name: "file".to_string(),
            description: "Read and write files in scoped workspaces.".to_string(),
            risk_class: RiskClass::SafeWrite,
        },
        McpToolDefinition {
            name: "kubectl".to_string(),
            description: "Kubernetes inspection and operations commands.".to_string(),
            risk_class: RiskClass::Destructive,
        },
        McpToolDefinition {
            name: "git".to_string(),
            description: "Repository inspection and change workflows.".to_string(),
            risk_class: RiskClass::SafeWrite,
        },
    ]
}

pub fn evaluate_mcp_call(
    tool_name: &str,
    command: &str,
    rollback_template: Option<&str>,
) -> McpCallDecision {
    if !opsclaw_mcp_tools()
        .iter()
        .any(|tool| tool.name == tool_name.trim())
    {
        return McpCallDecision::UnknownTool(format!("unknown MCP tool '{}'", tool_name));
    }

    if classify_command_risk(command) == RiskClass::Forbidden {
        return McpCallDecision::Forbidden(
            "forbidden command blocked by safety policy".to_string(),
        );
    }

    match plan_command_execution(command, rollback_template) {
        ExecutionDecision::AllowReadOnly => McpCallDecision::AllowReadOnly,
        ExecutionDecision::RequireApproval(card) => McpCallDecision::RequireApproval(card),
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_mcp_call, opsclaw_mcp_tools, McpCallDecision};

    #[test]
    fn mcp_tool_catalog_contains_expected_builtin_tools() {
        let names: Vec<String> = opsclaw_mcp_tools().into_iter().map(|t| t.name).collect();
        assert!(names.contains(&"shell".to_string()));
        assert!(names.contains(&"http".to_string()));
        assert!(names.contains(&"file".to_string()));
        assert!(names.contains(&"kubectl".to_string()));
        assert!(names.contains(&"git".to_string()));
    }

    #[test]
    fn read_only_mcp_call_is_allowed() {
        let decision = evaluate_mcp_call("shell", "kubectl get pods", None);
        assert_eq!(decision, McpCallDecision::AllowReadOnly);
    }

    #[test]
    fn mutating_mcp_call_requires_approval() {
        let decision = evaluate_mcp_call("shell", "kubectl delete pod api-1", None);
        match decision {
            McpCallDecision::RequireApproval(card) => {
                assert!(card.command.contains("kubectl delete"));
            }
            _ => panic!("mutating command should require approval"),
        }
    }

    #[test]
    fn forbidden_mcp_call_is_rejected() {
        let decision = evaluate_mcp_call("shell", "rm -rf /", None);
        match decision {
            McpCallDecision::Forbidden(reason) => {
                assert!(reason.contains("forbidden"));
            }
            _ => panic!("forbidden command should be rejected"),
        }
    }
}
