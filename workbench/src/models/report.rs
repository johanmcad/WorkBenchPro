use serde::{Deserialize, Serialize};

use super::results::BenchmarkRun;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    pub baseline: BenchmarkRun,
    pub comparison: BenchmarkRun,
    pub differences: Vec<MetricDifference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDifference {
    pub test_id: String,
    pub name: String,
    pub baseline_value: f64,
    pub comparison_value: f64,
    pub difference_percent: f64,
    pub is_improvement: bool,
}

impl MetricDifference {
    pub fn new(
        test_id: String,
        name: String,
        baseline_value: f64,
        comparison_value: f64,
        higher_is_better: bool,
    ) -> Self {
        let difference_percent = if baseline_value != 0.0 {
            ((comparison_value - baseline_value) / baseline_value) * 100.0
        } else {
            0.0
        };

        let is_improvement = if higher_is_better {
            comparison_value > baseline_value
        } else {
            comparison_value < baseline_value
        };

        Self {
            test_id,
            name,
            baseline_value,
            comparison_value,
            difference_percent,
            is_improvement,
        }
    }

    pub fn multiplier(&self) -> f64 {
        if self.baseline_value != 0.0 {
            self.comparison_value / self.baseline_value
        } else {
            1.0
        }
    }
}
