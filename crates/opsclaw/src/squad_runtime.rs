use crate::channels_router::{route_platform_event, ChannelPlatform, ChannelRouteDecision};
use crate::squad_responder::response_for_input;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RuntimeInboundEvent {
    pub platform: String,
    pub payload_json: String,
    pub identity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeOutboundEvent {
    pub platform: String,
    pub target_ref: String,
    pub route_kind: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeLoopOutcome {
    pub events_processed: usize,
    pub responses_emitted: usize,
}

pub fn process_inbound_event(
    platform: &str,
    payload_json: &str,
    identity: Option<&str>,
    template: &str,
) -> Result<Option<RuntimeOutboundEvent>, String> {
    let parsed_platform = ChannelPlatform::parse(platform)?;
    let decision = route_platform_event(parsed_platform, payload_json, identity)?;

    match decision {
        ChannelRouteDecision::Ignore => Ok(None),
        ChannelRouteDecision::Routed(route) => {
            let text = response_for_input(template, route.text.as_str());

            Ok(Some(RuntimeOutboundEvent {
                platform: route.platform,
                target_ref: route.target_ref,
                route_kind: route.route_kind,
                text,
            }))
        }
    }
}

pub fn run_stdio_loop(
    mut reader: impl BufRead,
    writer: &mut impl Write,
    template: &str,
    max_events: Option<usize>,
) -> Result<RuntimeLoopOutcome, String> {
    let mut buffer = String::new();
    let mut events_processed = 0usize;
    let mut responses_emitted = 0usize;

    loop {
        if let Some(limit) = max_events {
            if events_processed >= limit {
                break;
            }
        }

        buffer.clear();
        let bytes = reader
            .read_line(&mut buffer)
            .map_err(|err| format!("failed reading stdio input: {err}"))?;

        if bytes == 0 {
            break;
        }

        let trimmed = buffer.trim();
        if trimmed.is_empty() {
            continue;
        }

        let inbound: RuntimeInboundEvent = serde_json::from_str(trimmed)
            .map_err(|err| format!("invalid runtime inbound JSON: {err}"))?;

        events_processed += 1;

        if let Some(reply) = process_inbound_event(
            inbound.platform.as_str(),
            inbound.payload_json.as_str(),
            inbound.identity.as_deref(),
            template,
        )? {
            let json = serde_json::to_string(&reply)
                .map_err(|err| format!("failed serializing runtime outbound event: {err}"))?;
            writer
                .write_all(format!("{json}\n").as_bytes())
                .map_err(|err| format!("failed writing runtime outbound event: {err}"))?;
            responses_emitted += 1;
        }
    }

    Ok(RuntimeLoopOutcome {
        events_processed,
        responses_emitted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parity_for_squad_command_across_platforms() {
        let telegram = process_inbound_event(
            "telegram",
            r#"{"message":{"chat":{"id":7,"type":"private"},"text":"/squad"}}"#,
            Some("opsclaw_bot"),
            "sre-squad",
        )
        .expect("telegram route should parse")
        .expect("telegram should route");

        let discord = process_inbound_event(
            "discord",
            r#"{"type":2,"data":{"name":"squad"},"member":{"roles":["ops"]}}"#,
            None,
            "sre-squad",
        )
        .expect("discord route should parse")
        .expect("discord should route");

        assert_eq!(telegram.text, discord.text);
    }

    #[test]
    fn non_routable_event_returns_none() {
        let ignored = process_inbound_event(
            "slack",
            r#"{"type":"event_callback","event":{"type":"message","text":"hello","channel":"C1"}}"#,
            Some("UOPS"),
            "sre-squad",
        )
        .expect("route should parse");

        assert!(ignored.is_none());
    }

    #[test]
    fn stdio_loop_emits_only_routed_responses() {
        let input = r#"{"platform":"telegram","payload_json":"{\"message\":{\"chat\":{\"id\":7,\"type\":\"private\"},\"text\":\"/squad\"}}","identity":"opsclaw_bot"}
{"platform":"slack","payload_json":"{\"type\":\"event_callback\",\"event\":{\"type\":\"message\",\"text\":\"hello\",\"channel\":\"C1\"}}","identity":"UOPS"}
"#;
        let reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        let outcome = run_stdio_loop(reader, &mut output, "sre-squad", None)
            .expect("stdio loop should succeed");

        assert_eq!(outcome.events_processed, 2);
        assert_eq!(outcome.responses_emitted, 1);

        let lines = String::from_utf8(output).expect("output should be utf8");
        assert!(lines.contains("\"platform\":\"telegram\""));
        assert!(lines.contains("Active SRE squad"));
    }
}
