use oax_core::types::IpcEnvelope;

pub const IPC_SCHEMA_VERSION: &str = "opsclaw.ipc.v1alpha1";

pub fn parse_ipc_line(line: &str) -> Result<IpcEnvelope, String> {
    serde_json::from_str::<IpcEnvelope>(line).map_err(|err| format!("invalid ipc json line: {err}"))
}

pub fn serialize_ipc_line(envelope: &IpcEnvelope) -> Result<String, String> {
    serde_json::to_string(envelope)
        .map_err(|err| format!("failed to serialize ipc response: {err}"))
}

pub fn handle_runtime_message(request: &IpcEnvelope) -> IpcEnvelope {
    if request.schema_version != IPC_SCHEMA_VERSION {
        return error_response(
            request.run_id.clone(),
            format!(
                "invalid schema_version '{}', expected '{}'",
                request.schema_version, IPC_SCHEMA_VERSION
            ),
        );
    }

    match request.message_type.as_str() {
        "runtime.ping" => ok_response(request.run_id.clone(), "runtime.pong", r#"{"status":"ok"}"#),
        "runtime.forward" => ok_response(
            request.run_id.clone(),
            "runtime.forward.ack",
            r#"{"accepted":true}"#,
        ),
        _ => error_response(
            request.run_id.clone(),
            format!(
                "unsupported runtime message_type '{}'",
                request.message_type
            ),
        ),
    }
}

pub fn handle_control_message(request: &IpcEnvelope) -> IpcEnvelope {
    if request.schema_version != IPC_SCHEMA_VERSION {
        return error_response(
            request.run_id.clone(),
            format!(
                "invalid schema_version '{}', expected '{}'",
                request.schema_version, IPC_SCHEMA_VERSION
            ),
        );
    }

    match request.message_type.as_str() {
        "control.health" => ok_response(
            request.run_id.clone(),
            "control.health.ok",
            r#"{"status":"healthy"}"#,
        ),
        "control.stop" => ok_response(
            request.run_id.clone(),
            "control.stop.ack",
            r#"{"stop":"requested"}"#,
        ),
        _ => error_response(
            request.run_id.clone(),
            format!(
                "unsupported control message_type '{}'",
                request.message_type
            ),
        ),
    }
}

pub fn malformed_line_error_response(err: &str) -> IpcEnvelope {
    error_response(None, err.to_string())
}

fn ok_response(run_id: Option<String>, message_type: &str, payload_json: &str) -> IpcEnvelope {
    IpcEnvelope {
        schema_version: IPC_SCHEMA_VERSION.to_string(),
        message_type: message_type.to_string(),
        run_id,
        payload_json: payload_json.to_string(),
        ok: Some(true),
        error: None,
    }
}

fn error_response(run_id: Option<String>, err: String) -> IpcEnvelope {
    IpcEnvelope {
        schema_version: IPC_SCHEMA_VERSION.to_string(),
        message_type: "error".to_string(),
        run_id,
        payload_json: "{}".to_string(),
        ok: Some(false),
        error: Some(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_schema_version() {
        let line = r#"{"schema_version":"opsclaw.ipc.v0","message_type":"runtime.ping","run_id":null,"payload_json":"{}","ok":null,"error":null}"#;
        let parsed = parse_ipc_line(line).expect("line should parse");
        let response = handle_runtime_message(&parsed);
        assert_eq!(response.ok, Some(false));
        assert!(response
            .error
            .as_ref()
            .expect("error should be populated")
            .contains("schema_version"));
    }

    #[test]
    fn runtime_ping_returns_ok_response() {
        let line = r#"{"schema_version":"opsclaw.ipc.v1alpha1","message_type":"runtime.ping","run_id":"run-1","payload_json":"{}","ok":null,"error":null}"#;
        let parsed = parse_ipc_line(line).expect("line should parse");
        let response = handle_runtime_message(&parsed);
        assert_eq!(response.ok, Some(true));
        assert_eq!(response.message_type, "runtime.pong");
    }

    #[test]
    fn unknown_control_message_returns_error() {
        let line = r#"{"schema_version":"opsclaw.ipc.v1alpha1","message_type":"control.unknown","run_id":null,"payload_json":"{}","ok":null,"error":null}"#;
        let parsed = parse_ipc_line(line).expect("line should parse");
        let response = handle_control_message(&parsed);
        assert_eq!(response.ok, Some(false));
        assert_eq!(response.message_type, "error");
    }

    #[test]
    fn serialize_round_trip_is_stable() {
        let envelope = IpcEnvelope {
            schema_version: IPC_SCHEMA_VERSION.to_string(),
            message_type: "control.health".to_string(),
            run_id: Some("run-42".to_string()),
            payload_json: "{}".to_string(),
            ok: None,
            error: None,
        };
        let line = serialize_ipc_line(&envelope).expect("should serialize");
        let round_trip = parse_ipc_line(&line).expect("should parse");
        assert_eq!(round_trip, envelope);
    }
}
