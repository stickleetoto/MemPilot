use crate::telemetry::SystemSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureLevel {
    Normal,
    Watch,
    Pressure,
    High,
    Critical,
}

pub fn score(snapshot: &SystemSnapshot) -> f64 {
    let physical = snapshot.physical_pressure();
    let commit = snapshot.commit_pressure();
    let hard_faults = (snapshot.hard_faults_per_sec / 1000.0).clamp(0.0, 1.0);

    (physical * 45.0 + commit * 45.0 + hard_faults * 10.0).clamp(0.0, 100.0)
}

pub fn classify(score: f64) -> PressureLevel {
    match score.clamp(0.0, 100.0) {
        value if value < 30.0 => PressureLevel::Normal,
        value if value < 55.0 => PressureLevel::Watch,
        value if value < 75.0 => PressureLevel::Pressure,
        value if value < 90.0 => PressureLevel::High,
        _ => PressureLevel::Critical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_boundaries() {
        assert_eq!(classify(0.0), PressureLevel::Normal);
        assert_eq!(classify(30.0), PressureLevel::Watch);
        assert_eq!(classify(55.0), PressureLevel::Pressure);
        assert_eq!(classify(75.0), PressureLevel::High);
        assert_eq!(classify(90.0), PressureLevel::Critical);
        assert_eq!(classify(100.0), PressureLevel::Critical);
    }

    #[test]
    fn high_commit_and_physical_pressure_score_high() {
        let snapshot = SystemSnapshot {
            total_physical_bytes: 100,
            available_physical_bytes: 10,
            commit_total_pages: 90,
            commit_limit_pages: 100,
            hard_faults_per_sec: 500.0,
        };

        let value = score(&snapshot);
        assert!(value >= 75.0);
    }
}
