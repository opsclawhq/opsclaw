use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryQuery {
    pub key: String,
}

#[async_trait]
pub trait Memory: Send + Sync {
    async fn put(&self, entry: MemoryEntry) -> Result<(), String>;
    async fn get(&self, query: MemoryQuery) -> Result<Option<MemoryEntry>, String>;
}

#[cfg(test)]
mod tests {
    use super::{MemoryEntry, MemoryQuery};

    #[test]
    fn memory_types_hold_keys_and_values() {
        let entry = MemoryEntry {
            key: "incident-123".to_string(),
            value: "root cause pending".to_string(),
        };
        let query = MemoryQuery {
            key: "incident-123".to_string(),
        };

        assert_eq!(entry.key, query.key);
    }
}
