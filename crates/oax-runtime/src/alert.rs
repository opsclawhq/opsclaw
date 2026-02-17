use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertPayload {
    PagerDuty {
        incident_id: String,
        severity: String,
        summary: String,
    },
    Prometheus {
        alertname: String,
        severity: String,
        summary: String,
    },
}

pub fn parse_alert_payload(input: &str) -> Result<AlertPayload, String> {
    let value: Value =
        serde_json::from_str(input).map_err(|e| format!("invalid json payload: {e}"))?;

    if let Some(pd) = value.get("data").and_then(|v| v.get("incident")) {
        let incident_id = pd
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing data.incident.id".to_string())?
            .to_string();
        let severity = pd
            .get("urgency")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        let summary = pd
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("pagerduty incident")
            .to_string();

        return Ok(AlertPayload::PagerDuty {
            incident_id,
            severity,
            summary,
        });
    }

    if let Some(alerts) = value.get("alerts").and_then(Value::as_array) {
        let first = alerts
            .first()
            .ok_or_else(|| "alerts array is empty".to_string())?;
        let labels = first
            .get("labels")
            .and_then(Value::as_object)
            .ok_or_else(|| "missing alerts[0].labels".to_string())?;
        let annotations = first.get("annotations").and_then(Value::as_object);

        let alertname = labels
            .get("alertname")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing labels.alertname".to_string())?
            .to_string();
        let severity = labels
            .get("severity")
            .and_then(Value::as_str)
            .or_else(|| value.get("status").and_then(Value::as_str))
            .unwrap_or("unknown")
            .to_string();
        let summary = annotations
            .and_then(|a| a.get("summary"))
            .and_then(Value::as_str)
            .unwrap_or("prometheus alert")
            .to_string();

        return Ok(AlertPayload::Prometheus {
            alertname,
            severity,
            summary,
        });
    }

    Err("unsupported alert payload shape".to_string())
}

#[cfg(test)]
mod tests {
    use super::{parse_alert_payload, AlertPayload};

    #[test]
    fn parses_pagerduty_payload() {
        let payload = r#"{
          "data": {
            "incident": {
              "id": "P123",
              "urgency": "high",
              "title": "Database CPU saturation"
            }
          }
        }"#;

        let parsed = parse_alert_payload(payload).expect("pagerduty payload should parse");
        assert_eq!(
            parsed,
            AlertPayload::PagerDuty {
                incident_id: "P123".to_string(),
                severity: "high".to_string(),
                summary: "Database CPU saturation".to_string(),
            }
        );
    }

    #[test]
    fn parses_prometheus_payload() {
        let payload = r#"{
          "alerts": [
            {
              "labels": {"alertname": "PodCrashLooping", "severity": "critical"},
              "annotations": {"summary": "pod api-1 crashlooping"}
            }
          ]
        }"#;

        let parsed = parse_alert_payload(payload).expect("prometheus payload should parse");
        assert_eq!(
            parsed,
            AlertPayload::Prometheus {
                alertname: "PodCrashLooping".to_string(),
                severity: "critical".to_string(),
                summary: "pod api-1 crashlooping".to_string(),
            }
        );
    }

    #[test]
    fn parses_prometheus_payload_without_annotations() {
        let payload = r#"{
          "alerts": [
            {
              "labels": {"alertname": "HighMemoryUsage", "severity": "warning"}
            }
          ]
        }"#;

        let parsed = parse_alert_payload(payload).expect("prometheus payload should parse");
        assert_eq!(
            parsed,
            AlertPayload::Prometheus {
                alertname: "HighMemoryUsage".to_string(),
                severity: "warning".to_string(),
                summary: "prometheus alert".to_string(),
            }
        );
    }

    #[test]
    fn falls_back_to_prometheus_status_for_severity() {
        let payload = r#"{
          "status": "firing",
          "alerts": [
            {
              "labels": {"alertname": "DiskPressure"}
            }
          ]
        }"#;

        let parsed = parse_alert_payload(payload).expect("prometheus payload should parse");
        assert_eq!(
            parsed,
            AlertPayload::Prometheus {
                alertname: "DiskPressure".to_string(),
                severity: "firing".to_string(),
                summary: "prometheus alert".to_string(),
            }
        );
    }

    #[test]
    fn rejects_unknown_shape() {
        let payload = r#"{"hello":"world"}"#;
        let err = parse_alert_payload(payload).expect_err("unknown shape should error");
        assert!(err.contains("unsupported"));
    }
}
