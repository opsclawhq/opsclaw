use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeEvent {
    pub schema_version: u16,
    pub event_type: String,
    pub run_id: String,
    pub payload_json: String,
}

pub struct JsonlEventJournal {
    path: PathBuf,
}

impl JsonlEventJournal {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn append(&self, event: &RuntimeEvent) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let line = serde_json::to_string(event).map_err(std::io::Error::other)?;
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn read_all(&self) -> std::io::Result<Vec<RuntimeEvent>> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(err),
        };

        let mut out = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let event: RuntimeEvent =
                serde_json::from_str(&line).map_err(std::io::Error::other)?;
            out.push(event);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonlEventJournal, RuntimeEvent};
    use tempfile::NamedTempFile;

    #[test]
    fn appends_and_reads_events_in_order() {
        let file = NamedTempFile::new().expect("temp file");
        let journal = JsonlEventJournal::new(file.path());

        let e1 = RuntimeEvent {
            schema_version: 1,
            event_type: "queued".to_string(),
            run_id: "run-1".to_string(),
            payload_json: "{\"a\":1}".to_string(),
        };
        let e2 = RuntimeEvent {
            schema_version: 1,
            event_type: "completed".to_string(),
            run_id: "run-1".to_string(),
            payload_json: "{\"ok\":true}".to_string(),
        };

        journal.append(&e1).expect("append e1");
        journal.append(&e2).expect("append e2");

        let replay = journal.read_all().expect("read_all");
        assert_eq!(replay, vec![e1, e2]);
    }

    #[test]
    fn returns_empty_when_journal_file_missing() {
        let path = std::env::temp_dir().join("opsclaw-nonexistent-event-journal.jsonl");
        let _ = std::fs::remove_file(&path);
        let journal = JsonlEventJournal::new(&path);
        let replay = journal.read_all().expect("missing journal should return empty");
        assert!(replay.is_empty());
    }
}
