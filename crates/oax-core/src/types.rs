use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KpiSnapshot {
    pub activation: f64,
    pub ttfv_minutes: f64,
    pub reliability_success_rate: f64,
    pub retention_d30: f64,
    pub enterprise_pilots: f64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanTraceRef {
    pub phase: u8,
    pub plan_id: String,
    pub branch: String,
    pub pr: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IpcEnvelope {
    pub schema_version: String,
    pub message_type: String,
    pub run_id: Option<String>,
    pub payload_json: String,
    pub ok: Option<bool>,
    pub error: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DashboardAgentStatus {
    Idle,
    InProgress,
    WaitingApproval,
    Error,
    Offline,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardAgentSummary {
    pub agent_id: String,
    pub name: String,
    pub role: String,
    pub status: DashboardAgentStatus,
    pub soul_profile: String,
    pub skills: Vec<String>,
    pub token_budget_remaining: u32,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DashboardTaskStage {
    Inbox,
    Assigned,
    InProgress,
    Review,
    Done,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardTaskCard {
    pub task_id: String,
    pub title: String,
    pub stage: DashboardTaskStage,
    pub assignee_agent_id: Option<String>,
    pub priority: String,
    pub updated_at: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardKanbanColumn {
    pub stage: DashboardTaskStage,
    pub tasks: Vec<DashboardTaskCard>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardKanbanSnapshot {
    pub generated_at: String,
    pub columns: Vec<DashboardKanbanColumn>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardActivityItem {
    pub event_id: String,
    pub agent_id: String,
    pub event_type: String,
    pub summary: String,
    pub occurred_at: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardApprovalRequest {
    pub approval_id: String,
    pub run_id: String,
    pub command: String,
    pub blast_radius: String,
    pub rollback_steps: String,
    pub requested_by: String,
    pub requested_at: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DashboardEventType {
    AgentUpdated,
    KanbanUpdated,
    ActivityUpdated,
    ApprovalUpdated,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardStreamEvent {
    pub event_type: DashboardEventType,
    pub occurred_at: String,
    pub payload_json: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_stream_event_roundtrip_json() {
        let event = DashboardStreamEvent {
            event_type: DashboardEventType::ActivityUpdated,
            occurred_at: "2026-02-18T01:00:00Z".to_string(),
            payload_json: "{\"id\":\"evt-1\"}".to_string(),
        };

        let json = serde_json::to_string(&event).expect("event should serialize");
        let parsed: DashboardStreamEvent =
            serde_json::from_str(json.as_str()).expect("event should deserialize");
        assert_eq!(parsed, event);
    }
}
