use crate::squad_responder::{
    agent_reply, approval_message, callback_ack_message, help_message, squad_message,
    start_message,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TelegramCommand {
    pub chat_id: i64,
    pub chat_type: String,
    pub command_name: String,
    pub text: String,
    pub is_group: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelegramRouteDecision {
    Command(TelegramCommand),
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramInlineButton {
    pub text: String,
    pub callback_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TelegramInlineKeyboard {
    pub inline_keyboard: Vec<Vec<TelegramInlineButton>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramLiveConfig {
    pub bot_username: String,
    pub template: String,
    pub max_updates: Option<usize>,
    pub poll_timeout_seconds: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TelegramLiveOutcome {
    pub updates_processed: usize,
    pub replies_sent: usize,
    pub last_update_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum TelegramLiveDecision {
    Replied { chat_id: i64, text: String },
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramOutgoingMessage {
    pub chat_id: i64,
    pub text: String,
    pub reply_markup: Option<TelegramInlineKeyboard>,
}

pub trait TelegramApi {
    fn get_updates(&mut self, offset: Option<i64>, timeout_seconds: u16)
    -> Result<Vec<Value>, String>;
    fn send_message(&mut self, message: TelegramOutgoingMessage) -> Result<(), String>;
}

pub struct HttpTelegramApi {
    token: String,
    client: ureq::Agent,
}

impl HttpTelegramApi {
    pub fn new(token: String) -> Result<Self, String> {
        if token.trim().is_empty() {
            return Err("telegram bot token cannot be empty".to_string());
        }

        Ok(Self {
            token,
            client: ureq::AgentBuilder::new().build(),
        })
    }

    fn endpoint(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }
}

impl TelegramApi for HttpTelegramApi {
    fn get_updates(
        &mut self,
        offset: Option<i64>,
        timeout_seconds: u16,
    ) -> Result<Vec<Value>, String> {
        let mut request = self
            .client
            .get(self.endpoint("getUpdates").as_str())
            .query("timeout", timeout_seconds.to_string().as_str());

        if let Some(value) = offset {
            request = request.query("offset", value.to_string().as_str());
        }

        let response = request.call().map_err(|err| {
            format!("telegram getUpdates request failed: {err}")
        })?;

        let parsed: Value = response.into_json().map_err(|err| {
            format!("telegram getUpdates response parse failed: {err}")
        })?;

        if !parsed
            .get("ok")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            return Err(format!("telegram getUpdates returned non-ok payload: {parsed}"));
        }

        let updates = parsed
            .get("result")
            .and_then(Value::as_array)
            .ok_or_else(|| "telegram getUpdates missing `result` array".to_string())?;

        Ok(updates.to_vec())
    }

    fn send_message(&mut self, message: TelegramOutgoingMessage) -> Result<(), String> {
        let mut payload = serde_json::json!({
            "chat_id": message.chat_id,
            "text": message.text,
        });

        if let Some(reply_markup) = message.reply_markup {
            payload["reply_markup"] = serde_json::to_value(reply_markup)
                .map_err(|err| format!("telegram reply_markup serialization failed: {err}"))?;
        }

        let response = self
            .client
            .post(self.endpoint("sendMessage").as_str())
            .send_json(payload)
            .map_err(|err| format!("telegram sendMessage request failed: {err}"))?;

        let parsed: Value = response
            .into_json()
            .map_err(|err| format!("telegram sendMessage response parse failed: {err}"))?;

        if !parsed
            .get("ok")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            return Err(format!("telegram sendMessage returned non-ok payload: {parsed}"));
        }

        Ok(())
    }
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
        format!("telegram bot token not provided; set `{env_var_name}` or pass --bot-token")
    })?;

    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "telegram bot token env var `{env_var_name}` is set but empty"
        ));
    }

    Ok(trimmed.to_string())
}

pub fn run_live_session(
    api: &mut dyn TelegramApi,
    config: &TelegramLiveConfig,
) -> Result<TelegramLiveOutcome, String> {
    let bot_username = config.bot_username.trim();
    if bot_username.is_empty() {
        return Err("telegram live session requires non-empty bot_username".to_string());
    }

    let template = config.template.trim();
    if template.is_empty() {
        return Err("telegram live session requires non-empty template".to_string());
    }

    let mut offset: Option<i64> = None;
    let mut updates_processed = 0usize;
    let mut replies_sent = 0usize;
    let mut last_update_id = None;

    loop {
        if let Some(limit) = config.max_updates {
            if updates_processed >= limit {
                break;
            }
        }

        let updates = api.get_updates(offset, config.poll_timeout_seconds)?;
        if updates.is_empty() {
            if config.max_updates.is_some() {
                break;
            }
            continue;
        }

        for update in updates {
            if let Some(update_id) = update.get("update_id").and_then(Value::as_i64) {
                offset = Some(update_id + 1);
                last_update_id = Some(update_id);
            }

            updates_processed += 1;

            if let Some(reply) = build_reply_for_update(&update, bot_username, template)? {
                api.send_message(reply)?;
                replies_sent += 1;
            }

            if let Some(limit) = config.max_updates {
                if updates_processed >= limit {
                    break;
                }
            }
        }
    }

    Ok(TelegramLiveOutcome {
        updates_processed,
        replies_sent,
        last_update_id,
    })
}

pub fn handle_live_event(
    api: &mut dyn TelegramApi,
    payload_json: &str,
    bot_username: &str,
    template: &str,
) -> Result<TelegramLiveDecision, String> {
    if bot_username.trim().is_empty() {
        return Err("telegram live-event requires non-empty bot_username".to_string());
    }

    if template.trim().is_empty() {
        return Err("telegram live-event requires non-empty template".to_string());
    }

    let update: Value = serde_json::from_str(payload_json)
        .map_err(|err| format!("invalid telegram update json: {err}"))?;

    match build_reply_for_update(&update, bot_username, template)? {
        Some(reply) => {
            let chat_id = reply.chat_id;
            let text = reply.text.clone();
            api.send_message(reply)?;
            Ok(TelegramLiveDecision::Replied { chat_id, text })
        }
        None => Ok(TelegramLiveDecision::Ignore),
    }
}

pub fn route_telegram_update(
    payload_json: &str,
    bot_username: &str,
) -> Result<TelegramRouteDecision, String> {
    let value: Value = serde_json::from_str(payload_json)
        .map_err(|err| format!("invalid telegram update json: {err}"))?;

    let message = match value.get("message").and_then(Value::as_object) {
        Some(message) => message,
        None => return Ok(TelegramRouteDecision::Ignore),
    };

    let chat = message
        .get("chat")
        .and_then(Value::as_object)
        .ok_or_else(|| "telegram update missing `message.chat`".to_string())?;

    let chat_id = chat
        .get("id")
        .and_then(Value::as_i64)
        .ok_or_else(|| "telegram update missing numeric `message.chat.id`".to_string())?;

    let chat_type = chat
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| "telegram update missing `message.chat.type`".to_string())?
        .to_string();

    let text = message
        .get("text")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();

    if text.is_empty() {
        return Ok(TelegramRouteDecision::Ignore);
    }

    let normalized_bot = bot_username
        .trim()
        .trim_start_matches('@')
        .to_ascii_lowercase();
    if normalized_bot.is_empty() {
        return Err("telegram bot_username cannot be empty".to_string());
    }

    let is_group = is_group_chat(chat_type.as_str());

    let command_name = if let Some(slash_command) = parse_slash_command(text.as_str()) {
        if is_group {
            if let Some(target_bot) = slash_command.target_bot {
                if target_bot != normalized_bot {
                    return Ok(TelegramRouteDecision::Ignore);
                }
            }
        }

        slash_command.command_name
    } else if is_group {
        let mention = format!("@{normalized_bot}");
        if !text.to_ascii_lowercase().contains(mention.as_str()) {
            return Ok(TelegramRouteDecision::Ignore);
        }
        "message".to_string()
    } else {
        "message".to_string()
    };

    Ok(TelegramRouteDecision::Command(TelegramCommand {
        chat_id,
        chat_type,
        command_name,
        text,
        is_group,
    }))
}

