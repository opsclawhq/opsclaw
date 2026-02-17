use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SkillRiskClass {
    Read,
    SafeWrite,
    Destructive,
    Forbidden,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub required_bins: Vec<String>,
    pub risk: SkillRiskClass,
    pub trust: Option<String>,
    pub rollback_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillDocument {
    pub frontmatter: SkillFrontmatter,
    pub body: String,
}

pub fn parse_skill_markdown(input: &str) -> Result<SkillDocument, String> {
    let mut lines = input.lines();
    let Some(first) = lines.next() else {
        return Err("skill markdown is empty".to_string());
    };
    if first.trim() != "---" {
        return Err("missing skill frontmatter opening delimiter".to_string());
    }

    let mut frontmatter_lines = Vec::new();
    let mut found_closing = false;
    let mut body_lines = Vec::new();

    for line in lines.by_ref() {
        if line.trim() == "---" {
            found_closing = true;
            break;
        }
        frontmatter_lines.push(line);
    }

    if !found_closing {
        return Err("missing skill frontmatter closing delimiter".to_string());
    }

    for line in lines {
        body_lines.push(line);
    }

    let frontmatter_yaml = frontmatter_lines.join("\n");
    let frontmatter: SkillFrontmatter =
        serde_yaml::from_str(&frontmatter_yaml).map_err(|e| format!("invalid frontmatter: {e}"))?;

    Ok(SkillDocument {
        frontmatter,
        body: body_lines.join("\n"),
    })
}

pub fn validate_required_bins(frontmatter: &SkillFrontmatter) -> Vec<String> {
    frontmatter
        .required_bins
        .iter()
        .filter(|bin| which::which(bin).is_err())
        .cloned()
        .collect()
}

pub fn validate_install_policy(frontmatter: &SkillFrontmatter) -> Result<(), String> {
    if frontmatter
        .trust
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        return Err("missing trust field".to_string());
    }

    if frontmatter.risk == SkillRiskClass::Destructive
        && frontmatter
            .rollback_template
            .as_ref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
    {
        return Err("destructive skills require rollback_template".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        parse_skill_markdown, validate_install_policy, validate_required_bins, SkillRiskClass,
    };

    #[test]
    fn parses_valid_skill_markdown() {
        let input = r#"---
name: kube-debugger
description: Diagnose pods
required_bins: [kubectl]
risk: READ
trust: verified
---
Use `kubectl describe pod` and summarize findings.
"#;

        let doc = parse_skill_markdown(input).expect("skill should parse");
        assert_eq!(doc.frontmatter.name, "kube-debugger".to_string());
        assert_eq!(doc.frontmatter.risk, SkillRiskClass::Read);
        assert!(doc.body.contains("summarize findings"));
    }

    #[test]
    fn rejects_missing_frontmatter() {
        let err = parse_skill_markdown("no frontmatter").expect_err("must fail");
        assert!(err.contains("frontmatter"));
    }

    #[test]
    fn reports_missing_required_bins() {
        let input = r#"---
name: missing-bin-skill
description: test
required_bins: [definitely-not-a-real-binary-opsclaw]
risk: READ
trust: verified
---
body
"#;
        let doc = parse_skill_markdown(input).expect("parses");
        let missing = validate_required_bins(&doc.frontmatter);
        assert_eq!(missing, vec!["definitely-not-a-real-binary-opsclaw".to_string()]);
    }

    #[test]
    fn rejects_destructive_without_rollback_or_trust() {
        let input = r#"---
name: destroy-all
description: test
required_bins: [kubectl]
risk: DESTRUCTIVE
---
body
"#;
        let doc = parse_skill_markdown(input).expect("parses");
        let err = validate_install_policy(&doc.frontmatter).expect_err("must reject");
        assert!(err.contains("trust"));
    }

    #[test]
    fn accepts_destructive_with_trust_and_rollback() {
        let input = r#"---
name: controlled-restart
description: restart rollout
required_bins: [kubectl]
risk: DESTRUCTIVE
trust: verified
rollback_template: kubectl rollout undo deploy/api
---
body
"#;
        let doc = parse_skill_markdown(input).expect("parses");
        validate_install_policy(&doc.frontmatter).expect("policy should pass");
    }
}
