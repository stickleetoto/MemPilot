use crate::pressure::PressureLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyAction {
    Observe,
    DemoteBackgroundMemory,
    SelectiveTrim,
    EmergencyProtect,
}

pub fn action_for(level: PressureLevel) -> PolicyAction {
    match level {
        PressureLevel::Normal | PressureLevel::Watch => PolicyAction::Observe,
        PressureLevel::Pressure => PolicyAction::DemoteBackgroundMemory,
        PressureLevel::High => PolicyAction::SelectiveTrim,
        PressureLevel::Critical => PolicyAction::EmergencyProtect,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_escalates_with_pressure() {
        assert_eq!(action_for(PressureLevel::Normal), PolicyAction::Observe);
        assert_eq!(
            action_for(PressureLevel::Pressure),
            PolicyAction::DemoteBackgroundMemory
        );
        assert_eq!(action_for(PressureLevel::High), PolicyAction::SelectiveTrim);
        assert_eq!(
            action_for(PressureLevel::Critical),
            PolicyAction::EmergencyProtect
        );
    }
}
