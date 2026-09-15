use crate::{
    policy::{action_for, Candidate, PolicyAction},
    pressure::PressureAssessment,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OptimizationPlan {
    pub pressure: PressureAssessment,
    pub action: PolicyAction,
    pub mutates_system: bool,
    pub reason: &'static str,
    pub candidates: Vec<Candidate>,
}

pub fn dry_run(
    pressure: PressureAssessment,
    candidates: Vec<Candidate>,
    candidate_limit: usize,
) -> OptimizationPlan {
    let action = action_for(pressure.level);
    let reason = match action {
        PolicyAction::Observe => "pressure is low enough to observe only",
        PolicyAction::RecommendMemoryPriorityDemotion => {
            "pressure justifies considering lower memory priority for cold background processes"
        }
        PolicyAction::RecommendSelectiveTrim => {
            "high pressure justifies evaluating selective trim candidates"
        }
        PolicyAction::EmergencyProtect => {
            "critical pressure requires protecting active workloads and avoiding indiscriminate actions"
        }
    };

    OptimizationPlan {
        pressure,
        action,
        mutates_system: false,
        reason,
        candidates: candidates.into_iter().take(candidate_limit).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pressure::PressureLevel;

    #[test]
    fn v0_2_plans_never_mutate() {
        let pressure = PressureAssessment {
            raw_score: 99.0,
            smoothed_score: 99.0,
            level: PressureLevel::Critical,
            physical_pressure: 0.99,
            commit_pressure: 0.99,
            low_memory_signal: Some(true),
        };

        let plan = dry_run(pressure, Vec::new(), 10);
        assert!(!plan.mutates_system);
    }
}
