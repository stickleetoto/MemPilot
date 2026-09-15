use crate::telemetry::{ProcessKey, ProcessSnapshot};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct ProcessDelta {
    pub page_fault_delta: u64,
    pub working_set_delta_bytes: i64,
    pub private_commit_delta_bytes: i64,
}

#[derive(Debug, Clone)]
struct LastProcessSample {
    page_fault_count: u64,
    working_set_bytes: u64,
    private_commit_bytes: u64,
}

#[derive(Debug, Default)]
pub struct AdaptiveTracker {
    previous: HashMap<ProcessKey, LastProcessSample>,
    fault_penalty: HashMap<String, f64>,
}

impl AdaptiveTracker {
    pub fn observe(&mut self, processes: &[ProcessSnapshot]) -> HashMap<ProcessKey, ProcessDelta> {
        let mut deltas = HashMap::new();
        let mut next = HashMap::with_capacity(processes.len());

        for process in processes {
            if let Some(previous) = self.previous.get(&process.key) {
                let delta = ProcessDelta {
                    page_fault_delta: process
                        .page_fault_count
                        .saturating_sub(previous.page_fault_count),
                    working_set_delta_bytes: signed_delta(
                        process.working_set_bytes,
                        previous.working_set_bytes,
                    ),
                    private_commit_delta_bytes: signed_delta(
                        process.private_commit_bytes,
                        previous.private_commit_bytes,
                    ),
                };

                let penalty = normalized_fault_penalty(delta.page_fault_delta);
                self.fault_penalty
                    .entry(process.image_name.to_ascii_lowercase())
                    .and_modify(|value| *value = *value * 0.8 + penalty * 0.2)
                    .or_insert(penalty);

                deltas.insert(process.key.clone(), delta);
            }

            next.insert(
                process.key.clone(),
                LastProcessSample {
                    page_fault_count: process.page_fault_count,
                    working_set_bytes: process.working_set_bytes,
                    private_commit_bytes: process.private_commit_bytes,
                },
            );
        }

        self.previous = next;
        deltas
    }

    pub fn fault_penalty_for(&self, image_name: &str) -> f64 {
        self.fault_penalty
            .get(&image_name.to_ascii_lowercase())
            .copied()
            .unwrap_or(0.0)
            .clamp(0.0, 1.0)
    }
}

fn normalized_fault_penalty(page_fault_delta: u64) -> f64 {
    (page_fault_delta as f64 / 2_000.0).clamp(0.0, 1.0)
}

fn signed_delta(current: u64, previous: u64) -> i64 {
    let delta = current as i128 - previous as i128;
    delta.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process(faults: u64, working_set: u64) -> ProcessSnapshot {
        ProcessSnapshot {
            key: ProcessKey {
                pid: 42,
                creation_time_100ns: 7,
            },
            image_name: "test.exe".into(),
            image_path: None,
            working_set_bytes: working_set,
            peak_working_set_bytes: working_set,
            private_commit_bytes: working_set / 2,
            page_fault_count: faults,
            foreground: false,
        }
    }

    #[test]
    fn tracker_emits_deltas_after_second_observation() {
        let mut tracker = AdaptiveTracker::default();
        assert!(tracker.observe(&[process(10, 100)]).is_empty());

        let deltas = tracker.observe(&[process(25, 120)]);
        let delta = deltas
            .get(&ProcessKey {
                pid: 42,
                creation_time_100ns: 7,
            })
            .expect("delta should exist");

        assert_eq!(delta.page_fault_delta, 15);
        assert_eq!(delta.working_set_delta_bytes, 20);
    }

    #[test]
    fn fault_penalty_is_bounded() {
        let mut tracker = AdaptiveTracker::default();
        tracker.observe(&[process(0, 100)]);
        tracker.observe(&[process(20_000, 100)]);
        assert_eq!(tracker.fault_penalty_for("test.exe"), 1.0);
    }
}
