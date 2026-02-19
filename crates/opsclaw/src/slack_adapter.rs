use crate::squad_responder::response_for_input;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::env;
use url::form_urlencoded::Serializer;

const SLACK_OAUTH_AUTHORIZE_URL: &str = "https://slack.com/oauth/v2/authorize";
const SLACK_CHAT_POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackInstallConfig {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub user_scopes: Vec<String>,
    pub redirect_uri: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackMentionRoute {
    pub channel: String,
    pub thread_ts: String,
    pub cleaned_text: String,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlackRouteDecision {
    UrlVerification { challenge: String },
    Mention(SlackMentionRoute),
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackOutgoingMessage {
    pub channel: String,
    pub thread_ts: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum SlackLiveDecision {
    UrlVerification {
        challenge: String,
    },
    Replied {
        channel: String,
        thread_ts: String,
        text: String,
    },
    Ignore,
}

pub trait SlackApi {
    fn post_message(&mut self, message: SlackOutgoingMessage) -> Result<(), String>;
}

pub struct HttpSlackApi {
    bot_token: String,
    client: ureq::Agent,
}

impl HttpSlackApi {
    pub fn new(bot_token: String) -> Result<Self, String> {
        if bot_token.trim().is_empty() {
            return Err("slack bot token cannot be empty".to_string());
        }

        Ok(Self {
            bot_token,
            client: ureq::AgentBuilder::new().build(),
        })
    }
}

impl SlackApi for HttpSlackApi {
    fn post_message(&mut self, message: SlackOutgoingMessage) -> Result<(), String> {
        let mut payload = serde_json::json!({
            "channel": message.channel,
            "text": message.text,
        });
        if let Some(thread_ts) = message.thread_ts {
            payload["thread_ts"] = serde_json::json!(thread_ts);
        }

        let response = match self
            .client
            .post(SLACK_CHAT_POST_MESSAGE_URL)
            .set("Authorization", format!("Bearer {}", self.bot_token).as_str())
            .set("Content-Type", "application/json")
            .send_json(payload)
        {
            Ok(value) => value,
            Err(ureq::Error::Status(status, response)) => {
                let retry_after = retry_after_seconds(status, response.header("Retry-After"));
                if let Some(seconds) = retry_after {
                    return Err(format!(
                        "slack chat.postMessage request failed: status={status} retry_after_seconds={seconds}"
                    ));
                }
                return Err(format!("slack chat.postMessage request failed: status={status}"));
            }
            Err(err) => return Err(format!("slack chat.postMessage request failed: {err}")),
        };

        let parsed: Value = response
            .into_json()
            .map_err(|err| format!("slack chat.postMessage response parse failed: {err}"))?;

        if !parsed.get("ok").and_then(Value::as_bool).unwrap_or(false) {
            let error = parsed
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("unknown_error");
            return Err(format!("slack chat.postMessage returned non-ok payload: {error}"));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct SlackEnvelope {
    #[serde(rename = "type")]
    envelope_type: String,
    challenge: Option<String>,
    event: Option<SlackEvent>,
}

#[derive(Debug, Deserialize)]
struct SlackEvent {
    #[serde(rename = "type")]
    event_type: String,
    channel: Option<String>,
    text: Option<String>,
    ts: Option<String>,
    thread_ts: Option<String>,
    user: Option<String>,
}

pub fn build_install_url(config: &SlackInstallConfig) -> Result<String, String> {
    if config.client_id.trim().is_empty() {
        return Err("client_id is required".to_string());
    }
    if config.state.trim().is_empty() {
        return Err("state is required".to_string());
    }
    if config.scopes.is_empty() {
        return Err("at least one scope is required".to_string());
    }

    let mut query = Serializer::new(String::new());
    query.append_pair("client_id", config.client_id.as_str());
    query.append_pair("scope", config.scopes.join(",").as_str());
    query.append_pair("state", config.state.as_str());

    if !config.user_scopes.is_empty() {
        query.append_pair("user_scope", config.user_scopes.join(",").as_str());
    }

    if let Some(redirect_uri) = &config.redirect_uri {
        if !redirect_uri.trim().is_empty() {
            query.append_pair("redirect_uri", redirect_uri.as_str());
        }
    }

    Ok(format!("{}?{}", SLACK_OAUTH_AUTHORIZE_URL, query.finish()))
}

pub fn route_for_bot(payload_json: &str, bot_user_id: &str) -> Result<SlackRouteDecision, String> {
    if bot_user_id.trim().is_empty() {
        return Err("bot_user_id is required".to_string());
    }

    let envelope = serde_json::from_str::<SlackEnvelope>(payload_json)
        .map_err(|err| format!("invalid slack payload json: {err}"))?;

    if envelope.envelope_type == "url_verification" {
        let challenge = envelope
            .challenge
            .ok_or_else(|| "url_verification payload missing challenge".to_string())?;
        return Ok(SlackRouteDecision::UrlVerification { challenge });
    }

    if envelope.envelope_type != "event_callback" {
        return Ok(SlackRouteDecision::Ignore);
    }

    let event = envelope
        .event
        .ok_or_else(|| "event_callback payload missing event object".to_string())?;

    if event.event_type != "app_mention" && event.event_type != "message" {
        return Ok(SlackRouteDecision::Ignore);
    }

    let mention_token = format!("<@{}>", bot_user_id);
    let text = event.text.unwrap_or_default();

    if !text.contains(mention_token.as_str()) {
        return Ok(SlackRouteDecision::Ignore);
    }

    let channel = event
        .channel
        .ok_or_else(|| "slack event missing channel".to_string())?;
    let ts = event.ts.ok_or_else(|| "slack event missing ts".to_string())?;
    let thread_ts = event.thread_ts.unwrap_or(ts);

    let cleaned_text = text.replace(mention_token.as_str(), "").trim().to_string();

    Ok(SlackRouteDecision::Mention(SlackMentionRoute {
        channel,
        thread_ts,
        cleaned_text,
        user_id: event.user,
    }))
}

pub fn retry_after_seconds(status_code: u16, retry_after_header: Option<&str>) -> Option<u64> {
    if status_code != 429 {
        return None;
    }

    retry_after_header.and_then(|value| value.trim().parse::<u64>().ok())
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
        format!("slack bot token not provided; set `{env_var_name}` or pass --bot-token")
    })?;

    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "slack bot token env var `{env_var_name}` is set but empty"
        ));
    }

    Ok(trimmed.to_string())
}

pub fn handle_live_event(
    api: &mut dyn SlackApi,
    payload_json: &str,
    bot_user_id: &str,
    template: &str,
) -> Result<SlackLiveDecision, String> {
    let route = route_for_bot(payload_json, bot_user_id)?;

    match route {
        SlackRouteDecision::UrlVerification { challenge } => {
            Ok(SlackLiveDecision::UrlVerification { challenge })
        }
        SlackRouteDecision::Ignore => Ok(SlackLiveDecision::Ignore),
        SlackRouteDecision::Mention(mention) => {
            let text = response_for_input(template, mention.cleaned_text.as_str());
            let outgoing = SlackOutgoingMessage {
                channel: mention.channel.clone(),
                thread_ts: Some(mention.thread_ts.clone()),
                text: text.clone(),
            };
            api.post_message(outgoing)?;
            Ok(SlackLiveDecision::Replied {
                channel: mention.channel,
                thread_ts: mention.thread_ts,
                text,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockSlackApi {
        sent: Vec<SlackOutgoingMessage>,
    }

    impl SlackApi for MockSlackApi {
        fn post_message(&mut self, message: SlackOutgoingMessage) -> Result<(), String> {
            self.sent.push(message);
            Ok(())
        }
    }

    #[test]
    fn builds_install_url_with_required_fields() {
        let config = SlackInstallConfig {
            client_id: "123.456".to_string(),
            scopes: vec!["app_mentions:read".to_string(), "chat:write".to_string()],
            user_scopes: vec![],
            redirect_uri: Some("https://example.com/slack/callback".to_string()),
            state: "state-1".to_string(),
        };

        let url = build_install_url(&config).expect("install url should build");
        assert!(url.contains("client_id=123.456"));
        assert!(url.contains("scope=app_mentions%3Aread%2Cchat%3Awrite"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fslack%2Fcallback"));
        assert!(url.contains("state=state-1"));
    }

    #[test]
    fn routes_app_mention_to_thread() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "app_mention",
                "channel": "C123",
                "text": "<@U_BOT> investigate error budget",
                "user": "U_CALLER",
                "ts": "173.10",
                "thread_ts": "173.01"
            }
        }"#;

        let route = route_for_bot(payload, "U_BOT").expect("route should parse");
        match route {
            SlackRouteDecision::Mention(mention) => {
                assert_eq!(mention.channel, "C123");
                assert_eq!(mention.thread_ts, "173.01");
                assert_eq!(mention.cleaned_text, "investigate error budget");
                assert_eq!(mention.user_id.as_deref(), Some("U_CALLER"));
            }
            _ => panic!("expected mention route"),
        }
    }

    #[test]
    fn mention_without_thread_defaults_to_message_ts() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123",
                "text": "hi <@U_BOT>",
                "ts": "173.10"
            }
        }"#;

        let route = route_for_bot(payload, "U_BOT").expect("route should parse");
        match route {
            SlackRouteDecision::Mention(mention) => {
                assert_eq!(mention.thread_ts, "173.10");
                assert_eq!(mention.cleaned_text, "hi");
            }
            _ => panic!("expected mention route"),
        }
    }

    #[test]
    fn non_mention_message_is_ignored() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123",
                "text": "hello everyone",
                "ts": "173.10"
            }
        }"#;

        let route = route_for_bot(payload, "U_BOT").expect("route should parse");
        assert_eq!(route, SlackRouteDecision::Ignore);
    }

    #[test]
    fn computes_retry_after_for_rate_limit_response() {
        let retry = retry_after_seconds(429, Some("30"));
        assert_eq!(retry, Some(30));
        assert_eq!(retry_after_seconds(200, Some("30")), None);
        assert_eq!(retry_after_seconds(429, Some("oops")), None);
    }

    #[test]
    fn handles_url_verification_payload() {
        let payload = r#"{
            "type": "url_verification",
            "challenge": "challenge-123"
        }"#;

        let route = route_for_bot(payload, "U_BOT").expect("route should parse");
        assert_eq!(
            route,
            SlackRouteDecision::UrlVerification {
                challenge: "challenge-123".to_string()
            }
        );
    }

    #[test]
    fn live_event_routes_mention_and_posts_reply() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "app_mention",
                "channel": "C123",
                "text": "<@U_BOT> squad",
                "user": "U_CALLER",
                "ts": "173.10",
                "thread_ts": "173.01"
            }
        }"#;

        let mut api = MockSlackApi::default();
        let decision = handle_live_event(
            &mut api,
            payload,
            "U_BOT",
            "sre-squad",
        )
        .expect("live event should succeed");

        match decision {
            SlackLiveDecision::Replied { channel, thread_ts, text } => {
                assert_eq!(channel, "C123");
                assert_eq!(thread_ts, "173.01");
                assert!(text.contains("Active SRE squad"));
                assert_eq!(api.sent.len(), 1);
            }
            _ => panic!("expected replied decision"),
        }
    }

    #[test]
    fn live_event_url_verification_does_not_post_message() {
        let payload = r#"{
            "type": "url_verification",
            "challenge": "challenge-123"
        }"#;

        let mut api = MockSlackApi::default();
        let decision = handle_live_event(
            &mut api,
            payload,
            "U_BOT",
            "sre-squad",
        )
        .expect("live event should succeed");

        match decision {
            SlackLiveDecision::UrlVerification { challenge } => {
                assert_eq!(challenge, "challenge-123");
                assert!(api.sent.is_empty());
            }
            _ => panic!("expected url verification"),
        }
    }

    #[test]
    fn live_event_ignores_non_mentions_without_posting() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123",
                "text": "hello everyone",
                "ts": "173.10"
            }
        }"#;

        let mut api = MockSlackApi::default();
        let decision = handle_live_event(&mut api, payload, "U_BOT", "sre-squad")
            .expect("live event should succeed");

        assert_eq!(decision, SlackLiveDecision::Ignore);
        assert!(api.sent.is_empty());
    }

    #[test]
    fn resolves_explicit_token_before_env() {
        std::env::set_var("OPSCLAW_TEST_SLACK_TOKEN_EXPLICIT", "env-token");
        let token = resolve_bot_token(
            Some("explicit-token"),
            None,
            "OPSCLAW_TEST_SLACK_TOKEN_EXPLICIT",
        )
        .expect("token should resolve");
        assert_eq!(token, "explicit-token");
        std::env::remove_var("OPSCLAW_TEST_SLACK_TOKEN_EXPLICIT");
    }
}
