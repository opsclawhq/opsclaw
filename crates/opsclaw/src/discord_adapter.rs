use crate::squad_responder::response_for_input;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::env;

const DISCORD_CHANNEL_MESSAGE_URL_PREFIX: &str = "https://discord.com/api/v10/channels";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscordSlashCommand {
    pub command_name: String,
    pub roles: Vec<String>,
    pub channel_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordRouteDecision {
    SlashCommand(DiscordSlashCommand),
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum DiscordLiveDecision {
    Posted { channel_id: String, text: String },
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscordOutgoingMessage {
    pub channel_id: String,
    pub text: String,
}

pub trait DiscordApi {
    fn post_channel_message(&mut self, message: DiscordOutgoingMessage) -> Result<(), String>;
}

pub struct HttpDiscordApi {
    bot_token: String,
    client: ureq::Agent,
}

impl HttpDiscordApi {
    pub fn new(bot_token: String) -> Result<Self, String> {
        if bot_token.trim().is_empty() {
            return Err("discord bot token cannot be empty".to_string());
        }

        Ok(Self {
            bot_token,
            client: ureq::AgentBuilder::new().build(),
        })
    }
}

impl DiscordApi for HttpDiscordApi {
    fn post_channel_message(&mut self, message: DiscordOutgoingMessage) -> Result<(), String> {
        let endpoint = format!(
            "{}/{}/messages",
            DISCORD_CHANNEL_MESSAGE_URL_PREFIX, message.channel_id
        );
        let payload = serde_json::json!({ "content": message.text });

        let response = self
            .client
            .post(endpoint.as_str())
            .set("Authorization", format!("Bot {}", self.bot_token).as_str())
            .set("Content-Type", "application/json")
            .send_json(payload)
            .map_err(|err| format!("discord channel message request failed: {err}"))?;

        let parsed: Value = response
            .into_json()
            .map_err(|err| format!("discord channel message response parse failed: {err}"))?;

        if parsed.get("id").and_then(Value::as_str).is_none() {
            let message = parsed
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown_error");
            return Err(format!("discord channel message returned non-ok payload: {message}"));
        }

        Ok(())
    }
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

    let channel_id = value
        .get("channel_id")
        .and_then(Value::as_str)
        .map(ToString::to_string);

    Ok(DiscordRouteDecision::SlashCommand(DiscordSlashCommand {
        command_name,
        roles,
        channel_id,
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

pub fn resolve_bot_token(
    explicit_token: Option<&str>,
    token_env_var: Option<&str>,
    default_env_var: &str,
) -> Result<String, String> {
    if let Some(token) = explicit_token.map(str::trim).filter(|value| !value.is_empty()) {
        return Ok(token.to_string());
    }

    let env_var_name = token_env_var
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(default_env_var);

    let token = env::var(env_var_name).map_err(|_| {
        format!("discord bot token not provided; set `{env_var_name}` or pass --bot-token")
    })?;

    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "discord bot token env var `{env_var_name}` is set but empty"
        ));
    }

    Ok(trimmed.to_string())
}

pub fn handle_live_event(
    api: &mut dyn DiscordApi,
    payload_json: &str,
    template: &str,
) -> Result<DiscordLiveDecision, String> {
    let route = route_discord_payload(payload_json)?;

    match route {
        DiscordRouteDecision::Ignore => Ok(DiscordLiveDecision::Ignore),
        DiscordRouteDecision::SlashCommand(command) => {
            let channel_id = command
                .channel_id
                .ok_or_else(|| "discord live-event requires `channel_id` in payload".to_string())?;
            let text = response_for_input(template, command.command_name.as_str());
            api.post_channel_message(DiscordOutgoingMessage {
                channel_id: channel_id.clone(),
                text: text.clone(),
            })?;
            Ok(DiscordLiveDecision::Posted { channel_id, text })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockDiscordApi {
        sent: Vec<DiscordOutgoingMessage>,
    }

    impl DiscordApi for MockDiscordApi {
        fn post_channel_message(&mut self, message: DiscordOutgoingMessage) -> Result<(), String> {
            self.sent.push(message);
            Ok(())
        }
    }

    #[test]
    fn routes_slash_command_payload() {
        let payload =
            r#"{"type":2,"channel_id":"777","data":{"name":"status"},"member":{"roles":["ops","oncall"]}}"#;

        let decision = route_discord_payload(payload).expect("route should parse");

        match decision {
            DiscordRouteDecision::SlashCommand(command) => {
                assert_eq!(command.command_name, "status");
                assert_eq!(command.roles, vec!["ops".to_string(), "oncall".to_string()]);
                assert_eq!(command.channel_id.as_deref(), Some("777"));
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

    #[test]
    fn live_event_posts_channel_message() {
        let payload = r#"{"type":2,"channel_id":"123","data":{"name":"squad"},"member":{"roles":["ops"]}}"#;
        let mut api = MockDiscordApi::default();

        let decision = handle_live_event(&mut api, payload, "sre-squad")
            .expect("live event should succeed");

        match decision {
            DiscordLiveDecision::Posted { channel_id, text } => {
                assert_eq!(channel_id, "123");
                assert!(text.contains("Active SRE squad"));
                assert_eq!(api.sent.len(), 1);
            }
            _ => panic!("expected posted decision"),
        }
    }

    #[test]
    fn live_event_ignores_non_slash_payload() {
        let payload = r#"{"type":1}"#;
        let mut api = MockDiscordApi::default();

        let decision = handle_live_event(&mut api, payload, "sre-squad")
            .expect("live event should succeed");

        assert_eq!(decision, DiscordLiveDecision::Ignore);
        assert!(api.sent.is_empty());
    }

    #[test]
    fn live_event_requires_channel_id() {
        let payload = r#"{"type":2,"data":{"name":"squad"},"member":{"roles":["ops"]}}"#;
        let mut api = MockDiscordApi::default();

        let err = handle_live_event(&mut api, payload, "sre-squad")
            .expect_err("missing channel_id should fail");

        assert!(err.contains("channel_id"));
    }
}
