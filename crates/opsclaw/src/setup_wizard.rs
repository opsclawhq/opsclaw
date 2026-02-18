use clap::ValueEnum;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Template {
    SreSquad,
    DevOpsTeam,
    IncidentResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct WizardStep {
    pub id: String,
    pub title: String,
    pub estimated_seconds: u16,
    pub required: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct WizardPlan {
    pub template: Template,
    pub template_label: String,
    pub estimated_seconds: u16,
    pub within_60_second_goal: bool,
    pub steps: Vec<WizardStep>,
}

impl Template {
    fn label(&self) -> &'static str {
        match self {
            Template::SreSquad => "SRE Squad",
            Template::DevOpsTeam => "DevOps Team",
            Template::IncidentResponse => "Incident Response",
        }
    }
}

pub fn build_wizard_plan(
    template: Template,
    has_api_key: bool,
    has_slack_workspace: bool,
) -> WizardPlan {
    let mut steps = vec![WizardStep {
        id: "choose_template".to_string(),
        title: format!("Select {} template", template.label()),
        estimated_seconds: 5,
        required: true,
    }];

    steps.push(WizardStep {
        id: "configure_api_key".to_string(),
        title: "Configure LLM API key".to_string(),
        estimated_seconds: if has_api_key { 5 } else { 20 },
        required: true,
    });

    steps.push(WizardStep {
        id: "connect_slack".to_string(),
        title: "Connect Slack workspace".to_string(),
        estimated_seconds: if has_slack_workspace { 5 } else { 20 },
        required: true,
    });

    steps.push(WizardStep {
        id: "deploy_squad".to_string(),
        title: "Deploy agents and open Mission Control".to_string(),
        estimated_seconds: 10,
        required: true,
    });

    steps.push(WizardStep {
        id: "verify_intro".to_string(),
        title: "Verify bot intro messages in Slack".to_string(),
        estimated_seconds: 8,
        required: true,
    });

    let estimated_seconds: u16 = steps.iter().map(|step| step.estimated_seconds).sum();

    WizardPlan {
        template_label: template.label().to_string(),
        template,
        within_60_second_goal: estimated_seconds <= 60,
        estimated_seconds,
        steps,
    }
}

pub fn write_wizard_plan(path: &Path, plan: &WizardPlan) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create wizard output dir {}: {err}", parent.display()))?;
    }

    let serialized =
        serde_json::to_string_pretty(plan).map_err(|err| format!("failed to serialize wizard plan: {err}"))?;

    fs::write(path, serialized)
        .map_err(|err| format!("failed to write wizard plan {}: {err}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_sre_template_plan_is_sub_60_seconds() {
        let plan = build_wizard_plan(Template::SreSquad, true, true);
        assert!(plan.within_60_second_goal);
        assert!(plan.estimated_seconds <= 60);
        assert_eq!(plan.steps.first().map(|step| step.id.as_str()), Some("choose_template"));
    }

    #[test]
    fn missing_setup_inputs_exceed_60_second_target() {
        let plan = build_wizard_plan(Template::IncidentResponse, false, false);
        assert!(!plan.within_60_second_goal);
        assert!(plan.estimated_seconds > 60);
    }

    #[test]
    fn template_label_matches_value() {
        let plan = build_wizard_plan(Template::DevOpsTeam, true, false);
        assert_eq!(plan.template_label, "DevOps Team");
    }
}