pub fn build_inline_keyboard(
    rows: Vec<Vec<TelegramInlineButton>>,
) -> Result<TelegramInlineKeyboard, String> {
    if rows.is_empty() {
        return Err("telegram inline keyboard requires at least one row".to_string());
    }

    if rows.iter().any(|row| row.is_empty()) {
        return Err("telegram inline keyboard rows cannot be empty".to_string());
    }

    for row in &rows {
        for button in row {
            if button.text.trim().is_empty() {
                return Err("telegram inline keyboard button text cannot be empty".to_string());
            }

            if button.callback_data.trim().is_empty() {
                return Err(
                    "telegram inline keyboard button callback_data cannot be empty".to_string(),
                );
            }
        }
    }

    Ok(TelegramInlineKeyboard {
        inline_keyboard: rows,
    })
}

pub fn is_group_chat(chat_type: &str) -> bool {
    matches!(chat_type, "group" | "supergroup")
}

#[derive(Debug)]
struct ParsedSlashCommand {
    command_name: String,
    target_bot: Option<String>,
}

fn parse_slash_command(text: &str) -> Option<ParsedSlashCommand> {
    let first_token = text.split_whitespace().next()?;
    if !first_token.starts_with('/') {
        return None;
    }

    let without_slash = &first_token[1..];
    if without_slash.trim().is_empty() {
        return None;
    }

    let (command_name, target_bot) = match without_slash.split_once('@') {
        Some((command_name, target_bot)) => (
            command_name.to_string(),
            Some(target_bot.to_ascii_lowercase()),
        ),
        None => (without_slash.to_string(), None),
    };

    Some(ParsedSlashCommand {
        command_name,
        target_bot,
    })
}

