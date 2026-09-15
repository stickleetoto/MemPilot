use serde::Serialize;
use std::io;

#[cfg(windows)]
mod windows_backend;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct SystemSnapshot {
    pub sampled_at_unix_ms: u64,
    pub total_physical_bytes: u64,
    pub available_physical_bytes: u64,
    pub commit_total_bytes: u64,
    pub commit_limit_bytes: u64,
    pub system_cache_bytes: u64,
    pub paged_pool_bytes: u64,
    pub nonpaged_pool_bytes: u64,
    pub memory_load_percent: u32,
    pub low_memory_signal: Option<bool>,
}

impl SystemSnapshot {
    pub fn physical_pressure(&self) -> f64 {
        if self.total_physical_bytes == 0 {
            return 0.0;
        }

        let available = self.available_physical_bytes.min(self.total_physical_bytes) as f64;
        let total = self.total_physical_bytes as f64;
        (1.0 - available / total).clamp(0.0, 1.0)
    }

    pub fn commit_pressure(&self) -> f64 {
        if self.commit_limit_bytes == 0 {
            return 0.0;
        }

        (self.commit_total_bytes as f64 / self.commit_limit_bytes as f64).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ProcessKey {
    pub pid: u32,
    pub creation_time_100ns: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProcessSnapshot {
    pub key: ProcessKey,
    pub image_name: String,
    pub image_path: Option<String>,
    pub working_set_bytes: u64,
    pub peak_working_set_bytes: u64,
    pub private_commit_bytes: u64,
    pub page_fault_count: u64,
    pub foreground: bool,
}

pub fn system_snapshot() -> io::Result<SystemSnapshot> {
    #[cfg(windows)]
    {
        return windows_backend::system_snapshot();
    }

    #[cfg(not(windows))]
    {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "live telemetry is supported only on Windows",
        ))
    }
}

pub fn process_snapshots() -> io::Result<Vec<ProcessSnapshot>> {
    #[cfg(windows)]
    {
        return windows_backend::process_snapshots();
    }

    #[cfg(not(windows))]
    {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "process telemetry is supported only on Windows",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SystemSnapshot {
        SystemSnapshot {
            sampled_at_unix_ms: 0,
            total_physical_bytes: 16,
            available_physical_bytes: 4,
            commit_total_bytes: 9,
            commit_limit_bytes: 10,
            system_cache_bytes: 0,
            paged_pool_bytes: 0,
            nonpaged_pool_bytes: 0,
            memory_load_percent: 75,
            low_memory_signal: Some(false),
        }
    }

    #[test]
    fn pressure_ratios_are_bounded() {
        let snapshot = sample();
        assert!((snapshot.physical_pressure() - 0.75).abs() < f64::EPSILON);
        assert!((snapshot.commit_pressure() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn zero_denominators_are_safe() {
        let mut snapshot = sample();
        snapshot.total_physical_bytes = 0;
        snapshot.commit_limit_bytes = 0;
        assert_eq!(snapshot.physical_pressure(), 0.0);
        assert_eq!(snapshot.commit_pressure(), 0.0);
    }
}
