use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolInput {
    pub name: String,
    pub args_json: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolResult {
    pub ok: bool,
    pub output: String,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, input: ToolInput) -> Result<ToolResult, String>;
}

#[cfg(test)]
mod tests {
    use super::{ToolInput, ToolResult};

    #[test]
    fn tool_types_hold_input_and_output() {
        let input = ToolInput {
            name: "kubectl".to_string(),
            args_json: "{\"cmd\":\"get pods\"}".to_string(),
        };
        let result = ToolResult {
            ok: true,
            output: "pod-a Running".to_string(),
        };

        assert_eq!(input.name, "kubectl");
        assert!(result.ok);
    }
}
