use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscordSlashCommand {
    pub command_name: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordRouteDecision {
    SlashCommand(DiscordSlashCommand),
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscordEmbed {
    pub title: String,
    pub description: String,
    pub color: u32,
}

pub fn route_discord_payload(payload_json: &str) -> Result<DiscordRouteDecision, String> {
    let value: Value = serde_json::from_str(payload_json)
        .map_err(|err| format!("invalid discord payload json: {err}"))?;

    let payload_type = value
        .get("type")
        .and_then(Value::as_u64)
        .ok_or_else(|| "discord payload missing numeric `type`".to_string())?;

    if payload_type != 2 {
        return Ok(DiscordRouteDecision::Ignore);
    }

    let command_name = value
        .get("data")
        .and_then(Value::as_object)
        .and_then(|data| data.get("name"))
        .and_then(Value::as_str)
        .ok_or_else(|| "discord slash payload missing `data.name`".to_string())?
        .to_string();

    let roles = value
        .get("member")
        .and_then(Value::as_object)
        .and_then(|member| member.get("roles"))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(DiscordRouteDecision::SlashCommand(DiscordSlashCommand {
        command_name,
        roles,
    }))
}

pub fn build_embed(title: &str, description: &str) -> Result<DiscordEmbed, String> {
    if title.trim().is_empty() {
        return Err("discord embed title cannot be empty".to_string());
    }

    if description.trim().is_empty() {
        return Err("discord embed description cannot be empty".to_string());
    }

    Ok(DiscordEmbed {
        title: title.to_string(),
        description: description.to_string(),
        color: 0x1f8b4c,
    })
}

pub fn is_role_authorized(required_role: &str, roles: &[String]) -> bool {
    roles.iter().any(|role| role == required_role)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_slash_command_payload() {
        let payload = r#"{"type":2,"data":{"name":"status"},"member":{"roles":["ops","oncall"]}}"#;

        let decision = route_discord_payload(payload).expect("route should parse");

        match decision {
            DiscordRouteDecision::SlashCommand(command) => {
                assert_eq!(command.command_name, "status");
                assert_eq!(command.roles, vec!["ops".to_string(), "oncall".to_string()]);
            }
            DiscordRouteDecision::Ignore => panic!("expected slash command route"),
        }
    }

    #[test]
    fn non_slash_event_is_ignored() {
        let payload = r#"{"type":1}"#;
        let decision = route_discord_payload(payload).expect("route should parse");
        assert_eq!(decision, DiscordRouteDecision::Ignore);
    }

    #[test]
    fn build_embed_payload_has_required_fields() {
        let embed = build_embed("OpsClaw", "Deployment complete").expect("embed should build");
        assert_eq!(embed.title, "OpsClaw");
        assert_eq!(embed.description, "Deployment complete");
        assert_eq!(embed.color, 0x1f8b4c);
    }

    #[test]
    fn role_authorization_requires_match() {
        let roles = vec!["viewer".to_string(), "ops".to_string()];
        assert!(is_role_authorized("ops", &roles));
        assert!(!is_role_authorized("admin", &roles));
    }
}
