#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SystemSnapshot {
    pub total_physical_bytes: u64,
    pub available_physical_bytes: u64,
    pub commit_total_pages: u64,
    pub commit_limit_pages: u64,
    pub hard_faults_per_sec: f64,
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
        if self.commit_limit_pages == 0 {
            return 0.0;
        }

        (self.commit_total_pages as f64 / self.commit_limit_pages as f64).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_ratios_are_bounded() {
        let snapshot = SystemSnapshot {
            total_physical_bytes: 16,
            available_physical_bytes: 4,
            commit_total_pages: 9,
            commit_limit_pages: 10,
            hard_faults_per_sec: 0.0,
        };

        assert!((snapshot.physical_pressure() - 0.75).abs() < f64::EPSILON);
        assert!((snapshot.commit_pressure() - 0.9).abs() < f64::EPSILON);
    }
}
