use std::collections::VecDeque;
use hmac::{Hmac, Mac};
use sha2::Sha256;

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

pub fn enforce_rate_limit(
    request_timestamps: &mut VecDeque<u64>,
    now_epoch_seconds: u64,
    max_requests: Option<usize>,
    window_seconds: u64,
) -> Result<(), String> {
    let Some(max) = max_requests else {
        return Ok(());
    };

    if max == 0 {
        return Err("webhook rate limit max requests must be greater than zero".to_string());
    }

    if window_seconds == 0 {
        return Err("webhook rate limit window seconds must be greater than zero".to_string());
    }

    while let Some(first_seen) = request_timestamps.front().copied() {
        if now_epoch_seconds.saturating_sub(first_seen) >= window_seconds {
            request_timestamps.pop_front();
        } else {
            break;
        }
    }

    if request_timestamps.len() >= max {
        return Err(format!(
            "webhook rate limit exceeded: max {max} requests per {window_seconds}s"
        ));
    }

    request_timestamps.push_back(now_epoch_seconds);
    Ok(())
}

pub fn verify_slack_request_signature(
    request_body: &str,
    provided_signature: Option<&str>,
    provided_timestamp: Option<&str>,
    signing_secret: Option<&str>,
    now_epoch_seconds: u64,
    tolerance_seconds: u64,
) -> Result<(), String> {
    let secret = match signing_secret.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => value,
        None => return Ok(()),
    };

    if tolerance_seconds == 0 {
        return Err("slack signature tolerance seconds must be greater than zero".to_string());
    }

    let timestamp = provided_timestamp
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing slack request timestamp".to_string())?;

    let timestamp_seconds: u64 = timestamp
        .parse()
        .map_err(|_| "invalid slack request timestamp".to_string())?;

    let drift_seconds = now_epoch_seconds.abs_diff(timestamp_seconds);
    if drift_seconds > tolerance_seconds {
        return Err(format!(
            "stale slack request timestamp (drift={drift_seconds}s tolerance={tolerance_seconds}s)"
        ));
    }

    let signature = provided_signature
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing slack request signature".to_string())?;

    let expected_signature = compute_slack_signature(request_body, timestamp, secret)?;
    if !signature.eq_ignore_ascii_case(expected_signature.as_str()) {
        return Err("invalid slack request signature".to_string());
    }

    Ok(())
}

fn compute_slack_signature(
    request_body: &str,
    timestamp: &str,
    signing_secret: &str,
) -> Result<String, String> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(signing_secret.as_bytes())
        .map_err(|err| format!("failed to initialize slack signature verifier: {err}"))?;
    let base_string = format!("v0:{timestamp}:{request_body}");
    mac.update(base_string.as_bytes());

    let digest = mac.finalize().into_bytes();
    Ok(format!("v0={}", hex::encode(digest)))
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

    #[test]
    fn rate_limit_rejects_when_window_is_full() {
        let mut timestamps = VecDeque::from(vec![100, 110, 119]);

        let err = enforce_rate_limit(&mut timestamps, 120, Some(3), 60)
            .expect_err("window should be saturated");

        assert!(err.contains("webhook rate limit exceeded"));
    }

    #[test]
    fn rate_limit_allows_after_window_prune() {
        let mut timestamps = VecDeque::from(vec![10, 20, 79]);

        enforce_rate_limit(&mut timestamps, 80, Some(3), 60)
            .expect("old entries should be pruned");

        assert_eq!(timestamps, VecDeque::from(vec![79, 80]));
    }

    #[test]
    fn rate_limit_is_disabled_when_max_is_missing() {
        let mut timestamps = VecDeque::from(vec![1, 2, 3]);
        enforce_rate_limit(&mut timestamps, 4, None, 60)
            .expect("missing max limit should disable limiting");
        assert_eq!(timestamps, VecDeque::from(vec![1, 2, 3]));
    }

    #[test]
    fn slack_signature_rejects_mismatch() {
        let err = verify_slack_request_signature(
            "{\"type\":\"url_verification\"}",
            Some("v0=deadbeef"),
            Some("1700000000"),
            Some("signing-secret"),
            1700000010,
            300,
        )
        .expect_err("mismatched signature should fail");

        assert!(err.contains("invalid slack request signature"));
    }

    #[test]
    fn slack_signature_accepts_valid_signature() {
        let body = "{\"type\":\"url_verification\"}";
        let timestamp = "1700000000";
        let signature =
            compute_slack_signature(body, timestamp, "signing-secret").expect("signature");

        verify_slack_request_signature(
            body,
            Some(signature.as_str()),
            Some(timestamp),
            Some("signing-secret"),
            1700000010,
            300,
        )
        .expect("valid signature should pass");
    }

    #[test]
    fn slack_signature_rejects_stale_timestamp() {
        let body = "{\"type\":\"url_verification\"}";
        let timestamp = "1700000000";
        let signature =
            compute_slack_signature(body, timestamp, "signing-secret").expect("signature");

        let err = verify_slack_request_signature(
            body,
            Some(signature.as_str()),
            Some(timestamp),
            Some("signing-secret"),
            1700000900,
            300,
        )
        .expect_err("stale signature should fail");

        assert!(err.contains("stale slack request timestamp"));
    }

    #[test]
    fn slack_signature_allows_when_secret_not_configured() {
        verify_slack_request_signature(
            "{}",
            None,
            None,
            None,
            1700000000,
            300,
        )
        .expect("signature verification should be disabled");
    }
}
