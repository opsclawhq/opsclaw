use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct MemorySnapshot {
    agents: HashMap<String, HashMap<String, String>>,
}

pub struct JsonFileMemoryStore {
    path: PathBuf,
    snapshot: MemorySnapshot,
}

impl JsonFileMemoryStore {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let snapshot = match fs::read_to_string(&path) {
            Ok(content) if content.trim().is_empty() => MemorySnapshot::default(),
            Ok(content) => serde_json::from_str(&content).map_err(std::io::Error::other)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => MemorySnapshot::default(),
            Err(err) => return Err(err),
        };

        Ok(Self { path, snapshot })
    }

    pub fn put(&mut self, agent_id: &str, key: &str, value: &str) {
        self.snapshot
            .agents
            .entry(agent_id.to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, agent_id: &str, key: &str) -> Option<String> {
        self.snapshot
            .agents
            .get(agent_id)
            .and_then(|m| m.get(key))
            .cloned()
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let payload = serde_json::to_string(&self.snapshot).map_err(std::io::Error::other)?;
        fs::write(&self.path, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::JsonFileMemoryStore;
    use tempfile::tempdir;

    #[test]
    fn loads_empty_store_when_file_missing() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join("memory.json");
        let store = JsonFileMemoryStore::new(&path).expect("new store");
        assert_eq!(store.get("agent-a", "runbook"), None);
    }

    #[test]
    fn persists_memory_across_reloads() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join("memory.json");

        let mut store = JsonFileMemoryStore::new(&path).expect("new store");
        store.put("agent-a", "runbook", "restart-nginx");
        store.save().expect("save");

        let reloaded = JsonFileMemoryStore::new(&path).expect("reload");
        assert_eq!(
            reloaded.get("agent-a", "runbook"),
            Some("restart-nginx".to_string())
        );
    }

    #[test]
    fn overwrites_existing_memory_value() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join("memory.json");

        let mut store = JsonFileMemoryStore::new(&path).expect("new store");
        store.put("agent-a", "runbook", "v1");
        store.put("agent-a", "runbook", "v2");
        assert_eq!(store.get("agent-a", "runbook"), Some("v2".to_string()));
    }
}
