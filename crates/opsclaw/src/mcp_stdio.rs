use oax_runtime::mcp::{evaluate_mcp_call, opsclaw_mcp_tools, McpCallDecision};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Debug, Deserialize)]
struct McpRequest {
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Deserialize)]
struct ToolCallParams {
    tool_name: String,
    command: String,
    rollback_template: Option<String>,
}

#[derive(Debug, Serialize)]
struct McpResponse {
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub fn handle_mcp_request_line(input: &str) -> String {
    let parsed = match serde_json::from_str::<McpRequest>(input) {
        Ok(req) => req,
        Err(err) => {
            return serde_json::to_string(&McpResponse {
                id: None,
                result: None,
                error: Some(format!("invalid request: {err}")),
            })
            .unwrap_or_else(|_| {
                "{\"id\":null,\"error\":\"invalid request: serialization failure\"}".to_string()
            });
        }
    };

    let response = match parsed.method.as_str() {
        "tools/list" => {
            let tools = opsclaw_mcp_tools()
                .into_iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "risk_class": format!("{:?}", t.risk_class).to_lowercase(),
                    })
                })
                .collect::<Vec<Value>>();

            McpResponse {
                id: parsed.id,
                result: Some(json!({ "tools": tools })),
                error: None,
            }
        }
        "tools/call" => {
            let params = match serde_json::from_value::<ToolCallParams>(parsed.params) {
                Ok(params) => params,
                Err(err) => {
                    return serde_json::to_string(&McpResponse {
                        id: parsed.id,
                        result: None,
                        error: Some(format!("invalid tools/call params: {err}")),
                    })
                    .unwrap_or_else(|_| {
                        "{\"id\":null,\"error\":\"invalid tools/call params\"}".to_string()
                    });
                }
            };

            let decision = evaluate_mcp_call(
                params.tool_name.as_str(),
                params.command.as_str(),
                params.rollback_template.as_deref(),
            );
            let result = match decision {
                McpCallDecision::AllowReadOnly => {
                    json!({ "decision": "allow_read_only" })
                }
                McpCallDecision::RequireApproval(card) => json!({
                    "decision": "require_approval",
                    "approval_card": {
                        "command": card.command,
                        "expected_effect": card.expected_effect,
                        "blast_radius": card.blast_radius,
                        "rollback_steps": card.rollback_steps
                    }
                }),
                McpCallDecision::Forbidden(reason) => {
                    json!({ "decision": "forbidden", "reason": reason })
                }
                McpCallDecision::UnknownTool(reason) => {
                    json!({ "decision": "unknown_tool", "reason": reason })
                }
            };

            McpResponse {
                id: parsed.id,
                result: Some(result),
                error: None,
            }
        }
        _ => McpResponse {
            id: parsed.id,
            result: None,
            error: Some(format!("unknown method '{}'", parsed.method)),
        },
    };

    serde_json::to_string(&response).unwrap_or_else(|_| {
        "{\"id\":null,\"error\":\"internal serialization failure\"}".to_string()
    })
}

pub fn serve_stdio() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let response = handle_mcp_request_line(line.as_str());
        writeln!(stdout, "{response}")?;
        stdout.flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::handle_mcp_request_line;

    #[test]
    fn handles_tools_list_request() {
        let input = r#"{"id":"1","method":"tools/list"}"#;
        let output = handle_mcp_request_line(input);

        assert!(output.contains("\"id\":\"1\""));
        assert!(output.contains("\"result\""));
        assert!(output.contains("\"shell\""));
    }

    #[test]
    fn handles_tools_call_request() {
        let input = r#"{"id":"2","method":"tools/call","params":{"tool_name":"shell","command":"kubectl get pods"}}"#;
        let output = handle_mcp_request_line(input);

        assert!(output.contains("\"id\":\"2\""));
        assert!(output.contains("\"allow_read_only\""));
    }

    #[test]
    fn handles_malformed_request_with_error_response() {
        let output = handle_mcp_request_line("not-json");

        assert!(output.contains("\"error\""));
        assert!(output.contains("invalid request"));
    }

    #[test]
    fn handles_forbidden_call_with_forbidden_decision() {
        let input = r#"{"id":"3","method":"tools/call","params":{"tool_name":"shell","command":"rm -rf /"}}"#;
        let output = handle_mcp_request_line(input);
        assert!(output.contains("\"forbidden\""));
    }
}
