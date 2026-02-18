use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