fn build_reply_for_update(
    update: &Value,
    bot_username: &str,
    template: &str,
) -> Result<Option<TelegramOutgoingMessage>, String> {
    if let Some(reply) = build_callback_reply(update) {
        return Ok(Some(reply));
    }

    let payload_json = serde_json::to_string(update)
        .map_err(|err| format!("telegram update serialization failed: {err}"))?;

    match route_telegram_update(payload_json.as_str(), bot_username)? {
        TelegramRouteDecision::Ignore => Ok(None),
        TelegramRouteDecision::Command(command) => {
            let command_name = command.command_name.to_ascii_lowercase();
            let (text, reply_markup) = match command_name.as_str() {
                "start" => (start_message(template), None),
                "help" => (help_message(), None),
                "squad" => (squad_message(template), None),
                "approve" => {
                    let keyboard = build_inline_keyboard(vec![vec![
                        TelegramInlineButton {
                            text: "Approve".to_string(),
                            callback_data: "approve".to_string(),
                        },
                        TelegramInlineButton {
                            text: "Reject".to_string(),
                            callback_data: "reject".to_string(),
                        },
                    ]])?;

                    (
                        approval_message(),
                        Some(keyboard),
                    )
                }
                _ => (agent_reply(template, command.text.as_str()), None),
            };

            Ok(Some(TelegramOutgoingMessage {
                chat_id: command.chat_id,
                text,
                reply_markup,
            }))
        }
    }
}

