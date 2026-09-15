use crate::telemetry::SystemSnapshot;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum PressureLevel {
    Normal,
    Watch,
    Pressure,
    High,
    Critical,
}

impl fmt::Display for PressureLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Normal => "NORMAL",
            Self::Watch => "WATCH",
            Self::Pressure => "PRESSURE",
            Self::High => "HIGH",
            Self::Critical => "CRITICAL",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct PressureAssessment {
    pub raw_score: f64,
    pub smoothed_score: f64,
    pub level: PressureLevel,
    pub physical_pressure: f64,
    pub commit_pressure: f64,
    pub low_memory_signal: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PressureEngine {
    level: PressureLevel,
    smoothed_score: Option<f64>,
    smoothing_alpha: f64,
}

impl Default for PressureEngine {
    fn default() -> Self {
        Self {
            level: PressureLevel::Normal,
            smoothed_score: None,
            smoothing_alpha: 0.35,
        }
    }
}

impl PressureEngine {
    pub fn update(&mut self, snapshot: &SystemSnapshot) -> PressureAssessment {
        let raw_score = raw_score(snapshot);
        let smoothed_score = match self.smoothed_score {
            Some(previous) => {
                previous * (1.0 - self.smoothing_alpha) + raw_score * self.smoothing_alpha
            }
            None => raw_score,
        };

        self.smoothed_score = Some(smoothed_score);
        self.level = classify_with_hysteresis(
            smoothed_score,
            self.level,
            snapshot.low_memory_signal == Some(true),
        );

        PressureAssessment {
            raw_score,
            smoothed_score,
            level: self.level,
            physical_pressure: snapshot.physical_pressure(),
            commit_pressure: snapshot.commit_pressure(),
            low_memory_signal: snapshot.low_memory_signal,
        }
    }

    pub fn level(&self) -> PressureLevel {
        self.level
    }
}

pub fn raw_score(snapshot: &SystemSnapshot) -> f64 {
    let physical = snapshot.physical_pressure();
    let commit = snapshot.commit_pressure();

    let mut score = physical * 55.0 + commit * 45.0;
    if snapshot.low_memory_signal == Some(true) {
        score = score.max(82.0);
    }

    score.clamp(0.0, 100.0)
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

fn classify_with_hysteresis(
    score: f64,
    current: PressureLevel,
    low_memory_signal: bool,
) -> PressureLevel {
    if low_memory_signal {
        return PressureLevel::High.max(classify(score));
    }

    let target = classify(score);
    if target >= current {
        return target;
    }

    match current {
        PressureLevel::Critical if score >= 84.0 => PressureLevel::Critical,
        PressureLevel::High if score >= 69.0 => PressureLevel::High,
        PressureLevel::Pressure if score >= 49.0 => PressureLevel::Pressure,
        PressureLevel::Watch if score >= 24.0 => PressureLevel::Watch,
        _ => target,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(available: u64, commit: u64, low: Option<bool>) -> SystemSnapshot {
        SystemSnapshot {
            sampled_at_unix_ms: 0,
            total_physical_bytes: 100,
            available_physical_bytes: available,
            commit_total_bytes: commit,
            commit_limit_bytes: 100,
            system_cache_bytes: 0,
            paged_pool_bytes: 0,
            nonpaged_pool_bytes: 0,
            memory_load_percent: 0,
            low_memory_signal: low,
        }
    }

    #[test]
    fn classifies_boundaries() {
        assert_eq!(classify(0.0), PressureLevel::Normal);
        assert_eq!(classify(30.0), PressureLevel::Watch);
        assert_eq!(classify(55.0), PressureLevel::Pressure);
        assert_eq!(classify(75.0), PressureLevel::High);
        assert_eq!(classify(90.0), PressureLevel::Critical);
    }

    #[test]
    fn low_memory_signal_forces_high_or_worse() {
        let mut engine = PressureEngine::default();
        let assessment = engine.update(&snapshot(90, 10, Some(true)));
        assert!(assessment.level >= PressureLevel::High);
    }

    #[test]
    fn hysteresis_prevents_immediate_drop() {
        let mut engine = PressureEngine::default();
        engine.update(&snapshot(5, 95, None));
        assert_eq!(engine.level(), PressureLevel::Critical);

        let next = engine.update(&snapshot(25, 75, None));
        assert!(next.level >= PressureLevel::High);
    }
}
