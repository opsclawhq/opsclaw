use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelInput {
    pub prompt: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelOutput {
    pub text: String,
}

#[async_trait]
pub trait Model: Send + Sync {
    fn provider(&self) -> &str;
    fn name(&self) -> &str;
    async fn generate(&self, input: ModelInput) -> Result<ModelOutput, String>;
}

#[cfg(test)]
mod tests {
    use super::{ModelInput, ModelOutput};

    #[test]
    fn model_types_hold_prompt_and_text() {
        let input = ModelInput {
            prompt: "diagnose cpu spike".to_string(),
        };
        let output = ModelOutput {
            text: "checking cpu metrics".to_string(),
        };

        assert_eq!(input.prompt, "diagnose cpu spike");
        assert_eq!(output.text, "checking cpu metrics");
    }
}
