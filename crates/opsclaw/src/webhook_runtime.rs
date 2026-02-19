use std::collections::VecDeque;
use ed25519_dalek::{Signature as Ed25519Signature, Verifier, VerifyingKey};
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

pub fn verify_discord_request_signature(
    request_body: &str,
    provided_signature: Option<&str>,
    provided_timestamp: Option<&str>,
    discord_public_key: Option<&str>,
    now_epoch_seconds: u64,
    tolerance_seconds: u64,
) -> Result<(), String> {
    let public_key = match discord_public_key.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => value,
        None => return Ok(()),
    };

    if tolerance_seconds == 0 {
        return Err("discord signature tolerance seconds must be greater than zero".to_string());
    }

    let timestamp = provided_timestamp
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing discord request timestamp".to_string())?;

    let timestamp_seconds: u64 = timestamp
        .parse()
        .map_err(|_| "invalid discord request timestamp".to_string())?;

    let drift_seconds = now_epoch_seconds.abs_diff(timestamp_seconds);
    if drift_seconds > tolerance_seconds {
        return Err(format!(
            "stale discord request timestamp (drift={drift_seconds}s tolerance={tolerance_seconds}s)"
        ));
    }

    let signature = provided_signature
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing discord request signature".to_string())?;

    let public_key_bytes = hex::decode(public_key)
        .map_err(|_| "invalid discord public key encoding".to_string())?;
    let public_key_array: [u8; 32] = public_key_bytes
        .try_into()
        .map_err(|_| "invalid discord public key length".to_string())?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)
        .map_err(|_| "invalid discord public key".to_string())?;

    let signature_bytes = hex::decode(signature)
        .map_err(|_| "invalid discord request signature encoding".to_string())?;
    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| "invalid discord request signature length".to_string())?;
    let parsed_signature = Ed25519Signature::from_bytes(&signature_array);

    let signed_payload = format!("{timestamp}{request_body}");
    verifying_key
        .verify(signed_payload.as_bytes(), &parsed_signature)
        .map_err(|_| "invalid discord request signature".to_string())?;

    Ok(())
}

pub fn verify_telegram_webhook_secret(
    provided_secret_token: Option<&str>,
    configured_secret_token: Option<&str>,
) -> Result<(), String> {
    let expected = match configured_secret_token
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(value) => value,
        None => return Ok(()),
    };

    let provided = provided_secret_token
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing telegram webhook secret token".to_string())?;

    if provided != expected {
        return Err("invalid telegram webhook secret token".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

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

    #[test]
    fn discord_signature_rejects_mismatch() {
        let signing_key = SigningKey::from_bytes(&[3u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let public_key_hex = hex::encode(verifying_key.as_bytes());
        let mismatched_signature = "00".repeat(64);
        let err = verify_discord_request_signature(
            "{\"type\":1}",
            Some(mismatched_signature.as_str()),
            Some("1700000000"),
            Some(public_key_hex.as_str()),
            1700000010,
            300,
        )
        .expect_err("mismatched signature should fail");

        assert!(err.contains("invalid discord request signature"));
    }

    #[test]
    fn discord_signature_accepts_valid_signature() {
        let signing_key = SigningKey::from_bytes(&[7u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let public_key_hex = hex::encode(verifying_key.as_bytes());
        let body = "{\"type\":1}";
        let timestamp = "1700000000";
        let signed_payload = format!("{timestamp}{body}");
        let signature = signing_key.sign(signed_payload.as_bytes());
        let signature_hex = hex::encode(signature.to_bytes());

        verify_discord_request_signature(
            body,
            Some(signature_hex.as_str()),
            Some(timestamp),
            Some(public_key_hex.as_str()),
            1700000010,
            300,
        )
        .expect("valid discord signature should pass");
    }

    #[test]
    fn discord_signature_rejects_stale_timestamp() {
        let signing_key = SigningKey::from_bytes(&[9u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let public_key_hex = hex::encode(verifying_key.as_bytes());
        let body = "{\"type\":1}";
        let timestamp = "1700000000";
        let signed_payload = format!("{timestamp}{body}");
        let signature = signing_key.sign(signed_payload.as_bytes());
        let signature_hex = hex::encode(signature.to_bytes());

        let err = verify_discord_request_signature(
            body,
            Some(signature_hex.as_str()),
            Some(timestamp),
            Some(public_key_hex.as_str()),
            1700000900,
            300,
        )
        .expect_err("stale discord signature should fail");

        assert!(err.contains("stale discord request timestamp"));
    }

    #[test]
    fn discord_signature_allows_when_public_key_not_configured() {
        verify_discord_request_signature("{}", None, None, None, 1700000000, 300)
            .expect("discord signature verification should be disabled");
    }

    #[test]
    fn telegram_signature_rejects_mismatch() {
        let err = verify_telegram_webhook_secret(
            Some("wrong-token"),
            Some("expected-token"),
        )
        .expect_err("mismatched telegram secret should fail");

        assert!(err.contains("invalid telegram webhook secret token"));
    }

    #[test]
    fn telegram_signature_rejects_missing_secret() {
        let err = verify_telegram_webhook_secret(None, Some("expected-token"))
            .expect_err("missing telegram secret token should fail");

        assert!(err.contains("missing telegram webhook secret token"));
    }

    #[test]
    fn telegram_signature_accepts_matching_secret() {
        verify_telegram_webhook_secret(Some("expected-token"), Some("expected-token"))
            .expect("matching telegram secret token should pass");
    }

    #[test]
    fn telegram_signature_allows_when_secret_not_configured() {
        verify_telegram_webhook_secret(None, None)
            .expect("telegram secret verification should be disabled");
    }
}
