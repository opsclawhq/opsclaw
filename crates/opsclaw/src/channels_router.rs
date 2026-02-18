use crate::discord_adapter::{route_discord_payload, DiscordRouteDecision};
use crate::slack_adapter::{route_for_bot, SlackRouteDecision};
use crate::telegram_adapter::{route_telegram_update, TelegramRouteDecision};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelPlatform {
    Slack,
    Discord,
    Telegram,
}

impl ChannelPlatform {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "slack" => Ok(Self::Slack),
            "discord" => Ok(Self::Discord),
            "telegram" => Ok(Self::Telegram),
            other => Err(format!(
                "unsupported platform `{other}` (expected slack|discord|telegram)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnifiedRoute {
    pub platform: String,
    pub route_kind: String,
    pub target_ref: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelRouteDecision {
    Routed(UnifiedRoute),
    Ignore,
}

pub fn route_platform_event(
    platform: ChannelPlatform,
    payload_json: &str,
    identity: Option<&str>,
) -> Result<ChannelRouteDecision, String> {
    match platform {
        ChannelPlatform::Slack => route_slack(payload_json, identity),
        ChannelPlatform::Discord => route_discord(payload_json),
        ChannelPlatform::Telegram => route_telegram(payload_json, identity),
    }
}

fn route_slack(payload_json: &str, identity: Option<&str>) -> Result<ChannelRouteDecision, String> {
    let bot_user_id = identity
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "slack routing requires identity (bot_user_id)".to_string())?;

    let decision = route_for_bot(payload_json, bot_user_id)
        .map_err(|err| format!("slack route failed: {err}"))?;

    match decision {
        SlackRouteDecision::Mention(mention) => Ok(ChannelRouteDecision::Routed(UnifiedRoute {
            platform: "slack".to_string(),
            route_kind: "mention".to_string(),
            target_ref: mention.channel,
            text: mention.cleaned_text,
        })),
        SlackRouteDecision::UrlVerification { .. } | SlackRouteDecision::Ignore => {
            Ok(ChannelRouteDecision::Ignore)
        }
    }
}

fn route_discord(payload_json: &str) -> Result<ChannelRouteDecision, String> {
    let decision = route_discord_payload(payload_json)
        .map_err(|err| format!("discord route failed: {err}"))?;

    match decision {
        DiscordRouteDecision::SlashCommand(command) => {
            Ok(ChannelRouteDecision::Routed(UnifiedRoute {
                platform: "discord".to_string(),
                route_kind: "slash_command".to_string(),
                target_ref: "discord".to_string(),
                text: command.command_name,
            }))
        }
        DiscordRouteDecision::Ignore => Ok(ChannelRouteDecision::Ignore),
    }
}

fn route_telegram(
    payload_json: &str,
    identity: Option<&str>,
) -> Result<ChannelRouteDecision, String> {
    let bot_username = identity
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "telegram routing requires identity (bot_username)".to_string())?;

    let decision = route_telegram_update(payload_json, bot_username)
        .map_err(|err| format!("telegram route failed: {err}"))?;

    match decision {
        TelegramRouteDecision::Command(command) => Ok(ChannelRouteDecision::Routed(UnifiedRoute {
            platform: "telegram".to_string(),
            route_kind: "command".to_string(),
            target_ref: format!("{}:{}", command.chat_type, command.chat_id),
            text: command.command_name,
        })),
        TelegramRouteDecision::Ignore => Ok(ChannelRouteDecision::Ignore),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_slack_mention_into_unified_event() {
        let payload = r#"{"type":"event_callback","event":{"type":"app_mention","user":"U123","text":"<@UOPS> deploy status","channel":"C123","ts":"1000.1","thread_ts":"1000.1"}}"#;

        let decision = route_platform_event(ChannelPlatform::Slack, payload, Some("UOPS"))
            .expect("slack route should parse");

        match decision {
            ChannelRouteDecision::Routed(route) => {
                assert_eq!(route.platform, "slack");
                assert_eq!(route.route_kind, "mention");
                assert_eq!(route.target_ref, "C123");
                assert_eq!(route.text, "deploy status");
            }
            ChannelRouteDecision::Ignore => panic!("expected routed event"),
        }
    }

    #[test]
    fn routes_discord_slash_into_unified_event() {
        let payload = r#"{"type":2,"data":{"name":"status"},"member":{"roles":["ops"]}}"#;

        let decision = route_platform_event(ChannelPlatform::Discord, payload, None)
            .expect("discord route should parse");

        match decision {
            ChannelRouteDecision::Routed(route) => {
                assert_eq!(route.platform, "discord");
                assert_eq!(route.route_kind, "slash_command");
                assert_eq!(route.target_ref, "discord");
                assert_eq!(route.text, "status");
            }
            ChannelRouteDecision::Ignore => panic!("expected routed event"),
        }
    }

    #[test]
    fn routes_telegram_group_command_into_unified_event() {
        let payload =
            r#"{"message":{"chat":{"id":42,"type":"group"},"text":"/status@opsclaw_bot"}}"#;

        let decision =
            route_platform_event(ChannelPlatform::Telegram, payload, Some("opsclaw_bot"))
                .expect("telegram route should parse");

        match decision {
            ChannelRouteDecision::Routed(route) => {
                assert_eq!(route.platform, "telegram");
                assert_eq!(route.route_kind, "command");
                assert_eq!(route.target_ref, "group:42");
                assert_eq!(route.text, "status");
            }
            ChannelRouteDecision::Ignore => panic!("expected routed event"),
        }
    }

    #[test]
    fn ignores_non_routable_events() {
        let payload = r#"{"type":"event_callback","event":{"type":"message","text":"hello","channel":"C123"}}"#;

        let decision = route_platform_event(ChannelPlatform::Slack, payload, Some("UOPS"))
            .expect("slack route should parse");
        assert_eq!(decision, ChannelRouteDecision::Ignore);
    }
}