fn build_callback_reply(update: &Value) -> Option<TelegramOutgoingMessage> {
    let callback = update.get("callback_query")?;
    let data = callback
        .get("data")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())?;

    let chat_id = callback
        .get("message")
        .and_then(|value| value.get("chat"))
        .and_then(|value| value.get("id"))
        .and_then(Value::as_i64)?;

    Some(TelegramOutgoingMessage {
        chat_id,
        text: callback_ack_message(data),
        reply_markup: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    struct MockTelegramApi {
        updates: VecDeque<Vec<Value>>,
        sent_messages: Vec<TelegramOutgoingMessage>,
    }

    impl MockTelegramApi {
        fn with_updates(updates: Vec<Vec<Value>>) -> Self {
            Self {
                updates: VecDeque::from(updates),
                sent_messages: Vec::new(),
            }
        }
    }

    impl TelegramApi for MockTelegramApi {
        fn get_updates(
            &mut self,
            _offset: Option<i64>,
            _timeout_seconds: u16,
        ) -> Result<Vec<Value>, String> {
            Ok(self.updates.pop_front().unwrap_or_default())
        }

        fn send_message(&mut self, message: TelegramOutgoingMessage) -> Result<(), String> {
            self.sent_messages.push(message);
            Ok(())
        }
    }

    #[test]
    fn routes_group_command_update() {
        let payload =
            r#"{"message":{"chat":{"id":42,"type":"group"},"text":"/status@opsclaw_bot"}}"#;

        let decision = route_telegram_update(payload, "opsclaw_bot").expect("route should parse");

        match decision {
            TelegramRouteDecision::Command(command) => {
                assert_eq!(command.chat_id, 42);
                assert_eq!(command.chat_type, "group");
                assert_eq!(command.command_name, "status");
                assert!(command.is_group);
            }
            TelegramRouteDecision::Ignore => panic!("expected group command route"),
        }
    }

    #[test]
    fn group_command_for_different_bot_is_ignored() {
        let payload = r#"{"message":{"chat":{"id":42,"type":"group"},"text":"/status@other_bot"}}"#;

        let decision = route_telegram_update(payload, "opsclaw_bot").expect("route should parse");
        assert_eq!(decision, TelegramRouteDecision::Ignore);
    }

    #[test]
    fn builds_inline_keyboard_payload_has_required_shape() {
        let keyboard = build_inline_keyboard(vec![vec![TelegramInlineButton {
            text: "Approve".to_string(),
            callback_data: "approve:run-1".to_string(),
        }]])
        .expect("keyboard should build");

        assert_eq!(keyboard.inline_keyboard.len(), 1);
        assert_eq!(keyboard.inline_keyboard[0].len(), 1);
        assert_eq!(keyboard.inline_keyboard[0][0].text, "Approve");
        assert_eq!(
            keyboard.inline_keyboard[0][0].callback_data,
            "approve:run-1"
        );
    }

    #[test]
    fn group_support_detection_matches_types() {
        assert!(!is_group_chat("private"));
        assert!(is_group_chat("group"));
        assert!(is_group_chat("supergroup"));
    }

    #[test]
    fn resolves_explicit_token_before_env() {
        std::env::set_var("OPSCLAW_TEST_TELEGRAM_TOKEN_EXPLICIT", "env-token");

        let token = resolve_bot_token(
            Some("explicit-token"),
            None,
            "OPSCLAW_TEST_TELEGRAM_TOKEN_EXPLICIT",
        )
        .expect("token should resolve");

        assert_eq!(token, "explicit-token");

        std::env::remove_var("OPSCLAW_TEST_TELEGRAM_TOKEN_EXPLICIT");
    }

    #[test]
    fn resolves_env_token_when_explicit_missing() {
        std::env::set_var("OPSCLAW_TEST_TELEGRAM_TOKEN_ENV", "env-token");

        let token = resolve_bot_token(None, None, "OPSCLAW_TEST_TELEGRAM_TOKEN_ENV")
            .expect("token should resolve from env");

        assert_eq!(token, "env-token");

        std::env::remove_var("OPSCLAW_TEST_TELEGRAM_TOKEN_ENV");
    }

    #[test]
    fn missing_token_returns_clear_error() {
        std::env::remove_var("OPSCLAW_TEST_TELEGRAM_TOKEN_MISSING");

        let err = resolve_bot_token(None, None, "OPSCLAW_TEST_TELEGRAM_TOKEN_MISSING")
            .expect_err("missing token should fail");

        assert!(err.contains("OPSCLAW_TEST_TELEGRAM_TOKEN_MISSING"));
    }

    #[test]
    fn live_loop_replies_to_group_mention() {
        let update = serde_json::json!({
            "update_id": 100,
            "message": {
                "chat": { "id": 42, "type": "group" },
                "text": "hey @opsclaw_bot check cluster health"
            }
        });

        let mut mock = MockTelegramApi::with_updates(vec![vec![update]]);
        let config = TelegramLiveConfig {
            bot_username: "opsclaw_bot".to_string(),
            template: "sre-squad".to_string(),
            max_updates: Some(1),
            poll_timeout_seconds: 0,
        };

        let outcome = run_live_session(&mut mock, &config).expect("live session should succeed");
        assert_eq!(outcome.updates_processed, 1);
        assert_eq!(outcome.replies_sent, 1);
        assert_eq!(mock.sent_messages.len(), 1);
        assert_eq!(mock.sent_messages[0].chat_id, 42);
    }

    #[test]
    fn squad_command_lists_template_members() {
        let update = serde_json::json!({
            "update_id": 101,
            "message": {
                "chat": { "id": 9, "type": "private" },
                "text": "/squad"
            }
        });

        let mut mock = MockTelegramApi::with_updates(vec![vec![update]]);
        let config = TelegramLiveConfig {
            bot_username: "opsclaw_bot".to_string(),
            template: "sre-squad".to_string(),
            max_updates: Some(1),
            poll_timeout_seconds: 0,
        };

        run_live_session(&mut mock, &config).expect("live session should succeed");
        let reply = &mock.sent_messages[0].text;
        assert!(reply.contains("Remy"));
        assert!(reply.contains("SRE"));
    }

    #[test]
    fn approve_command_sends_inline_keyboard() {
        let update = serde_json::json!({
            "update_id": 102,
            "message": {
                "chat": { "id": 11, "type": "private" },
                "text": "/approve"
            }
        });

        let mut mock = MockTelegramApi::with_updates(vec![vec![update]]);
        let config = TelegramLiveConfig {
            bot_username: "opsclaw_bot".to_string(),
            template: "sre-squad".to_string(),
            max_updates: Some(1),
            poll_timeout_seconds: 0,
        };

        run_live_session(&mut mock, &config).expect("live session should succeed");
        assert!(mock.sent_messages[0].reply_markup.is_some());
    }

    #[test]
    fn callback_query_generates_ack_message() {
        let update = serde_json::json!({
            "update_id": 103,
            "callback_query": {
                "id": "cb1",
                "data": "approve",
                "message": {
                    "chat": { "id": 11, "type": "private" }
                }
            }
        });

        let mut mock = MockTelegramApi::with_updates(vec![vec![update]]);
        let config = TelegramLiveConfig {
            bot_username: "opsclaw_bot".to_string(),
            template: "sre-squad".to_string(),
            max_updates: Some(1),
            poll_timeout_seconds: 0,
        };

        run_live_session(&mut mock, &config).expect("live session should succeed");
        assert!(mock.sent_messages[0].text.contains("Approval recorded"));
    }

    #[test]
    fn live_event_posts_reply_for_group_mention() {
        let payload = r#"{
            "update_id": 200,
            "message": {
                "chat": { "id": 42, "type": "group" },
                "text": "hey @opsclaw_bot status"
            }
        }"#;
        let mut mock = MockTelegramApi::with_updates(vec![]);

        let decision = handle_live_event(&mut mock, payload, "opsclaw_bot", "sre-squad")
            .expect("live event should succeed");

        match decision {
            TelegramLiveDecision::Replied { chat_id, text } => {
                assert_eq!(chat_id, 42);
                assert!(text.contains("taking point"));
                assert_eq!(mock.sent_messages.len(), 1);
            }
            _ => panic!("expected replied decision"),
        }
    }

    #[test]
    fn live_event_ignores_non_routable_update() {
        let payload = r#"{
            "update_id": 201,
            "message": {
                "chat": { "id": 42, "type": "private" },
                "text": ""
            }
        }"#;
        let mut mock = MockTelegramApi::with_updates(vec![]);

        let decision = handle_live_event(&mut mock, payload, "opsclaw_bot", "sre-squad")
            .expect("live event should succeed");

        assert_eq!(decision, TelegramLiveDecision::Ignore);
        assert!(mock.sent_messages.is_empty());
    }
}
