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

pub fn validate_shared_secret(
    provided_secret: Option<&str>,
    required_secret: Option<&str>,
) -> Result<(), String> {
    let required = match required_secret.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => value,
        None => return Ok(()),
    };

    let provided = provided_secret
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing webhook shared secret".to_string())?;

    if provided != required {
        return Err("invalid webhook shared secret".to_string());
    }

    Ok(())
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

    #[test]
    fn rejects_missing_secret_when_required() {
        let err = validate_shared_secret(None, Some("expected-secret"))
            .expect_err("missing secret should fail");
        assert!(err.contains("missing webhook shared secret"));
    }

    #[test]
    fn rejects_invalid_secret_when_required() {
        let err = validate_shared_secret(Some("wrong-secret"), Some("expected-secret"))
            .expect_err("wrong secret should fail");
        assert!(err.contains("invalid webhook shared secret"));
    }

    #[test]
    fn accepts_when_secret_matches_or_not_required() {
        validate_shared_secret(Some("expected-secret"), Some("expected-secret"))
            .expect("matching secret should pass");
        validate_shared_secret(None, None).expect("open ingress should pass");
    }
}
