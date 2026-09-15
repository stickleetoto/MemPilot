use crate::{
    adaptive::{AdaptiveTracker, ProcessDelta},
    pressure::PressureLevel,
    telemetry::ProcessSnapshot,
};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PolicyAction {
    Observe,
    RecommendMemoryPriorityDemotion,
    RecommendSelectiveTrim,
    EmergencyProtect,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Candidate {
    pub pid: u32,
    pub image_name: String,
    pub score: f64,
    pub expected_reclaim_bytes: u64,
    pub refault_risk: f64,
    pub reason: String,
}

pub fn action_for(level: PressureLevel) -> PolicyAction {
    match level {
        PressureLevel::Normal | PressureLevel::Watch => PolicyAction::Observe,
        PressureLevel::Pressure => PolicyAction::RecommendMemoryPriorityDemotion,
        PressureLevel::High => PolicyAction::RecommendSelectiveTrim,
        PressureLevel::Critical => PolicyAction::EmergencyProtect,
    }
}

pub fn rank_candidates(
    processes: &[ProcessSnapshot],
    deltas: &HashMap<crate::telemetry::ProcessKey, ProcessDelta>,
    adaptive: &AdaptiveTracker,
) -> Vec<Candidate> {
    let mut candidates = processes
        .iter()
        .filter_map(|process| score_candidate(process, deltas.get(&process.key), adaptive))
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| {
                right
                    .expected_reclaim_bytes
                    .cmp(&left.expected_reclaim_bytes)
            })
            .then_with(|| left.pid.cmp(&right.pid))
    });
    candidates
}

fn score_candidate(
    process: &ProcessSnapshot,
    delta: Option<&ProcessDelta>,
    adaptive: &AdaptiveTracker,
) -> Option<Candidate> {
    if process.foreground || process.working_set_bytes < 32 * 1024 * 1024 {
        return None;
    }

    let image_lower = process.image_name.to_ascii_lowercase();
    if is_protected_name(&image_lower) {
        return None;
    }

    let reclaim_base = process.working_set_bytes.min(
        process
            .private_commit_bytes
            .max(process.working_set_bytes / 4),
    );

    let size_score = (reclaim_base as f64 / (1024.0 * 1024.0 * 1024.0)).clamp(0.0, 1.0) * 65.0;

    let current_fault_risk = delta
        .map(|value| (value.page_fault_delta as f64 / 2_000.0).clamp(0.0, 1.0))
        .unwrap_or(0.15);

    let historical_fault_risk = adaptive.fault_penalty_for(&process.image_name);
    let refault_risk = (current_fault_risk * 0.6 + historical_fault_risk * 0.4).clamp(0.0, 1.0);

    let stability_bonus = delta
        .map(|value| {
            if value.working_set_delta_bytes.abs() < 8 * 1024 * 1024 {
                20.0
            } else {
                5.0
            }
        })
        .unwrap_or(5.0);

    let score = (size_score + stability_bonus - refault_risk * 55.0).clamp(0.0, 100.0);
    if score < 10.0 {
        return None;
    }

    Some(Candidate {
        pid: process.key.pid,
        image_name: process.image_name.clone(),
        score,
        expected_reclaim_bytes: reclaim_base,
        refault_risk,
        reason: format!(
            "background working set {:.0} MiB, private commit {:.0} MiB, refault risk {:.0}%",
            process.working_set_bytes as f64 / (1024.0 * 1024.0),
            process.private_commit_bytes as f64 / (1024.0 * 1024.0),
            refault_risk * 100.0,
        ),
    })
}

fn is_protected_name(name: &str) -> bool {
    matches!(
        name,
        "system"
            | "registry"
            | "memory compression"
            | "secure system"
            | "idle"
            | "csrss.exe"
            | "wininit.exe"
            | "winlogon.exe"
            | "services.exe"
            | "lsass.exe"
            | "smss.exe"
            | "dwm.exe"
            | "explorer.exe"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::{ProcessKey, ProcessSnapshot};

    fn process(name: &str, foreground: bool, mib: u64) -> ProcessSnapshot {
        ProcessSnapshot {
            key: ProcessKey {
                pid: 100,
                creation_time_100ns: 1,
            },
            image_name: name.into(),
            image_path: None,
            working_set_bytes: mib * 1024 * 1024,
            peak_working_set_bytes: mib * 1024 * 1024,
            private_commit_bytes: mib * 1024 * 1024,
            page_fault_count: 0,
            foreground,
        }
    }

    #[test]
    fn foreground_is_never_candidate() {
        let tracker = AdaptiveTracker::default();
        let ranked = rank_candidates(
            &[process("game.exe", true, 2048)],
            &HashMap::new(),
            &tracker,
        );
        assert!(ranked.is_empty());
    }

    #[test]
    fn large_background_process_is_candidate() {
        let tracker = AdaptiveTracker::default();
        let ranked = rank_candidates(
            &[process("worker.exe", false, 1024)],
            &HashMap::new(),
            &tracker,
        );
        assert_eq!(ranked.len(), 1);
        assert!(ranked[0].score > 0.0);
    }

    #[test]
    fn protected_windows_process_is_excluded() {
        let tracker = AdaptiveTracker::default();
        let ranked = rank_candidates(
            &[process("lsass.exe", false, 1024)],
            &HashMap::new(),
            &tracker,
        );
        assert!(ranked.is_empty());
    }
}
