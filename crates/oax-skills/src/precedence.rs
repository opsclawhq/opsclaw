use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillSource {
    Bundled,
    Global,
    Workspace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSkill {
    pub source: SkillSource,
    pub path: PathBuf,
    pub content: String,
}

pub fn resolve_skill_catalog(
    bundled_root: impl AsRef<Path>,
    global_root: impl AsRef<Path>,
    workspace_root: impl AsRef<Path>,
) -> std::io::Result<HashMap<String, ResolvedSkill>> {
    let mut catalog = HashMap::new();

    scan_root(bundled_root.as_ref(), SkillSource::Bundled, &mut catalog)?;
    scan_root(global_root.as_ref(), SkillSource::Global, &mut catalog)?;
    scan_root(workspace_root.as_ref(), SkillSource::Workspace, &mut catalog)?;

    Ok(catalog)
}

fn scan_root(
    root: &Path,
    source: SkillSource,
    catalog: &mut HashMap<String, ResolvedSkill>,
) -> std::io::Result<()> {
    if !root.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let content = fs::read_to_string(&path)?;

        catalog.insert(
            stem.to_string(),
            ResolvedSkill {
                source,
                path,
                content,
            },
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{resolve_skill_catalog, SkillSource};

    #[test]
    fn workspace_overrides_global_and_bundled() {
        let dir = tempdir().expect("tempdir");
        let bundled = dir.path().join("bundled");
        let global = dir.path().join("global");
        let workspace = dir.path().join("workspace");
        fs::create_dir_all(&bundled).expect("bundled dir");
        fs::create_dir_all(&global).expect("global dir");
        fs::create_dir_all(&workspace).expect("workspace dir");

        fs::write(bundled.join("kube-debugger.md"), "bundled").expect("write bundled");
        fs::write(global.join("kube-debugger.md"), "global").expect("write global");
        fs::write(workspace.join("kube-debugger.md"), "workspace").expect("write workspace");

        let catalog = resolve_skill_catalog(&bundled, &global, &workspace).expect("catalog");
        let entry = catalog
            .get("kube-debugger")
            .expect("skill should exist");
        assert_eq!(entry.source, SkillSource::Workspace);
        assert_eq!(entry.content, "workspace".to_string());
    }

    #[test]
    fn global_overrides_bundled_when_workspace_missing() {
        let dir = tempdir().expect("tempdir");
        let bundled = dir.path().join("bundled");
        let global = dir.path().join("global");
        let workspace = dir.path().join("workspace");
        fs::create_dir_all(&bundled).expect("bundled dir");
        fs::create_dir_all(&global).expect("global dir");
        fs::create_dir_all(&workspace).expect("workspace dir");

        fs::write(bundled.join("incident-responder.md"), "bundled").expect("write bundled");
        fs::write(global.join("incident-responder.md"), "global").expect("write global");

        let catalog = resolve_skill_catalog(&bundled, &global, &workspace).expect("catalog");
        let entry = catalog
            .get("incident-responder")
            .expect("skill should exist");
        assert_eq!(entry.source, SkillSource::Global);
        assert_eq!(entry.content, "global".to_string());
    }

    #[test]
    fn missing_roots_return_empty_catalog() {
        let dir = tempdir().expect("tempdir");
        let catalog = resolve_skill_catalog(
            dir.path().join("missing-bundled"),
            dir.path().join("missing-global"),
            dir.path().join("missing-workspace"),
        )
        .expect("catalog");
        assert!(catalog.is_empty());
    }
}
