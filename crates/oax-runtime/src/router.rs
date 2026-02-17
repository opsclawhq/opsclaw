#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutedMessage {
    pub target: String,
    pub payload: String,
}

pub fn parse_agent_tags(input: &str) -> Vec<RoutedMessage> {
    let mut out = Vec::new();
    let mut remaining = input;

    while let Some(start) = remaining.find("[@") {
        let after_start = &remaining[start + 2..];
        let Some(colon_idx) = after_start.find(':') else {
            break;
        };
        let target = after_start[..colon_idx].trim();
        let after_colon = &after_start[colon_idx + 1..];
        let Some(end_idx) = after_colon.find(']') else {
            break;
        };
        let payload = after_colon[..end_idx].trim();

        if !target.is_empty() && !payload.is_empty() {
            out.push(RoutedMessage {
                target: target.to_string(),
                payload: payload.to_string(),
            });
        }

        remaining = &after_colon[end_idx + 1..];
    }

    out
}

#[cfg(test)]
mod tests {
    use super::{parse_agent_tags, RoutedMessage};

    #[test]
    fn parses_multiple_agent_tags_in_order() {
        let parsed = parse_agent_tags(
            "Investigating now. [@db-agent: check pg locks] and [@k8s-agent: inspect pod events]",
        );

        assert_eq!(
            parsed,
            vec![
                RoutedMessage {
                    target: "db-agent".to_string(),
                    payload: "check pg locks".to_string(),
                },
                RoutedMessage {
                    target: "k8s-agent".to_string(),
                    payload: "inspect pod events".to_string(),
                },
            ]
        );
    }

    #[test]
    fn ignores_non_tag_text() {
        let parsed = parse_agent_tags("No tags in this message.");
        assert!(parsed.is_empty());
    }
}
