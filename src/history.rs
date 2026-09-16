use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const HISTORY_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessHistory {
    pub executable: String,
    pub observations: u64,
    pub interventions: u64,
    pub useful_reclaim_bytes: u64,
    pub refault_penalty: f64,
    pub last_seen_unix_ms: u64,
}

impl ProcessHistory {
    pub fn reclaim_effectiveness(&self) -> f64 {
        if self.interventions == 0 {
            return 0.0;
        }
        self.useful_reclaim_bytes as f64 / self.interventions as f64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryStore {
    pub schema_version: u32,
    pub processes: BTreeMap<String, ProcessHistory>,
}

impl Default for HistoryStore {
    fn default() -> Self {
        Self {
            schema_version: HISTORY_SCHEMA_VERSION,
            processes: BTreeMap::new(),
        }
    }
}

impl HistoryStore {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }

        let bytes = fs::read(path)?;
        let store: Self = serde_json::from_slice(&bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if store.schema_version != HISTORY_SCHEMA_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "unsupported MemPilot history schema {} (expected {})",
                    store.schema_version, HISTORY_SCHEMA_VERSION
                ),
            ));
        }

        Ok(store)
    }

    pub fn save_atomic(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let temp = temporary_path(path);
        let data = serde_json::to_vec_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&temp, data)?;

        if path.exists() {
            fs::remove_file(path)?;
        }
        fs::rename(temp, path)
    }

    pub fn record_observation(&mut self, executable: &str, now_unix_ms: u64) {
        let key = normalize_executable(executable);
        let entry = self.processes.entry(key).or_insert_with(|| ProcessHistory {
            executable: executable.to_owned(),
            observations: 0,
            interventions: 0,
            useful_reclaim_bytes: 0,
            refault_penalty: 0.0,
            last_seen_unix_ms: now_unix_ms,
        });
        entry.observations = entry.observations.saturating_add(1);
        entry.last_seen_unix_ms = now_unix_ms;
    }

    pub fn record_intervention(
        &mut self,
        executable: &str,
        useful_reclaim_bytes: u64,
        refault_penalty: f64,
        now_unix_ms: u64,
    ) {
        self.record_observation(executable, now_unix_ms);
        let key = normalize_executable(executable);
        if let Some(entry) = self.processes.get_mut(&key) {
            entry.interventions = entry.interventions.saturating_add(1);
            entry.useful_reclaim_bytes = entry
                .useful_reclaim_bytes
                .saturating_add(useful_reclaim_bytes);
            entry.refault_penalty = (entry.refault_penalty + refault_penalty.max(0.0)).min(1_000_000.0);
        }
    }

    pub fn reset(&mut self) {
        self.processes.clear();
    }
}

fn normalize_executable(executable: &str) -> String {
    executable.trim().to_ascii_lowercase()
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut temp = path.as_os_str().to_owned();
    temp.push(".tmp");
    PathBuf::from(temp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_store_is_versioned_and_empty() {
        let store = HistoryStore::default();
        assert_eq!(store.schema_version, HISTORY_SCHEMA_VERSION);
        assert!(store.processes.is_empty());
    }

    #[test]
    fn executable_identity_is_case_insensitive() {
        let mut store = HistoryStore::default();
        store.record_observation("Chrome.EXE", 10);
        store.record_observation("chrome.exe", 20);
        assert_eq!(store.processes.len(), 1);
        let item = store.processes.values().next().unwrap();
        assert_eq!(item.observations, 2);
        assert_eq!(item.last_seen_unix_ms, 20);
    }

    #[test]
    fn intervention_tracks_benefit_and_penalty() {
        let mut store = HistoryStore::default();
        store.record_intervention("build.exe", 1024, 0.25, 42);
        let item = store.processes.values().next().unwrap();
        assert_eq!(item.interventions, 1);
        assert_eq!(item.useful_reclaim_bytes, 1024);
        assert!((item.refault_penalty - 0.25).abs() < f64::EPSILON);
        assert_eq!(item.reclaim_effectiveness(), 1024.0);
    }
}