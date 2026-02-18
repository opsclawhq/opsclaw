use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentContext {
    pub run_id: String,
    pub actor: String,
    pub intent: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentMessage {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    async fn respond(
        &self,
        context: AgentContext,
        messages: Vec<AgentMessage>,
    ) -> Result<AgentMessage, String>;
}

#[cfg(test)]
mod tests {
    use super::{AgentContext, AgentMessage};

    #[test]
    fn context_and_message_are_constructible() {
        let ctx = AgentContext {
            run_id: "run-1".to_string(),
            actor: "human".to_string(),
            intent: "triage".to_string(),
        };
        let msg = AgentMessage {
            role: "user".to_string(),
            content: "check pod restarts".to_string(),
        };

        assert_eq!(ctx.actor, "human");
        assert_eq!(msg.role, "user");
    }
}
