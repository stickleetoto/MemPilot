use crate::{policy::{action_for, PolicyAction}, pressure::PressureLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimizationPlan {
    pub action: PolicyAction,
    pub mutates_system: bool,
    pub reason: &'static str,
}

pub fn plan(level: PressureLevel) -> OptimizationPlan {
    let action = action_for(level);

    match action {
        PolicyAction::Observe => OptimizationPlan {
            action,
            mutates_system: false,
            reason: "pressure is low enough to observe only",
        },
        PolicyAction::DemoteBackgroundMemory => OptimizationPlan {
            action,
            mutates_system: true,
            reason: "pressure justifies lowering background memory priority",
        },
        PolicyAction::SelectiveTrim => OptimizationPlan {
            action,
            mutates_system: true,
            reason: "high pressure justifies selective trimming of cold candidates",
        },
        PolicyAction::EmergencyProtect => OptimizationPlan {
            action,
            mutates_system: true,
            reason: "critical pressure requires protecting active workloads first",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_mode_never_mutates() {
        let plan = plan(PressureLevel::Normal);
        assert!(!plan.mutates_system);
        assert_eq!(plan.action, PolicyAction::Observe);
    }
}
