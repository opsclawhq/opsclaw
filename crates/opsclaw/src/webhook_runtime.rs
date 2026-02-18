#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookPlatform {
    Slack,
    Discord,
    Telegram,
}

pub fn platform_from_path(path: &str) -> Result<WebhookPlatform, String> {
    match path {
        "/slack/events" => Ok(WebhookPlatform::Slack),
        "/discord/interactions" => Ok(WebhookPlatform::Discord),
        "/telegram/webhook" => Ok(WebhookPlatform::Telegram),
        other => Err(format!(
            "unsupported webhook path `{other}` (expected /slack/events|/discord/interactions|/telegram/webhook)"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_supported_paths() {
        assert_eq!(platform_from_path("/slack/events").expect("slack path"), WebhookPlatform::Slack);
        assert_eq!(platform_from_path("/discord/interactions").expect("discord path"), WebhookPlatform::Discord);
        assert_eq!(platform_from_path("/telegram/webhook").expect("telegram path"), WebhookPlatform::Telegram);
    }

    #[test]
    fn rejects_unknown_paths() {
        let err = platform_from_path("/unknown").expect_err("unknown path should fail");
        assert!(err.contains("unsupported webhook path"));
    }
}
