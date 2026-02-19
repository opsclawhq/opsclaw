pub fn parse_retry_after_seconds_from_error(error: &str) -> Option<u64> {
    let marker = "retry_after_seconds=";
    let marker_index = error.find(marker)?;
    let value_start = marker_index + marker.len();
    let digits: String = error[value_start..]
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();

    if digits.is_empty() {
        return None;
    }

    digits.parse::<u64>().ok()
}

pub fn is_retryable_live_error(error: &str) -> bool {
    let normalized = error.to_ascii_lowercase();

    normalized.contains("status=429")
        || normalized.contains("rate_limited")
        || normalized.contains("request failed")
        || normalized.contains("timed out")
        || normalized.contains("timeout")
        || normalized.contains("connection")
        || normalized.contains("temporarily unavailable")
}

pub fn compute_retry_backoff_millis(retry_index: u32, base_backoff_millis: u64, error: &str) -> u64 {
    if let Some(retry_after_seconds) = parse_retry_after_seconds_from_error(error) {
        return retry_after_seconds.saturating_mul(1000);
    }

    let bounded_index = retry_index.max(1);
    let mut delay = base_backoff_millis;
    for _ in 1..bounded_index {
        delay = delay.saturating_mul(2);
    }
    delay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_retry_after_seconds_from_error() {
        let value = parse_retry_after_seconds_from_error(
            "slack chat.postMessage request failed: status=429 retry_after_seconds=30",
        );
        assert_eq!(value, Some(30));
    }

    #[test]
    fn retry_after_parser_returns_none_when_marker_missing() {
        let value = parse_retry_after_seconds_from_error("request failed: status=429");
        assert_eq!(value, None);
    }

    #[test]
    fn classifies_retryable_errors() {
        assert!(is_retryable_live_error("status=429 retry_after_seconds=5"));
        assert!(is_retryable_live_error("slack rate_limited"));
        assert!(is_retryable_live_error("discord channel message request failed: timed out"));
    }

    #[test]
    fn classifies_non_retryable_errors() {
        assert!(!is_retryable_live_error(
            "discord live-event requires `channel_id` in payload"
        ));
        assert!(!is_retryable_live_error("invalid telegram webhook secret token"));
    }

    #[test]
    fn backoff_prefers_retry_after_when_present() {
        let delay = compute_retry_backoff_millis(
            3,
            250,
            "status=429 retry_after_seconds=7",
        );
        assert_eq!(delay, 7000);
    }

    #[test]
    fn backoff_uses_exponential_schedule() {
        assert_eq!(compute_retry_backoff_millis(1, 250, "request failed"), 250);
        assert_eq!(compute_retry_backoff_millis(2, 250, "request failed"), 500);
        assert_eq!(compute_retry_backoff_millis(3, 250, "request failed"), 1000);
    }
}
