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
