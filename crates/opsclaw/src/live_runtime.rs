use crate::squad_runtime::RuntimeInboundEvent;
use serde::Serialize;
use std::io::{BufRead, Write};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LiveRuntimeLoopOutcome {
    pub events_processed: usize,
    pub decisions_emitted: usize,
}

pub fn run_live_stdio_loop(
    mut reader: impl BufRead,
    writer: &mut impl Write,
    max_events: Option<usize>,
    mut dispatch: impl FnMut(&RuntimeInboundEvent) -> Result<Option<serde_json::Value>, String>,
) -> Result<LiveRuntimeLoopOutcome, String> {
    let mut buffer = String::new();
    let mut events_processed = 0usize;
    let mut decisions_emitted = 0usize;

    loop {
        if let Some(limit) = max_events {
            if events_processed >= limit {
                break;
            }
        }

        buffer.clear();
        let bytes = reader
            .read_line(&mut buffer)
            .map_err(|err| format!("failed reading live stdio input: {err}"))?;

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

        if let Some(decision) = dispatch(&inbound)? {
            let json = serde_json::to_string(&decision)
                .map_err(|err| format!("failed serializing live runtime decision: {err}"))?;
            writer
                .write_all(format!("{json}\n").as_bytes())
                .map_err(|err| format!("failed writing live runtime decision: {err}"))?;
            decisions_emitted += 1;
        }
    }

    Ok(LiveRuntimeLoopOutcome {
        events_processed,
        decisions_emitted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn stdio_loop_emits_decisions() {
        let input = "{\"platform\":\"slack\",\"payload_json\":\"{}\",\"identity\":\"U_BOT\"}\n\
{\"platform\":\"discord\",\"payload_json\":\"{}\",\"identity\":null}\n";
        let reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        let outcome = run_live_stdio_loop(reader, &mut output, None, |_event| {
            Ok(Some(serde_json::json!({"decision":"ignore"})))
        })
        .expect("loop should succeed");

        assert_eq!(outcome.events_processed, 2);
        assert_eq!(outcome.decisions_emitted, 2);

        let lines = String::from_utf8(output).expect("output should be utf8");
        assert!(lines.contains("\"decision\":\"ignore\""));
    }

    #[test]
    fn stdio_loop_skips_empty_lines() {
        let input = "\n\n{\"platform\":\"discord\",\"payload_json\":\"{}\",\"identity\":null}\n";
        let reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        let outcome = run_live_stdio_loop(reader, &mut output, None, |_event| {
            Ok(Some(serde_json::json!({"decision":"ignore"})))
        })
        .expect("loop should succeed");

        assert_eq!(outcome.events_processed, 1);
        assert_eq!(outcome.decisions_emitted, 1);
    }

    #[test]
    fn stdio_loop_rejects_invalid_json() {
        let input = "not-json\n";
        let reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        let err = run_live_stdio_loop(reader, &mut output, None, |_event| {
            Ok(Some(serde_json::json!({"decision":"ignore"})))
        })
        .expect_err("invalid json should fail");

        assert!(err.contains("invalid runtime inbound JSON"));
    }
}
