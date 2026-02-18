use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentProfile {
    pub name: String,
    pub role: String,
    pub specialty: String,
    pub personality: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionTurn {
    pub speaker: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionPlan {
    pub assignee: String,
    pub escalation_required: bool,
    pub turns: Vec<DiscussionTurn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlackResponsePayload {
    Inline {
        text: String,
    },
    Snippet {
        preview: String,
        file_name: String,
        content: String,
    },
}

pub fn build_intro_message(profile: &AgentProfile) -> Result<String, String> {
    if profile.name.trim().is_empty() {
        return Err("agent name is required".to_string());
    }
    if profile.role.trim().is_empty() {
        return Err("agent role is required".to_string());
    }
    if profile.personality.trim().is_empty() {
        return Err("agent personality is required".to_string());
    }

    let specialty_clause = if profile.specialty.trim().is_empty() {
        "I will coordinate with the squad to keep systems healthy.".to_string()
    } else {
        format!("My specialty is {}.", profile.specialty.trim())
    };

    Ok(format!(
        "Hi team, I am {} ({}) in OpsClaw. {} I communicate in a {} style.",
        profile.name.trim(),
        profile.role.trim(),
        specialty_clause,
        profile.personality.trim()
    ))
}

pub fn plan_visible_discussion(task: &str, agents: &[AgentProfile]) -> Result<DiscussionPlan, String> {
    if task.trim().is_empty() {
        return Err("task is required".to_string());
    }
    if agents.len() < 2 {
        return Err("at least two agents are required for visible discussion".to_string());
    }

    let task_lc = task.to_lowercase();
    let mut best_index = 0usize;
    let mut best_score = 0usize;

    for (idx, agent) in agents.iter().enumerate() {
        let specialty = agent.specialty.trim().to_lowercase();
        if specialty.is_empty() {
            continue;
        }
        let score = specialty
            .split_whitespace()
            .filter(|token| task_lc.contains(token))
            .count();
        if score > best_score {
            best_score = score;
            best_index = idx;
        }
    }

    let assignee = agents[best_index].name.clone();
    let escalation_required = best_score == 0;
    let mut turns = Vec::with_capacity(agents.len() + 2);

    for agent in agents {
        turns.push(DiscussionTurn {
            speaker: agent.name.clone(),
            message: format!(
                "I am {} ({}) reviewing task: {}",
                agent.name, agent.role, task
            ),
        });
    }

    turns.push(DiscussionTurn {
        speaker: assignee.clone(),
        message: "I will lead execution for this task based on specialty alignment.".to_string(),
    });

    if escalation_required {
        turns.push(DiscussionTurn {
            speaker: assignee.clone(),
            message: "No direct specialty match detected. Escalating to human operator for assignment.".to_string(),
        });
    }

    Ok(DiscussionPlan {
        assignee,
        escalation_required,
        turns,
    })
}

pub fn prepare_response_for_slack(
    text: &str,
    max_chars: usize,
    snippet_name: &str,
) -> Result<SlackResponsePayload, String> {
    if max_chars == 0 {
        return Err("max_chars must be greater than zero".to_string());
    }
    if snippet_name.trim().is_empty() {
        return Err("snippet_name is required".to_string());
    }

    if text.chars().count() <= max_chars {
        return Ok(SlackResponsePayload::Inline {
            text: text.to_string(),
        });
    }

    let suffix = "... [full response attached]";
    let suffix_len = suffix.chars().count();
    let head_len = max_chars.saturating_sub(suffix_len);
    let preview_head: String = text.chars().take(head_len).collect();
    let preview = if head_len == 0 {
        suffix.chars().take(max_chars).collect()
    } else {
        format!("{preview_head}{suffix}")
    };

    Ok(SlackResponsePayload::Snippet {
        preview,
        file_name: snippet_name.trim().to_string(),
        content: text.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_intro_message_for_agent() {
        let profile = AgentProfile {
            name: "Remy".to_string(),
            role: "SRE".to_string(),
            specialty: "kubernetes".to_string(),
            personality: "calm and direct".to_string(),
        };

        let intro = build_intro_message(&profile).expect("intro should build");
        assert!(intro.contains("Remy"));
        assert!(intro.contains("SRE"));
    }

    #[test]
    fn plans_visible_discussion_with_specialist_assignment() {
        let agents = vec![
            AgentProfile {
                name: "Remy".to_string(),
                role: "SRE".to_string(),
                specialty: "kubernetes".to_string(),
                personality: "calm and direct".to_string(),
            },
            AgentProfile {
                name: "Wren".to_string(),
                role: "Cost".to_string(),
                specialty: "cost".to_string(),
                personality: "analytical".to_string(),
            },
        ];

        let plan = plan_visible_discussion("Investigate kubernetes crash loops", &agents)
            .expect("discussion plan should build");
        assert_eq!(plan.assignee, "Remy");
        assert!(!plan.escalation_required);
        assert!(plan.turns.len() >= 3);
    }

    #[test]
    fn escalates_when_no_specialty_matches_task() {
        let agents = vec![
            AgentProfile {
                name: "Remy".to_string(),
                role: "SRE".to_string(),
                specialty: "kubernetes".to_string(),
                personality: "calm and direct".to_string(),
            },
            AgentProfile {
                name: "Wren".to_string(),
                role: "Cost".to_string(),
                specialty: "cost".to_string(),
                personality: "analytical".to_string(),
            },
        ];

        let plan = plan_visible_discussion("Investigate mysterious latency pattern", &agents)
            .expect("discussion plan should build");
        assert!(plan.escalation_required);
    }

    #[test]
    fn over_limit_response_moves_to_snippet() {
        let text = "x".repeat(120);
        let payload = prepare_response_for_slack(text.as_str(), 80, "run-1.txt")
            .expect("response formatting should succeed");

        match payload {
            SlackResponsePayload::Snippet {
                preview,
                file_name,
                content,
            } => {
                assert_eq!(file_name, "run-1.txt");
                assert!(preview.len() <= 80);
                assert_eq!(content, text);
            }
            _ => panic!("expected snippet payload"),
        }
    }

    #[test]
    fn under_limit_response_stays_inline() {
        let payload =
            prepare_response_for_slack("short reply", 80, "run-1.txt").expect("formatting should work");
        assert_eq!(
            payload,
            SlackResponsePayload::Inline {
                text: "short reply".to_string()
            }
        );
    }
}
