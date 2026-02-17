use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulProfile {
    pub name: String,
    pub role: String,
    pub personality: String,
    pub communication_style: String,
    pub avatar: String,
    pub system_prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct SoulFrontmatter {
    name: String,
    role: String,
    personality: String,
    communication_style: String,
    avatar: String,
}

pub fn parse_soul_markdown(input: &str) -> Result<SoulProfile, String> {
    let mut lines = input.lines();
    let Some(first) = lines.next() else {
        return Err("soul markdown is empty".to_string());
    };
    if first.trim() != "---" {
        return Err("missing soul frontmatter opening delimiter".to_string());
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
        return Err("missing soul frontmatter closing delimiter".to_string());
    }

    for line in lines {
        body_lines.push(line);
    }

    let frontmatter_yaml = frontmatter_lines.join("\n");
    let frontmatter: SoulFrontmatter =
        serde_yaml::from_str(&frontmatter_yaml).map_err(|e| format!("invalid frontmatter: {e}"))?;

    Ok(SoulProfile {
        name: frontmatter.name,
        role: frontmatter.role,
        personality: frontmatter.personality,
        communication_style: frontmatter.communication_style,
        avatar: frontmatter.avatar,
        system_prompt: body_lines.join("\n"),
    })
}

pub fn load_soul_file(path: &Path) -> Result<SoulProfile, String> {
    let markdown = fs::read_to_string(path)
        .map_err(|e| format!("failed to read soul profile {}: {e}", path.display()))?;
    parse_soul_markdown(&markdown)
}

pub fn preset_soul_paths() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../souls/presets");
    let mut out = Vec::new();

    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                out.push(path);
            }
        }
    }

    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{load_soul_file, parse_soul_markdown, preset_soul_paths};

    #[test]
    fn parses_valid_soul_markdown() {
        let input = r#"---
name: Remy
role: SRE
personality: Calm and analytical
communication_style: concise
avatar: remy.png
---
You are an SRE specialist focused on reliability and clear escalation notes.
"#;

        let soul = parse_soul_markdown(input).expect("soul should parse");
        assert_eq!(soul.name, "Remy".to_string());
        assert_eq!(soul.role, "SRE".to_string());
        assert!(soul.system_prompt.contains("reliability"));
    }

    #[test]
    fn rejects_missing_soul_frontmatter() {
        let err = parse_soul_markdown("plain text").expect_err("must fail");
        assert!(err.contains("frontmatter"));
    }

    #[test]
    fn loads_soul_profile_from_file() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time is valid")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("opsclaw-soul-{unique}.md"));
        fs::write(
            &path,
            r#"---
name: Custom
role: Operator
personality: Practical
communication_style: concise
avatar: custom.png
---
Custom system prompt text.
"#,
        )
        .expect("temp soul file should be created");

        let soul = load_soul_file(&path).expect("soul profile should load");
        assert_eq!(soul.name, "Custom");
        assert_eq!(soul.role, "Operator");
        assert!(soul.system_prompt.contains("Custom system prompt"));

        fs::remove_file(path).expect("temp soul file should be removed");
    }

    #[test]
    fn preset_souls_are_discoverable() {
        let paths: Vec<PathBuf> = preset_soul_paths();
        let names: HashSet<String> = paths
            .iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
            .map(|s| s.to_string())
            .collect();

        assert!(names.contains("remy.md"));
        assert!(names.contains("ferris.md"));
        assert!(names.contains("wren.md"));
    }
}
