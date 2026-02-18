use oax_skills::{parse_skill_markdown, validate_install_policy, validate_required_bins};
use std::fs;
use std::path::{Path, PathBuf};

pub fn install_skill_to_default_location(source: &Path) -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME is not set".to_string())?;
    let destination_root = PathBuf::from(home).join(".opsclaw/skills");
    install_skill_from_file(source, &destination_root)
}

pub fn install_skill_from_file(source: &Path, destination_root: &Path) -> Result<PathBuf, String> {
    let content = fs::read_to_string(source)
        .map_err(|e| format!("failed to read skill file {}: {e}", source.display()))?;
    let skill = parse_skill_markdown(&content)?;

    validate_install_policy(&skill.frontmatter)?;
    let missing_bins = validate_required_bins(&skill.frontmatter);
    if !missing_bins.is_empty() {
        return Err(format!(
            "missing required bins: {}",
            missing_bins.join(",")
        ));
    }

    fs::create_dir_all(destination_root)
        .map_err(|e| format!("failed to create destination directory: {e}"))?;

    let destination = destination_root.join(format!("{}.md", skill.frontmatter.name));
    fs::write(&destination, content)
        .map_err(|e| format!("failed to write installed skill: {e}"))?;
    Ok(destination)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::install_skill_from_file;

    #[test]
    fn installs_valid_skill_markdown() {
        let dir = tempdir().expect("tempdir");
        let src = dir.path().join("source.md");
        let dest = dir.path().join("skills");
        fs::write(
            &src,
            r#"---
name: kube-debugger
description: Diagnose k8s pods
required_bins: []
risk: READ
trust: verified
---
body
"#,
        )
        .expect("write src");

        let out = install_skill_from_file(&src, &dest).expect("install should pass");
        assert!(out.exists());
        assert!(out.ends_with("kube-debugger.md"));
    }

    #[test]
    fn rejects_skill_without_trust() {
        let dir = tempdir().expect("tempdir");
        let src = dir.path().join("source.md");
        let dest = dir.path().join("skills");
        fs::write(
            &src,
            r#"---
name: unsafe-skill
description: no trust
required_bins: []
risk: READ
---
body
"#,
        )
        .expect("write src");

        let err = install_skill_from_file(&src, &dest).expect_err("install must fail");
        assert!(err.contains("trust"));
    }

    #[test]
    fn rejects_skill_with_missing_required_binary() {
        let dir = tempdir().expect("tempdir");
        let src = dir.path().join("source.md");
        let dest = dir.path().join("skills");
        fs::write(
            &src,
            r#"---
name: missing-bin
description: missing binary
required_bins: [definitely-not-a-real-binary-opsclaw]
risk: READ
trust: verified
---
body
"#,
        )
        .expect("write src");

        let err = install_skill_from_file(&src, &dest).expect_err("install must fail");
        assert!(err.contains("required bins"));
    }
}
