pub fn start_message(template: &str) -> String {
    format!(
        "OpsClaw is live. {}\n\nCommands:\n/start\n/help\n/squad\n/approve\n\nMention the bot in group chats or send a private message to start.",
        squad_message(template)
    )
}

pub fn help_message() -> String {
    "OpsClaw commands:\n/start - intro and quick start\n/help - command reference\n/squad - list active squad members\n/approve - preview inline approval keyboard\n\nIn groups, include @botname in your message.".to_string()
}

pub fn squad_message(template: &str) -> String {
    let members = squad_members(template);
    let mut lines = vec![format!("Active {} squad:", template_label(template))];
    for (name, role) in members {
        lines.push(format!("- {name} ({role})"));
    }
    lines.join("\n")
}

pub fn approval_message() -> String {
    "Approval requested. Choose an action below.".to_string()
}

pub fn callback_ack_message(callback_data: &str) -> String {
    match callback_data {
        "approve" => "Approval recorded: proceeding with the safe execution plan.".to_string(),
        "reject" => "Approval rejected: execution cancelled and escalation requested.".to_string(),
        other => format!("Callback received: `{other}`"),
    }
}

pub fn agent_reply(template: &str, text: &str) -> String {
    let members = squad_members(template);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "I received an empty request. Share the issue and I will route it.".to_string();
    }

    let index = trimmed
        .bytes()
        .fold(0usize, |acc, value| acc.wrapping_add(value as usize))
        % members.len();
    let (name, role) = members[index];

    format!(
        "{name} ({role}) taking point: `{trimmed}`\nNext: assess risk, propose safe action, and report status."
    )
}

fn template_label(template: &str) -> &'static str {
    let normalized = normalized_template(template);
    match normalized.as_str() {
        "dev-ops-team" => "DevOps",
        "incident-response" => "Incident Response",
        _ => "SRE",
    }
}

fn squad_members(template: &str) -> [(&'static str, &'static str); 3] {
    let normalized = normalized_template(template);
    match normalized.as_str() {
        "dev-ops-team" => [
            ("Dax", "Delivery Engineer"),
            ("Piper", "Platform Operator"),
            ("Scout", "Reliability Guard"),
        ],
        "incident-response" => [
            ("Blaze", "Incident Commander"),
            ("Pulse", "Comms Lead"),
            ("Anchor", "Recovery Engineer"),
        ],
        _ => [
            ("Remy", "SRE"),
            ("Ferris", "Platform Engineer"),
            ("Nova", "Incident Commander"),
        ],
    }
}

fn normalized_template(template: &str) -> String {
    template.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squad_message_lists_members() {
        let message = squad_message("sre-squad");
        assert!(message.contains("Remy"));
        assert!(message.contains("Ferris"));
    }

    #[test]
    fn callback_ack_tracks_decision() {
        assert!(callback_ack_message("approve").contains("Approval recorded"));
        assert!(callback_ack_message("reject").contains("Approval rejected"));
    }
}
