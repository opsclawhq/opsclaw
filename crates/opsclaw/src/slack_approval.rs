use oax_tools::approval::{plan_command_execution, ExecutionDecision};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackApprovalCard {
    pub run_id: String,
    pub command: String,
    pub expected_effect: String,
    pub blast_radius: String,
    pub rollback_steps: String,
    pub approve_action_id: String,
    pub reject_action_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalDecision {
    Approve,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackInteractionDecision {
    pub run_id: String,
    pub decision: ApprovalDecision,
}

#[derive(Debug, Deserialize)]
struct SlackInteractionPayload {
    actions: Option<Vec<SlackAction>>,
}

#[derive(Debug, Deserialize)]
struct SlackAction {
    action_id: Option<String>,
}

pub fn build_approval_card(
    run_id: &str,
    command: &str,
    rollback_template: Option<&str>,
) -> Result<SlackApprovalCard, String> {
    if run_id.trim().is_empty() {
        return Err("run_id is required".to_string());
    }
    if command.trim().is_empty() {
        return Err("command is required".to_string());
    }

    let decision = plan_command_execution(command, rollback_template);
    let ExecutionDecision::RequireApproval(card) = decision else {
        return Err("read-only command does not require approval card".to_string());
    };

    Ok(SlackApprovalCard {
        run_id: run_id.trim().to_string(),
        command: card.command,
        expected_effect: card.expected_effect,
        blast_radius: card.blast_radius,
        rollback_steps: card.rollback_steps,
        approve_action_id: format!("opsclaw:approval:{}:approve", run_id.trim()),
        reject_action_id: format!("opsclaw:approval:{}:reject", run_id.trim()),
    })
}

pub fn card_to_block_kit_json(card: &SlackApprovalCard) -> serde_json::Value {
    serde_json::json!({
        "text": "OpsClaw approval required",
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": format!(
                        "*Approval required* for run `{}`\n*Command:* `{}`",
                        card.run_id, card.command
                    )
                }
            },
            {
                "type": "section",
                "fields": [
                    {"type": "mrkdwn", "text": format!("*Expected effect*\n{}", card.expected_effect)},
                    {"type": "mrkdwn", "text": format!("*Blast radius*\n{}", card.blast_radius)},
                    {"type": "mrkdwn", "text": format!("*Rollback*\n{}", card.rollback_steps)}
                ]
            },
            {
                "type": "actions",
                "elements": [
                    {
                        "type": "button",
                        "text": {"type": "plain_text", "text": "Approve"},
                        "style": "primary",
                        "action_id": card.approve_action_id
                    },
                    {
                        "type": "button",
                        "text": {"type": "plain_text", "text": "Reject"},
                        "style": "danger",
                        "action_id": card.reject_action_id
                    }
                ]
            }
        ]
    })
}

pub fn parse_interaction_decision(payload_json: &str) -> Result<SlackInteractionDecision, String> {
    let payload: SlackInteractionPayload = serde_json::from_str(payload_json)
        .map_err(|err| format!("invalid interaction payload json: {err}"))?;

    let action_id = payload
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .and_then(|action| action.action_id.as_ref())
        .ok_or_else(|| "interaction payload missing actions[0].action_id".to_string())?;

    parse_action_id(action_id)
}

fn parse_action_id(action_id: &str) -> Result<SlackInteractionDecision, String> {
    let rest = action_id
        .strip_prefix("opsclaw:approval:")
        .ok_or_else(|| format!("invalid action_id '{action_id}'"))?;

    let (run_id, decision_suffix) = rest
        .rsplit_once(':')
        .ok_or_else(|| format!("invalid action_id '{action_id}'"))?;

    if run_id.trim().is_empty() {
        return Err(format!("invalid action_id '{action_id}'"));
    }

    let decision = match decision_suffix {
        "approve" => ApprovalDecision::Approve,
        "reject" => ApprovalDecision::Reject,
        _ => {
            return Err(format!(
                "action_id '{action_id}' has unsupported decision suffix"
            ));
        }
    };

    Ok(SlackInteractionDecision {
        run_id: run_id.to_string(),
        decision,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_approval_card_for_mutating_command() {
        let card = build_approval_card(
            "run-123",
            "kubectl delete pod api-1 -n prod",
            Some("kubectl apply -f backup.yaml"),
        )
        .expect("card should build");

        assert_eq!(card.run_id, "run-123");
        assert_eq!(card.approve_action_id, "opsclaw:approval:run-123:approve");
        assert_eq!(card.reject_action_id, "opsclaw:approval:run-123:reject");
        assert!(card.rollback_steps.contains("backup"));

        let payload = card_to_block_kit_json(&card);
        assert_eq!(payload["blocks"][2]["type"], "actions");
    }

    #[test]
    fn read_only_command_is_rejected_for_card_generation() {
        let err = build_approval_card("run-123", "kubectl get pods", None)
            .expect_err("read-only command should not need approval card");
        assert!(err.contains("read-only"));
    }

    #[test]
    fn parses_approve_interaction_decision() {
        let payload = r#"{
            "type": "block_actions",
            "actions": [{"action_id": "opsclaw:approval:run-123:approve"}]
        }"#;

        let decision = parse_interaction_decision(payload).expect("decision should parse");
        assert_eq!(decision.run_id, "run-123");
        assert_eq!(decision.decision, ApprovalDecision::Approve);
    }

    #[test]
    fn parses_reject_interaction_decision() {
        let payload = r#"{
            "type": "block_actions",
            "actions": [{"action_id": "opsclaw:approval:run-456:reject"}]
        }"#;

        let decision = parse_interaction_decision(payload).expect("decision should parse");
        assert_eq!(decision.run_id, "run-456");
        assert_eq!(decision.decision, ApprovalDecision::Reject);
    }

    #[test]
    fn rejects_invalid_action_id() {
        let payload = r#"{
            "type": "block_actions",
            "actions": [{"action_id": "bad-action"}]
        }"#;

        let err = parse_interaction_decision(payload).expect_err("payload should fail");
        assert!(err.contains("action_id"));
    }
}
